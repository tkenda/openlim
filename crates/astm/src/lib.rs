#![forbid(unsafe_code)]

use async_trait::async_trait;
use log::error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

mod error;
mod message;
mod values;
mod socket;
#[cfg(test)]
mod tests;

pub use error::ASTMError;
pub type Result<T> = std::result::Result<T, ASTMError>;

pub use message::{Frame, Message};
pub use socket::server::SocketServer;

#[macro_export]
macro_rules! ctrl {
    ($x: ident) => {
        CtrlChar::$x
    };
}

macro_rules! some_ctrl {
    ($x: ident) => {
        Some(vec![CtrlChar::$x])
    };
}

pub struct CtrlChar {}

impl CtrlChar {
    const STX: u8 = 0x02;
    const ETX: u8 = 0x03;
    const EOT: u8 = 0x04;
    const ENQ: u8 = 0x05;
    const ACK: u8 = 0x06;
    const NAK: u8 = 0x15;
    const ETB: u8 = 0x17;
    const CR: u8 = 0x0D;
    const LF: u8 = 0x0A;
}

#[derive(Clone)]
pub enum CharEncoding {
    ASCII,
    Windows1251,
    UTF8,
}

#[derive(Clone, PartialEq)]
enum State {
    Idle,
    Receiving,
    Sending,
}

impl Default for State {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Clone, Default)]
struct DataLink {
    state: Arc<Mutex<State>>,
    in_message: Arc<Mutex<Message>>,
    out_message: Arc<Mutex<Message>>,
    timeout: Arc<Mutex<Option<u64>>>,
}

impl DataLink {
    async fn get_state(&self) -> State {
        let state = self.state.lock().await;
        (*state).to_owned()
    }

    async fn set_state(&self, src: State) {
        let mut state = self.state.lock().await;
        *state = src;
    }

    async fn get_in_message(&self) -> Message {
        let in_message = self.in_message.lock().await;
        (*in_message).to_owned()
    }

    async fn push_in_frame(&self, src: Frame) {
        let mut in_message = self.in_message.lock().await;
        (*in_message).push_frame(src);
    }

    async fn drop_in_message(&self) {
        let mut in_message = self.in_message.lock().await;
        *in_message = Message::default();
    }

    async fn set_out_message(&self, src: Message) {
        let mut out_message = self.out_message.lock().await;
        *out_message = src;
    }

    async fn is_out_message_empty(&self) -> bool {
        let out_message = self.out_message.lock().await;
        (*out_message).is_empty()
    }

    async fn pop_out_frame(&self) -> Option<Frame> {
        let mut out_message = self.out_message.lock().await;
        (*out_message).pop_frame()
    }

    async fn set_timeout(&self, src: u64) {
        let mut timeout = self.timeout.lock().await;
        *timeout = Some(src);
    }

    async fn reset_timeout(&self) {
        let mut timeout = self.timeout.lock().await;
        *timeout = None;
    }

    async fn is_timeout(&self) -> bool {
        let mut timeout = self.timeout.lock().await;

        match *timeout {
            Some(t) if t == 0 => true,
            Some(mut t) => {
                t -= 1;
                *timeout = Some(t);
                false
            }
            None => false,
        }
    }
}

impl DataLink {
    async fn read<S: Clone + Sync + Action<S>>(
        &self,
        src: &[u8],
        astm: ASTM<S>,
    ) -> Option<Vec<u8>> {
        match self.get_state().await {
            State::Idle => {
                if src[0] == ctrl!(ENQ) {
                    self.set_timeout(astm.timeout).await;
                    self.set_state(State::Receiving).await;
                    some_ctrl!(ACK)
                } else {
                    some_ctrl!(NAK)
                }
            }
            State::Receiving => match Frame::deserialize(src, astm.encoding) {
                Ok(frame) => {
                    self.set_timeout(astm.timeout).await;
                    let in_message = self.get_in_message().await;

                    match astm.instrument.on_recv_frame(frame, &in_message).await {
                        Ok(t) => {
                            self.push_in_frame(t).await;
                            some_ctrl!(ACK)
                        }
                        Err(err) => {
                            error!("{}", err);
                            some_ctrl!(NAK)
                        }
                    }
                }
                Err(err) => match src[0] {
                    ctrl!(EOT) => {
                        self.reset_timeout().await;
                        self.set_state(State::Idle).await;

                        let in_message = self.get_in_message().await;
                        if let Some(t) = astm.instrument.on_recv_message(&in_message).await {
                            self.set_out_message(t).await;
                        }

                        self.drop_in_message().await;

                        None
                    }
                    _ => {
                        self.set_timeout(astm.timeout).await;
                        error!("{}", err);
                        some_ctrl!(NAK)
                    }
                },
            },
            State::Sending => {
                if src[0] == ctrl!(ACK) {
                    match self.pop_out_frame().await {
                        Some(t) => match t.serialize(astm.encoding) {
                            Ok(t) => Some(t),
                            Err(err) => {
                                self.reset_timeout().await;
                                self.set_state(State::Idle).await;

                                error!("{}", err);
                                some_ctrl!(NAK)
                            }
                        },
                        None => some_ctrl!(EOT),
                    }
                } else {
                    None
                }
            }
        }
    }

    async fn control<S: Clone + Sync + Action<S>>(&self, astm: ASTM<S>) -> Option<Vec<u8>> {
        sleep(Duration::from_secs(1)).await;

        if self.is_timeout().await {
            self.drop_in_message().await;
            self.set_state(State::Idle).await;
            some_ctrl!(NAK)
        } else if self.get_state().await == State::Idle && !self.is_out_message_empty().await {
            self.set_timeout(astm.timeout).await;
            self.set_state(State::Sending).await;
            some_ctrl!(ENQ)
        } else {
            None
        }
    }

    async fn interval<S: Clone + Sync + Action<S>>(&self, astm: ASTM<S>) {
        loop {
            sleep(Duration::from_millis(astm.interval.unwrap())).await;

            if let Some(t) = astm.instrument.on_idle_interval().await {
                self.set_out_message(t).await;
            }
        }
    }
}

#[async_trait]
pub trait Action<I> {
    async fn on_recv_frame(&self, frame: Frame, message: &Message) -> Result<Frame> {
        Ok(frame)
    }

    async fn on_recv_message(&self, message: &Message) -> Option<Message>;

    async fn on_idle_interval(&self) -> Option<Message> {
        None
    }
}

#[async_trait]
pub trait PhysicalLayer<S>
where
    S: Clone,
{
    async fn run(&self, astm: ASTM<S>) -> Result<()>;
}

#[derive(Clone)]
pub struct ASTM<I>
where
    I: Clone,
{
    instrument: I,
    timeout: u64,
    interval: Option<u64>,
    encoding: CharEncoding,
}

impl<I: Clone> ASTM<I> {
    pub fn new(instrument: I) -> Self {
        Self {
            instrument,
            timeout: 20,
            interval: None,
            encoding: CharEncoding::ASCII,
        }
    }

    pub fn interval(mut self, src: u64) -> Self {
        self.interval = Some(src);
        self
    }

    pub fn encoding(mut self, src: CharEncoding) -> Self {
        self.encoding = src;
        self
    }

    pub async fn run<P: PhysicalLayer<I>>(self, physical_layer: P) -> Result<()> {
        physical_layer.run(self).await
    }
}
