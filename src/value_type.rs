use strum_macros::Display; // used to generate names for the enum variants. Used only for error messages (as of now).

use crate::{
    errors_result::{Result, VenumError},
    value::Value,
};

#[derive(Default, Display, Debug, Clone, PartialEq, Eq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ValueType {
    Char,
    String,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    Float32,
    Float64,
    #[default]
    Bool,
    Decimal,
    NaiveDate,
    NaiveDateTime,
    DateTime,
}

impl ValueType {
    pub fn is_some_date_type(&self) -> bool {
        matches!(
            self,
            ValueType::NaiveDate | ValueType::NaiveDateTime | ValueType::DateTime
        )
    }

    pub fn is_some_signed_int_type(&self) -> bool {
        matches!(
            self,
            ValueType::Int8
                | ValueType::Int16
                | ValueType::Int32
                | ValueType::Int64
                | ValueType::Int128
        )
    }

    pub fn is_some_unsigned_int_type(&self) -> bool {
        matches!(
            self,
            ValueType::UInt8
                | ValueType::UInt16
                | ValueType::UInt32
                | ValueType::UInt64
                | ValueType::UInt128
        )
    }

    pub fn is_some_int_type(&self) -> bool {
        self.is_some_signed_int_type() || self.is_some_unsigned_int_type()
    }

    pub fn is_some_float_type(&self) -> bool {
        matches!(self, ValueType::Float32 | ValueType::Float64)
    }
}

impl TryFrom<&Value> for ValueType {
    type Error = VenumError;

    fn try_from(value: &Value) -> Result<Self> {
        match value {
            Value::None => Err(VenumError::Generic {
                msg: format!(
                    "Cannot convert {} as it has no correspondence in target",
                    value
                ),
            }),
            Value::Char(_) => Ok(ValueType::Char),
            Value::String(_) => Ok(ValueType::String),
            Value::Int8(_) => Ok(ValueType::Int8),
            Value::Int16(_) => Ok(ValueType::Int16),
            Value::Int32(_) => Ok(ValueType::Int32),
            Value::Int64(_) => Ok(ValueType::Int64),
            Value::Int128(_) => Ok(ValueType::Int128),
            Value::UInt8(_) => Ok(ValueType::UInt8),
            Value::UInt16(_) => Ok(ValueType::UInt16),
            Value::UInt32(_) => Ok(ValueType::UInt32),
            Value::UInt64(_) => Ok(ValueType::UInt64),
            Value::UInt128(_) => Ok(ValueType::UInt128),
            Value::Float32(_) => Ok(ValueType::Float32),
            Value::Float64(_) => Ok(ValueType::Float64),
            Value::Bool(_) => Ok(ValueType::Bool),
            Value::Decimal(_) => Ok(ValueType::Decimal),
            Value::NaiveDate(_) => Ok(ValueType::NaiveDate),
            Value::NaiveDateTime(_) => Ok(ValueType::NaiveDateTime),
            Value::DateTime(_) => Ok(ValueType::DateTime),
        }
    }
}

impl TryFrom<Value> for ValueType {
    type Error = VenumError;

    fn try_from(value: Value) -> Result<Self> {
        ValueType::try_from(&value)
    }
}
