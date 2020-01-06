use std::fmt;
use std::result;

#[derive(Debug)]
pub enum Error {
    Scanner(u64, char),
    Parser(u64, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Scanner(ref line, ref msg) =>
                write!(f, "{}: {}", line, msg),
            Error::Parser(ref line, ref msg) =>
                write!(f, "{}: {}", line, msg),
        }
    }
}
// TODO: перенести в lox.rs
pub type Result<T> = result::Result<T, Error>;
