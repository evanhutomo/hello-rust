use std::fmt;
use std::fs::File;
use std::io::{self, Read};

// Define a custom error type
#[derive(Debug)]
pub enum CustomError {
    Io(io::Error),
    ParseInt(std::num::ParseIntError),
    Utf8(std::string::FromUtf8Error),
    Other(String),
}

// Implement the Display trait for user-friendly error messages
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::Io(err) => write!(f, "IO error: {}", err),
            CustomError::ParseInt(err) => write!(f, "ParseInt error: {}", err),
            CustomError::Utf8(err) => write!(f, "UTF-8 error: {}", err),
            CustomError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

// Implement the Error trait for compatibility with Rust's error-handling ecosystem
impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CustomError::Io(err) => Some(err),
            CustomError::ParseInt(err) => Some(err),
            CustomError::Utf8(err) => Some(err),
            CustomError::Other(_) => None,
        }
    }
}

// Function that reads a file and returns a Result with the file content or a CustomError
pub fn read_file(file_path: &str) -> Result<String, CustomError> {
    let mut file = File::open(file_path).map_err(CustomError::Io)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content).map_err(CustomError::Io)?;
    String::from_utf8(content).map_err(CustomError::Utf8)
}
