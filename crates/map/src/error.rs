use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("{} is not a directory,", .0)]
    NotADir(String),
    #[error("{} is not a file", .0)]
    NotAFile(String),
    #[error("Not a string.")]
    NotAString,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Std(Box<dyn std::error::Error>),
    #[error("Error building {}: {}", .0, .1)]
    BuilderErrorGeneric(String, String),
    #[error("Error building {}: `{}` not set", .0, .1)]
    BuilderError(String, String),
}
