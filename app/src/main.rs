use log::{info, error, LevelFilter};

use astm::{ASTM, ASTMError};
use instruments::{Instruments, InstError};

mod error;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

/*
use async_trait::async_trait;
use astm::*;

#[derive(Clone)]
struct Instrument {
    test: u32,
}

#[async_trait]
impl<S> Action<S> for Instrument {
    async fn on_recv_message(&self, message: &Message) -> Option<Message> {
        println!("{:?}", message);
        None
    }
}

    let instrument = Instrument {
        test: 0
    };

    let socket_server = SocketServer::new("0.0.0.0", 7000);

    if let Err(err) = ASTM::new(instrument)
        .interval(1000)
        .run(socket_server)
        .await
    {
        print!("{}", err);
    }
*/

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .parse_default_env()
        .init();

    info!("Open Laboratory Instruments Middleware.");

    if let Err(err) = wrapper().await {
        error!("{}", err);
        return;
    }
}

async fn wrapper() -> Result<()> {
    let inst = Instruments::new().await?;

    Ok(())
}
