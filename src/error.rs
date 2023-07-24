use std::fmt::{Debug, Display};

pub enum Error {
    UnknownChar(String, char, usize, usize),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl Debug for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnknownChar(target,c, line, column) => {
                write!(_f, "unknown char{{target={target} char='{c}', line={line}, column={column}}}")
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnknownChar(target,c, line, column) => {
                write!(_f, "Unknown char '{}' parsing {target} at line {}, col {}", c, line, column)
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        todo!()
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_value: std::num::ParseIntError) -> Self {
        todo!()
    }
}
