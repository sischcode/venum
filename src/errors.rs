use strum_macros::Display;
use thiserror::Error;

#[derive(Debug, PartialEq, Clone, Display)]
pub enum VenumError {
    Generic { msg: String },
    Parsing(ParseError),
    Conversion(ConversionError),
}

// The Error derive also implements the Display trait!
#[derive(Error, Debug, PartialEq, Clone)]
pub enum ParseError {
    #[error(
        "Can't parse string '{src_value:?}' to construct a {target_type:?}. Optional info: {opt_info:?}"
    )]
    ValueFromStringFailed {
        src_value: String,
        target_type: String,
        opt_info: Option<String>,
    },
}

// The Error derive also implements the Display trait!
#[derive(Error, Debug, PartialEq, Clone)]
pub enum ConversionError {
    #[error("Can't convert {src_type:?} with value {src_value:?} to target type {target_type:?}. Optional info: {opt_info:?}")]
    WrongType {
        src_value: String,
        src_type: String,
        target_type: String,
        opt_info: Option<String>,
    },
}

pub type Result<T> = std::result::Result<T, VenumError>;
