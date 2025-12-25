use thiserror::Error;

#[derive(Error, Debug)]
pub enum SceneError {
    #[error(transparent)]
    InvalidScene(#[from] toml::de::Error),

    #[error(transparent)]
    InvalidFile(#[from] std::io::Error),
}
