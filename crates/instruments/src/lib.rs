use log::info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;

mod error;

const INST_PATH: &str = "drivers";
const YAML_FILE: &str = "driver.yaml";

pub use error::InstError;
pub type Result<T> = std::result::Result<T, InstError>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Protocol {
    ASTM,
    HL7,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Mode {
    Server,
    Client,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Driver {
    name: String,
    version: String,
    protocol: Protocol,
    modes: Vec<Mode>,
}

impl Driver {
    async fn read(path: PathBuf, folder_name: &str) -> Result<Self> {
        match fs::File::open(path).await {
            Ok(mut file) => {
                let mut contents = vec![];
                file.read_to_end(&mut contents)
                    .await
                    .map_err(|t| InstError::ReadDriverYaml(t.to_string()))?;
                serde_yaml::from_slice(&contents).map_err(|t| {
                    InstError::ParseDriverYaml(t.to_string(), folder_name.to_string())
                })
            }
            Err(err) => Err(InstError::OpenDriverYaml(err.to_string())),
        }
    }
}

#[derive(Default)]
pub struct Instruments {
    drivers: Arc<Mutex<Vec<Driver>>>,
}

impl Instruments {
    pub async fn new() -> Result<Self> {
        let mut dst = Self::default();
        dst.scan().await?;
        Ok(dst)
    }

    pub async fn scan(&mut self) -> Result<()> {
        // create drivers dir if missing
        match fs::create_dir(INST_PATH).await {
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(err) => return Err(InstError::CreateDriversFolder(err.to_string())),
            _ => {}
        }

        // scan drivers folder
        let mut entries = match fs::read_dir(INST_PATH).await {
            Ok(t) => t,
            Err(err) => return Err(InstError::ReadDriversFolder(err.to_string())),
        };

        info!("Scanning drivers folder..");

        let mut new = vec![];

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|t| InstError::ReadDriversFolderEntry(t.to_string()))?
        {
            let folder_name = entry.file_name().to_string_lossy().to_string();
            let driver_path = entry.path();
            let yaml_path = driver_path.join(YAML_FILE);

            if !yaml_path.exists() {
                return Err(InstError::MissingDriverYaml(folder_name));
            }

            let driver = Driver::read(yaml_path, &folder_name).await?;
            info!(
                "Found driver {} ({}) in folder {}.",
                driver.name, driver.version, folder_name
            );

            // check duplicated

            new.push(driver);
        }

        let mut drivers = self.drivers.lock().await;
        *drivers = new;

        Ok(())
    }
}
