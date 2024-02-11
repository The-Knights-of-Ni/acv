use std::fmt::{Debug, Display, Formatter};

pub enum Error {
    Image(image::error::ImageError),
    Io(std::io::Error),
    Other(String),
}

impl From<image::error::ImageError> for Error {
    fn from(e: image::error::ImageError) -> Self {
        Error::Image(e)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Other(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Image(e) => write!(f, "Image error: {:?}", e),
            Error::Io(e) => write!(f, "IO error: {:?}", e),
            Error::Other(s) => write!(f, "{}", s),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Image(e) => write!(f, "Image error: {}", e),
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Image(e) => Some(e),
            Error::Io(e) => Some(e),
            Error::Other(_) => None,
        }
    }
}
