use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum VenumError {
    Generic { msg: String },
    Conversion(ParseError),
}

impl Display for VenumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VenumError::Generic { msg } => write!(f, "An error occurred: {:?}", msg),
            VenumError::Conversion(ce) => ce.fmt(f),
        }
    }
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum ParseError {
    #[error("Can't unwrap Value::{src_value:?} to basic type {basic_type:?}")]
    UnwrapToBaseTypeFailed {
        src_value: String,
        basic_type: &'static str,
    },
    #[error("Can't construct Value::{target_type:?} from string '{src_value:?}'. Optional info: {opt_info:?}")]
    ValueFromStringFailed {
        src_value: String,
        target_type: &'static str,
        opt_info: Option<String>,
    },
}

pub type Result<T> = std::result::Result<T, VenumError>;
