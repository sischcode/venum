use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum VenumError {
    Generic { msg: String },
    Parsing(ParseError),
    Conversion(ConversionError),
}

impl Display for VenumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VenumError::Generic { msg } => write!(f, "An error occurred: {:?}", msg),
            VenumError::Parsing(ce) => ce.fmt(f),
            VenumError::Conversion(pe) => pe.fmt(f),
        }
    }
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum ParseError {
    #[error("Can't construct Value::{target_type:?} from string '{src_value:?}'. Optional info: {opt_info:?}")]
    ValueFromStringFailed {
        src_value: String,
        target_type: &'static str,
        opt_info: Option<String>,
    },
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum ConversionError {
    #[error("Can't convert {src_type:?} with value {src_value:?} to basic type {target_type:?}. Optional info: {opt_info:?}")]
    WrongType {
        src_value: String,
        src_type: String,
        target_type: String,
        opt_info: Option<String>,
    },
}

pub type Result<T> = std::result::Result<T, VenumError>;
