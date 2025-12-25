use thiserror::Error;

#[derive(Error, Debug)]
pub enum SceneError {
    #[error(transparent)]
    InvalidScene(#[from] toml::de::Error),
}
