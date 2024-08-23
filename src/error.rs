use std::string::FromUtf8Error;

pub type Result<T> = std::result::Result<T, PvkrError>;

#[derive(Debug)]
pub enum PvkrError {
    FileNotFound(String),
    NotAPvkrPackage(String),
    InvalidPvkrPackage(String),
    ValidationError(String),
    IOError(std::io::Error),
}

impl PvkrError {
    pub fn validation_error<T, S: AsRef<str>>(msg: S) -> Result<T> {
        Err(PvkrError::ValidationError(msg.as_ref().to_string()))
    }

    pub fn file_not_found<T, S: AsRef<str>>(msg: S) -> Result<T> {
        Err(PvkrError::FileNotFound(msg.as_ref().to_string()))
    }

    pub fn not_a_pvkr_package<T, S: AsRef<str>>(msg: S) -> Result<T> {
        Err(PvkrError::NotAPvkrPackage(msg.as_ref().to_string()))
    }

    pub fn invalid_pvkr_package<T, S: AsRef<str>>(msg: S) -> Result<T> {
        Err(PvkrError::InvalidPvkrPackage(msg.as_ref().to_string()))
    }
}

impl From<std::io::Error> for PvkrError {
    fn from(e: std::io::Error) -> Self {
        PvkrError::IOError(e)
    }
}

impl From<FromUtf8Error> for PvkrError {
    fn from(_: FromUtf8Error) -> Self {
        PvkrError::InvalidPvkrPackage("Invalid utf8 string in package.pvkr".to_string())
    }
}

impl std::fmt::Display for PvkrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PvkrError::FileNotFound(msg) => {
                write!(f, "File Not Found: {}", msg)
            }
            PvkrError::NotAPvkrPackage(msg) => {
                write!(f, "Not A Pvkr Package: {}", msg)
            }
            PvkrError::InvalidPvkrPackage(msg) => {
                write!(f, "Invalid Pvkr Package: {}", msg)
            }
            PvkrError::ValidationError(msg) => {
                write!(f, "Validation Error: {}", msg)
            }
            PvkrError::IOError(e) => write!(f, "IOError: {}", e),
        }
    }
}

impl std::error::Error for PvkrError {}
