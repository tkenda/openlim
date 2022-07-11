use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum InstError {
    #[error("Could not create drivers folder. {0}")]
    CreateDriversFolder(String),
    #[error("Could not read drivers folder. {0}")]
    ReadDriversFolder(String),
    #[error("Could not read drivers folder entry. {0}")]
    ReadDriversFolderEntry(String),
    #[error("Missing driver.yaml file in {0} folder.")]
    MissingDriverYaml(String),
    #[error("Could not read driver.yaml file. {0}")]
    ReadDriverYaml(String),
    #[error("Could not parse driver.yaml file from driver {1}. {0}")]
    ParseDriverYaml(String, String),
    #[error("Could not open driver.yaml file. {0}")]
    OpenDriverYaml(String),
}

impl From<InstError> for String {
    fn from(error: InstError) -> Self {
        error.to_string()
    }
}