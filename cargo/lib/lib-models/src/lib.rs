pub mod domain;

#[derive(thiserror::Error, Debug)]
pub enum SmError {
    #[error("EnvExeption: {0}")]
    EnvExeption(String),

    #[error("ParserExeption: {0}")]
    ParserExeption(String),

    #[error("TimeStampExeption: {0}")]
    TimeStampException(String),

    #[error("Setup Exeption: {0}")]
    SetupExeption(String),

    #[error("SqlExeption: {0}")]
    SqlExeption(String),

    #[error("Uknown: unknown error happend")]
    Unknow,
}
