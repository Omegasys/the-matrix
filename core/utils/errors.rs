use std::fmt;

#[derive(Debug)]
pub enum MatrixError {
    Network(String),
    IO(String),
    Security(String),
    Rendering(String),
    InvalidData(String),
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatrixError::Network(e) => write!(f, "Network error: {}", e),
            MatrixError::IO(e) => write!(f, "IO error: {}", e),
            MatrixError::Security(e) => write!(f, "Security error: {}", e),
            MatrixError::Rendering(e) => write!(f, "Rendering error: {}", e),
            MatrixError::InvalidData(e) => write!(f, "Invalid data: {}", e),
        }
    }
}

impl std::error::Error for MatrixError {}
