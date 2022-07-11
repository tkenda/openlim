use async_trait::async_trait;
use log::{debug, error, info};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

use crate::{ASTMError, Result};
use crate::{Action, DataLink, PhysicalLayer, ASTM};

pub struct SocketServer {
    address: SocketAddr,
}

impl SocketServer {
    pub fn new(ip: &str, port: u16) -> Self {
        Self {
            address: SocketAddr::new(ip.parse().unwrap(), port),
        }
    }
}

async fn process_stream<S: Send + Sync + Clone + Action<S> + 'static>(
    stream: TcpStream,
    addr: &SocketAddr,
    astm: ASTM<S>,
) {
    let data_link = DataLink::default();

    let (tx, mut rx) = mpsc::unbounded_channel();
    let (read, write) = stream.into_split();

    let addr = addr.clone();

    let tx_ref = tx.clone();
    let astm_ref = astm.clone();
    let data_link_ref = data_link.clone();

    tokio::spawn(async move {
        loop {
            // wait for the socket to be readable
            if let Err(err) = read.readable().await {
                debug!("Client connection error. {}", err);
                return;
            };

            // creating the buffer **after** the `await` prevents
            // it from being stored in the async task.
            let mut buffer = [0_u8; 4096];

            match read.try_read(&mut buffer) {
                Ok(size) if size == 0 => {
                    debug!("Client connection closed. [{:?}]", addr);
                    return;
                }
                Ok(size) => match data_link_ref.read(&buffer[0..size], astm_ref.clone()).await {
                    Some(chunk) => {
                        tx_ref.send(chunk).unwrap();
                    }
                    None => {}
                },
                Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(err) => {
                    error!("Failed to read from socket ({}); err = {:?}", addr, err);
                    return;
                }
            }
        }
    });

    let tx_ref = tx.clone();
    let astm_ref = astm.clone();
    let data_link_ref = data_link.clone();

    tokio::spawn(async move {
        loop {
            if let Some(t) = data_link_ref.control(astm_ref.clone()).await {
                tx_ref.send(t).unwrap()
            }
        }
    });

    if astm.interval.is_some() {
        tokio::spawn(async move {
            data_link.interval(astm.clone()).await;
        });
    }

    while let Some(chunk) = rx.recv().await {
        // wait for the socket to be writable
        if let Err(err) = write.writable().await {
            debug!("Client connection error. {}", err);
            return;
        };

        // try to write data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match write.try_write(&chunk) {
            Ok(_) => continue,
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(err) => {
                error!("Failed to write to socket ({}); err = {:?}", addr, err);
                return;
            }
        }
    }
}

#[async_trait]
impl<I: Send + Sync + Clone + 'static + Action<I>> PhysicalLayer<I> for SocketServer {
    async fn run(&self, astm: ASTM<I>) -> Result<()> {
        info!("Starting ASTM TCP/IP socket server..");

        let listener = TcpListener::bind(self.address)
            .await
            .map_err(|t| ASTMError::TcpBind(t.to_string()))?;

        loop {
            let (stream, addr) = listener
                .accept()
                .await
                .map_err(|t| ASTMError::TcpAccept(t.to_string()))?;
            debug!("Client {:?} connected.", &addr);

            let astm = astm.clone();

            tokio::spawn(async move {
                process_stream(stream, &addr, astm).await;
            });
        }
    }
}
