use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Ibibazo ntibyabonetse: {0}")]
    QuestionNotFound(u32),

    #[error("Imiterere ya dosiye ntiyemewe: {0}")]
    InvalidData(String),

    #[error("Ikosa ryo kubika cyangwa gusoma amakuru: {0}")]
    StorageError(String),

    #[error("Ikizamini ntikibonetse")]
    AttemptNotFound,

    #[error("Igisubizo nticyemewe ku kizamini gikomeye: Hitamo igisubizo mbere yo kwemeza")]
    NoSelectionMade,

    #[error("Igihe cy'ikizamini cyarangiye")]
    AttemptExpired,

    #[error("Ikosa rya Serde JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Ikosa rya IO: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
