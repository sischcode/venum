use strum_macros::Display;
use thiserror::Error;

use crate::{value::Value, value_type::ValueType};

#[derive(Debug, PartialEq, Clone, Display)]
pub enum VenumError {
    Generic { msg: String }, // TODO: can we get a nice error message here as well?
    Parsing(ParseError),
    Conversion(ConversionError),
}

// The Error derive also implements the Display trait!
#[derive(Error, Debug, PartialEq, Clone)]
pub enum ParseError {
    #[error(
        "Can't parse string '{src_value:?}' to construct a {target_type:?}. Details: {details:?}"
    )]
    ValueFromStringFailed {
        src_value: String,
        target_type: String,
        details: Option<String>,
    },
}

// The Error derive also implements the Display trait!
#[derive(Error, Debug, PartialEq, Clone)]
pub enum ConversionError {
    #[error("Can't convert {src_type:?} with value {src_value:?} to target type {target_type:?}. Details: {details:?}")]
    WrongType {
        src_value: String,
        src_type: String,
        target_type: String,
        details: Option<String>,
    },

    #[error("{msg:?}")]
    Generic { msg: String },

    #[error("{src:?} not representable as {target_type:?}")]
    NotRepresentableAs { src: Value, target_type: ValueType },

    #[error("{src_value:?} ({src_type:?}) not representable as Decimal")]
    NotRepresentableAsDecimal { src_type: String, src_value: String },
}

pub type Result<T> = std::result::Result<T, VenumError>;
