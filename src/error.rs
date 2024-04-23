use std::{error::Error, fmt::Display, io};

#[derive(Debug)]
pub enum DrawError {
    IoError(io::Error),
    OutOfBounds(String),
}

// Define how to print out the error when it occurs based on the type of error it is
impl Display for DrawError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DrawError::IoError(e) => write!(f, "IO Error Occured: {e}"),
            DrawError::OutOfBounds(msg) => write!(f, "Out of Bounds: {msg}"),
        }
    }
}

// Mark the fact MainError is an Error
impl Error for DrawError {}

// Allow converting io:Error to MainError,
// which allows the use of the '?' operator to automatically convert this
impl From<io::Error> for DrawError {
    fn from(value: io::Error) -> Self {
        DrawError::IoError(value)
    }
}



