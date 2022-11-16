use crate::prelude::*;

pub type AtrlResult<T> = std::result::Result<T, AtrlError>;

#[derive(Error, Debug)]
pub enum AtrlError {
    #[error("{} is not a directory,", .0)]
    NotADir(String),
    #[error("{} is not a file", .0)]
    NotAFile(String),
    #[error("Not a string.")]
    NotAString,

    #[error("Invalid world_position {{ {}, {}, {} }}", .0, .1, .2)]
    InvalidWorldPosition(i32, i32, i32),

    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Std(Box<dyn std::error::Error>),
}
