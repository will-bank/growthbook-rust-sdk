use std::env::VarError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum GrowthbookErrorCode {
    GenericError,
    SerdeDeserialize,
    ParseError,
    MissingEnvironmentVariable,
    GrowthbookGateway,
    GrowthbookGatewayDeserialize,
}

#[derive(Debug)]
pub struct GrowthbookError {
    pub code: GrowthbookErrorCode,
    pub message: String,
}

impl GrowthbookError {
    pub fn new(code: GrowthbookErrorCode, message: &str) -> Self {
        GrowthbookError {
            code,
            message: String::from(message),
        }
    }
}

impl Display for GrowthbookError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for GrowthbookError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<Box<dyn Error>> for GrowthbookError {
    fn from(error: Box<dyn Error>) -> Self {
        Self {
            code: GrowthbookErrorCode::GenericError,
            message: error.to_string(),
        }
    }
}

impl From<reqwest_middleware::Error> for GrowthbookError {
    fn from(error: reqwest_middleware::Error) -> Self {
        Self {
            code: GrowthbookErrorCode::GrowthbookGateway,
            message: error.to_string(),
        }
    }
}

impl From<reqwest::Error> for GrowthbookError {
    fn from(error: reqwest::Error) -> Self {
        Self {
            code: GrowthbookErrorCode::GrowthbookGatewayDeserialize,
            message: error.to_string(),
        }
    }
}

impl From<VarError> for GrowthbookError {
    fn from(error: VarError) -> Self {
        Self {
            code: GrowthbookErrorCode::MissingEnvironmentVariable,
            message: error.to_string(),
        }
    }
}

impl From<ParseIntError> for GrowthbookError {
    fn from(error: ParseIntError) -> Self {
        Self {
            code: GrowthbookErrorCode::ParseError,
            message: error.to_string(),
        }
    }
}
