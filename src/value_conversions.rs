// #![warn(
//     clippy::cast_possible_truncation,
//     clippy::cast_lossless,
//     clippy::cast_possible_wrap,
//     clippy::cast_precision_loss,
//     clippy::cast_sign_loss
// )]

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;

use crate::errors_result::{ConversionError, Result, VenumError};
use crate::value::Value;
use crate::value_type::ValueType;

const DEFAULT_RADIX_10: u32 = 10;

fn mk_not_rep_err(s: &Value, tt: ValueType) -> VenumError {
    VenumError::Conversion(ConversionError::NotRepresentableAs {
        src: s.clone(),
        target_type: tt,
    })
}

impl Value {
    // TODO: docu

    pub fn try_convert_to_char(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::Char;

        match self_type {
            ValueType::Char => Ok(self.clone()),
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    return Err(mk_not_rep_err(self, target_type));
                }
                let mut self_val_chars_iter = self_val.chars();
                let self_val_char = self_val_chars_iter.next().unwrap(); // there must at least be something, initially!
                match self_val_chars_iter.next() {
                    // however, if there is more, we have an error, as it is not a single char
                    Some(_) => Err(VenumError::Conversion(
                        ConversionError::NotRepresentableAs {
                            src: self.clone(),
                            target_type,
                        },
                    )),
                    None => Ok(Value::Char(self_val_char)),
                }
            }
            ValueType::UInt8 => {
                let self_val: u8 = self.try_into()?; // should never fail!
                let self_val_as_u32: u32 = self_val.into();
                let self_val_as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Char(self_val_as_char))
            }
            ValueType::UInt16 => {
                let self_val: u16 = self.try_into()?; // should never fail!
                let self_val_as_u32: u32 = self_val.into();
                let self_val_as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Char(self_val_as_char))
            }
            ValueType::UInt32 => {
                let self_val: u32 = self.try_into()?; // should never fail!
                let self_val_as_char = char::from_digit(self_val, DEFAULT_RADIX_10)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Char(self_val_as_char))
            }
            ValueType::UInt64 => {
                let self_val: u64 = self.try_into()?; // should never fail!
                let self_val_as_u32: u32 = self_val.try_into().map_err(|_err| {
                    VenumError::Conversion(ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type: target_type.clone(),
                    })
                })?;
                let self_val_as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Char(self_val_as_char))
            }
            ValueType::UInt128 => {
                let self_val: u128 = self.try_into()?; // should never fail!
                let self_val_as_u32: u32 = self_val.try_into().map_err(|_err| {
                    VenumError::Conversion(ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type: target_type.clone(),
                    })
                })?;
                let self_val_as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Char(self_val_as_char))
            }
            ValueType::Int8 => {
                let self_val: i8 = self.try_into()?; // should never fail!
                if self_val >= 0 {
                    let self_val_as_u32 = u32::try_from(self_val).unwrap(); // checked above and a positive signed int must fit into an usigned one, if the value is positive
                    let self_val_as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                        .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Char(self_val_as_char))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Int16 => {
                let self_val: i16 = self.try_into()?; // should never fail!
                if self_val >= 0 {
                    let self_val_as_u32 = u32::try_from(self_val).unwrap(); // checked above and a positive signed int must fit into an usigned one, if the value is positive
                    let self_val_as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                        .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Char(self_val_as_char))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Int32 => {
                let self_val: i32 = self.try_into()?; // should never fail!
                if self_val >= 0 {
                    let self_val_as_u32 = u32::try_from(self_val).unwrap(); // checked above and a positive signed int must fit into an usigned one, if the value is positive
                    let self_val_as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                        .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Char(self_val_as_char))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Int64 => {
                let self_val: i64 = self.try_into()?; // should never fail!
                if self_val >= 0 {
                    let self_val_as_u32 = u32::try_from(self_val).unwrap(); // checked above and a positive signed int must fit into an usigned one, if the value is positive
                    let self_val_as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                        .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Char(self_val_as_char))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Int128 => {
                let self_val: i128 = self.try_into()?; // should never fail!
                if self_val >= 0 {
                    let self_val_as_u32 = u32::try_from(self_val).unwrap(); // checked above and a positive signed int must fit into an usigned one, if the value is positive
                    let self_val_as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                        .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Char(self_val_as_char))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Float32 => Err(mk_not_rep_err(self, target_type)),
            ValueType::Float64 => Err(mk_not_rep_err(self, target_type)),
            ValueType::Bool => Err(mk_not_rep_err(self, target_type)),
            ValueType::Decimal => Err(mk_not_rep_err(self, target_type)),
            ValueType::NaiveDate | ValueType::NaiveDateTime | ValueType::DateTime => {
                Err(mk_not_rep_err(self, target_type))
            }
        }
    }

    pub fn try_convert_to_string(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error

        match self_type {
            ValueType::Char => {
                let val: char = self.try_into()?; // should never fail
                Ok(Value::String(val.to_string()))
            }
            ValueType::String => Ok(self.clone()),
            ValueType::Int8 => {
                let self_val: i8 = self.try_into()?;
                Ok(Value::String(self_val.to_string()))
            }
            ValueType::Int16 => {
                let self_val: i16 = self.try_into()?;
                Ok(Value::String(self_val.to_string()))
            }
            ValueType::Int32 => {
                let self_val: i32 = self.try_into()?;
                Ok(Value::String(self_val.to_string()))
            }
            ValueType::Int64 => {
                let self_val: i64 = self.try_into()?;
                Ok(Value::String(self_val.to_string()))
            }
            ValueType::Int128 => {
                let self_val: i128 = self.try_into()?;
                Ok(Value::String(self_val.to_string()))
            }
            ValueType::UInt8 => {
                let self_val: u8 = self.try_into()?;
                Ok(Value::String(self_val.to_string()))
            }
            ValueType::UInt16 => {
                let self_val: u16 = self.try_into()?;
                Ok(Value::String(self_val.to_string()))
            }
            ValueType::UInt32 => {
                let self_val: u32 = self.try_into()?;
                Ok(Value::String(self_val.to_string()))
            }
            ValueType::UInt64 => {
                let self_val: u64 = self.try_into()?;
                Ok(Value::String(self_val.to_string()))
            }
            ValueType::UInt128 => {
                let self_val: u128 = self.try_into()?;
                Ok(Value::String(self_val.to_string()))
            }
            ValueType::Float32 => {
                let self_val: f32 = self.try_into()?;
                Ok(Value::String(self_val.to_string()))
            }
            ValueType::Float64 => {
                let self_val: f64 = self.try_into()?;
                Ok(Value::String(self_val.to_string()))
            }
            ValueType::Bool => {
                let self_val: bool = self.try_into()?;
                Ok(Value::String(self_val.to_string()))
            }
            ValueType::Decimal => {
                let self_val: Decimal = self.try_into()?;
                Ok(Value::String(self_val.to_string()))
            }
            ValueType::NaiveDate => {
                let self_val: NaiveDate = self.try_into()?; // TODO format
                Ok(Value::String(self_val.to_string()))
            }
            ValueType::NaiveDateTime => {
                let self_val: NaiveDateTime = self.try_into()?; // TODO format
                Ok(Value::String(self_val.to_string()))
            }
            ValueType::DateTime => {
                let self_val: DateTime<FixedOffset> = self.try_into()?; // TODO format
                Ok(Value::String(self_val.to_string()))
            }
        }
    }

    pub fn try_convert_to_int8(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::Int8;

        match self_type {
            ValueType::Char => {
                let self_val: char = self.try_into()?;
                match self_val.to_digit(DEFAULT_RADIX_10) {
                    Some(self_val_as_digit_u32) => match self_val_as_digit_u32.try_into() {
                        Ok(self_val_as_target_primitive) => {
                            Ok(Value::Int8(self_val_as_target_primitive)) // success!
                        }
                        Err(_) => Err(mk_not_rep_err(self, target_type)),
                    },
                    None => Err(mk_not_rep_err(self, target_type)),
                }
            }
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    let self_val_as_target_primitive: i8 = self_val
                        .parse::<i8>()
                        .map_err(|_err| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Int8(self_val_as_target_primitive))
                }
            }
            ValueType::Int8 => Ok(self.clone()),
            ValueType::Int16 => {
                let self_val_primitive: i16 = self.try_into()?;
                let self_val_as_target_primitive: i8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int8(self_val_as_target_primitive))
            }
            ValueType::Int32 => {
                let self_val_primitive: i32 = self.try_into()?;
                let self_val_as_target_primitive: i8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int8(self_val_as_target_primitive))
            }
            ValueType::Int64 => {
                let self_val_primitive: i64 = self.try_into()?;
                let self_val_as_target_primitive: i8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int8(self_val_as_target_primitive))
            }
            ValueType::Int128 => {
                let self_val_primitive: i128 = self.try_into()?;
                let self_val_as_target_primitive: i8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int8(self_val_as_target_primitive))
            }
            ValueType::UInt8 => {
                let self_val_primitive: u8 = self.try_into()?;
                let self_val_as_target_primitive: i8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int8(self_val_as_target_primitive))
            }
            ValueType::UInt16 => {
                let self_val_primitive: u16 = self.try_into()?;
                let self_val_as_target_primitive: i8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int8(self_val_as_target_primitive))
            }
            ValueType::UInt32 => {
                let self_val_primitive: u32 = self.try_into()?;
                let self_val_as_target_primitive: i8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int8(self_val_as_target_primitive))
            }
            ValueType::UInt64 => {
                let self_val_primitive: u64 = self.try_into()?;
                let self_val_as_target_primitive: i8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int8(self_val_as_target_primitive))
            }
            ValueType::UInt128 => {
                let self_val_primitive: u128 = self.try_into()?;
                let self_val_as_target_primitive: i8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int8(self_val_as_target_primitive))
            }
            ValueType::Float32 => {
                let self_val_primitive: f32 = self.try_into()?;
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive <= i8::MAX.into()
                    && self_val_primitive >= i8::MIN.into()
                {
                    Ok(Value::Int8(self_val_primitive as i8))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Float64 => {
                let self_val_primitive: f64 = self.try_into()?;
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive <= i8::MAX.into()
                    && self_val_primitive >= i8::MIN.into()
                {
                    Ok(Value::Int8(self_val_primitive as i8))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Bool => {
                // TODO: is this really a good idea? Do we assume / imply too much here?
                let self_val_primitive: bool = self.try_into()?;
                if self_val_primitive {
                    Ok(Value::Int8(1))
                } else {
                    Ok(Value::Int8(0))
                }
            }
            ValueType::Decimal => {
                let self_val: Decimal = self.try_into()?;
                if self_val.fract().is_zero() {
                    let self_val_as_target_primitive: i8 = self_val
                        .to_i8()
                        .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Int8(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::NaiveDate | ValueType::NaiveDateTime | ValueType::DateTime => {
                Err(mk_not_rep_err(self, target_type))
            }
        }
    }

    pub fn try_convert_to_int16(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::Int16;

        match self_type {
            ValueType::Char => {
                let self_val: char = self.try_into()?;
                match self_val.to_digit(DEFAULT_RADIX_10) {
                    Some(self_val_as_digit_u32) => match self_val_as_digit_u32.try_into() {
                        Ok(self_val_as_target_primitive) => {
                            Ok(Value::Int16(self_val_as_target_primitive)) // success!
                        }
                        Err(_) => Err(mk_not_rep_err(self, target_type)),
                    },
                    None => Err(mk_not_rep_err(self, target_type)),
                }
            }
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    let self_val_as_target_primitive: i16 = self_val
                        .parse::<i16>()
                        .map_err(|_err| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Int16(self_val_as_target_primitive))
                }
            }
            ValueType::Int8 => {
                let self_val_primitive: i8 = self.try_into()?;
                let self_val_as_target_primitive: i16 = self_val_primitive.into();
                Ok(Value::Int16(self_val_as_target_primitive))
            }
            ValueType::Int16 => Ok(self.clone()),
            ValueType::Int32 => {
                let self_val_primitive: i32 = self.try_into()?;
                let self_val_as_target_primitive: i16 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int16(self_val_as_target_primitive))
            }
            ValueType::Int64 => {
                let self_val_primitive: i64 = self.try_into()?;
                let self_val_as_target_primitive: i16 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int16(self_val_as_target_primitive))
            }
            ValueType::Int128 => {
                let self_val_primitive: i128 = self.try_into()?;
                let self_val_as_target_primitive: i16 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int16(self_val_as_target_primitive))
            }
            ValueType::UInt8 => {
                let self_val_primitive: u8 = self.try_into()?;
                let self_val_as_target_primitive: i16 = self_val_primitive.into();
                Ok(Value::Int16(self_val_as_target_primitive))
            }
            ValueType::UInt16 => {
                let self_val_primitive: u16 = self.try_into()?;
                let self_val_as_target_primitive: i16 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int16(self_val_as_target_primitive))
            }
            ValueType::UInt32 => {
                let self_val_primitive: u32 = self.try_into()?;
                let self_val_as_target_primitive: i16 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int16(self_val_as_target_primitive))
            }
            ValueType::UInt64 => {
                let self_val_primitive: u64 = self.try_into()?;
                let self_val_as_target_primitive: i16 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int16(self_val_as_target_primitive))
            }
            ValueType::UInt128 => {
                let self_val_primitive: u128 = self.try_into()?;
                let self_val_as_target_primitive: i16 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int16(self_val_as_target_primitive))
            }
            ValueType::Float32 => {
                let self_val_primitive: f32 = self.try_into()?;
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive <= i16::MAX.into()
                    && self_val_primitive >= i16::MIN.into()
                {
                    Ok(Value::Int16(self_val_primitive as i16))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Float64 => {
                let self_val_primitive: f64 = self.try_into()?;
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive <= i16::MAX.into()
                    && self_val_primitive >= i16::MIN.into()
                {
                    Ok(Value::Int16(self_val_primitive as i16))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Bool => {
                // TODO: is this really a good idea? Do we assume / imply too much here?
                let self_val_primitive: bool = self.try_into()?;
                if self_val_primitive {
                    Ok(Value::Int16(1))
                } else {
                    Ok(Value::Int16(0))
                }
            }
            ValueType::Decimal => {
                let self_val: Decimal = self.try_into()?;
                if self_val.fract().is_zero() {
                    let self_val_as_target_primitive: i16 = self_val
                        .to_i16()
                        .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Int16(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::NaiveDate | ValueType::NaiveDateTime | ValueType::DateTime => {
                Err(mk_not_rep_err(self, target_type))
            }
        }
    }

    pub fn try_convert_to_int32(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::Int32;

        match self_type {
            ValueType::Char => {
                let self_val: char = self.try_into()?;
                match self_val.to_digit(DEFAULT_RADIX_10) {
                    Some(self_val_as_digit_u32) => match self_val_as_digit_u32.try_into() {
                        Ok(self_val_as_target_primitive) => {
                            Ok(Value::Int32(self_val_as_target_primitive)) // success!
                        }
                        Err(_) => Err(mk_not_rep_err(self, target_type)),
                    },
                    None => Err(mk_not_rep_err(self, target_type)),
                }
            }
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    let self_val_as_target_primitive: i32 = self_val
                        .parse::<i32>()
                        .map_err(|_err| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Int32(self_val_as_target_primitive))
                }
            }
            ValueType::Int8 => {
                let self_val_primitive: i8 = self.try_into()?;
                let self_val_as_target_primitive: i32 = self_val_primitive.into();
                Ok(Value::Int32(self_val_as_target_primitive))
            }
            ValueType::Int16 => {
                let self_val_primitive: i16 = self.try_into()?;
                let self_val_as_target_primitive: i32 = self_val_primitive.into();
                Ok(Value::Int32(self_val_as_target_primitive))
            }
            ValueType::Int32 => Ok(self.clone()),
            ValueType::Int64 => {
                let self_val_primitive: i64 = self.try_into()?;
                let self_val_as_target_primitive: i32 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int32(self_val_as_target_primitive))
            }
            ValueType::Int128 => {
                let self_val_primitive: i128 = self.try_into()?;
                let self_val_as_target_primitive: i32 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int32(self_val_as_target_primitive))
            }
            ValueType::UInt8 => {
                let self_val_primitive: u8 = self.try_into()?;
                let self_val_as_target_primitive: i32 = self_val_primitive.into();
                Ok(Value::Int32(self_val_as_target_primitive))
            }
            ValueType::UInt16 => {
                let self_val_primitive: u16 = self.try_into()?;
                let self_val_as_target_primitive: i32 = self_val_primitive.into();
                Ok(Value::Int32(self_val_as_target_primitive))
            }
            ValueType::UInt32 => {
                let self_val_primitive: u32 = self.try_into()?;
                let self_val_as_target_primitive: i32 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int32(self_val_as_target_primitive))
            }
            ValueType::UInt64 => {
                let self_val_primitive: u64 = self.try_into()?;
                let self_val_as_target_primitive: i32 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int32(self_val_as_target_primitive))
            }
            ValueType::UInt128 => {
                let self_val_primitive: u128 = self.try_into()?;
                let self_val_as_target_primitive: i32 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int32(self_val_as_target_primitive))
            }
            ValueType::Float32 => {
                let self_val_primitive: f32 = self.try_into()?;
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive <= i32::MAX as f32
                    && self_val_primitive >= i32::MIN as f32
                {
                    Ok(Value::Int32(self_val_primitive as i32))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Float64 => {
                let self_val_primitive: f64 = self.try_into()?;
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive <= f64::from(i32::MAX)
                    && self_val_primitive >= f64::from(i32::MIN)
                {
                    Ok(Value::Int32(self_val_primitive as i32))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Bool => {
                // TODO: is this really a good idea? Do we assume / imply too much here?
                let self_val_primitive: bool = self.try_into()?;
                if self_val_primitive {
                    Ok(Value::Int32(1))
                } else {
                    Ok(Value::Int32(0))
                }
            }
            ValueType::Decimal => {
                let self_val: Decimal = self.try_into()?;
                if self_val.fract().is_zero() {
                    let self_val_as_target_primitive: i32 = self_val
                        .to_i32()
                        .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Int32(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::NaiveDate | ValueType::NaiveDateTime | ValueType::DateTime => {
                Err(mk_not_rep_err(self, target_type))
            }
        }
    }

    pub fn try_convert_to_int64(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::Int64;

        match self_type {
            ValueType::Char => {
                let self_val: char = self.try_into()?;
                match self_val.to_digit(DEFAULT_RADIX_10) {
                    Some(self_val_as_digit_u32) => match self_val_as_digit_u32.try_into() {
                        Ok(self_val_as_target_primitive) => {
                            Ok(Value::Int64(self_val_as_target_primitive)) // success!
                        }
                        Err(_) => Err(mk_not_rep_err(self, target_type)),
                    },
                    None => Err(mk_not_rep_err(self, target_type)),
                }
            }
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    let self_val_as_target_primitive: i64 = self_val
                        .parse::<i64>()
                        .map_err(|_err| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Int64(self_val_as_target_primitive))
                }
            }
            ValueType::Int8 => {
                let self_val_primitive: i8 = self.try_into()?;
                let self_val_as_target_primitive: i64 = self_val_primitive.into();
                Ok(Value::Int64(self_val_as_target_primitive))
            }
            ValueType::Int16 => {
                let self_val_primitive: i16 = self.try_into()?;
                let self_val_as_target_primitive: i64 = self_val_primitive.into();
                Ok(Value::Int64(self_val_as_target_primitive))
            }
            ValueType::Int32 => {
                let self_val_primitive: i32 = self.try_into()?;
                let self_val_as_target_primitive: i64 = self_val_primitive.into();
                Ok(Value::Int64(self_val_as_target_primitive))
            }
            ValueType::Int64 => Ok(self.clone()),
            ValueType::Int128 => {
                let self_val_primitive: i128 = self.try_into()?;
                let self_val_as_target_primitive: i64 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int64(self_val_as_target_primitive))
            }
            ValueType::UInt8 => {
                let self_val_primitive: u8 = self.try_into()?;
                let self_val_as_target_primitive: i64 = self_val_primitive.into();
                Ok(Value::Int64(self_val_as_target_primitive))
            }
            ValueType::UInt16 => {
                let self_val_primitive: u16 = self.try_into()?;
                let self_val_as_target_primitive: i64 = self_val_primitive.into();
                Ok(Value::Int64(self_val_as_target_primitive))
            }
            ValueType::UInt32 => {
                let self_val_primitive: u32 = self.try_into()?;
                let self_val_as_target_primitive: i64 = self_val_primitive.into();
                Ok(Value::Int64(self_val_as_target_primitive))
            }
            ValueType::UInt64 => {
                let self_val_primitive: u64 = self.try_into()?;
                let self_val_as_target_primitive: i64 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int64(self_val_as_target_primitive))
            }
            ValueType::UInt128 => {
                let self_val_primitive: u128 = self.try_into()?;
                let self_val_as_target_primitive: i64 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int64(self_val_as_target_primitive))
            }
            ValueType::Float32 => {
                let self_val_primitive: f32 = self.try_into()?;
                // We get the biggest/smallest i64 representable by a f32, then we compare against that.
                // If our f32 value is bigger/smaller than these limits, we cannot represent it as a i64
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive <= i64::MAX as f32
                    && self_val_primitive >= i64::MIN as f32
                {
                    Ok(Value::Int64(self_val_primitive as i64))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Float64 => {
                let self_val_primitive: f64 = self.try_into()?;
                // We get the biggest/smallest i64 representable by a f64, then we compare against that.
                // If our f64 value is bigger/smaller than these limits, we cannot represent it as a i64
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive <= i64::MAX as f64
                    && self_val_primitive >= i64::MIN as f64
                {
                    Ok(Value::Int64(self_val_primitive as i64))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Bool => {
                // TODO: is this really a good idea? Do we assume / imply too much here?
                let self_val_primitive: bool = self.try_into()?;
                if self_val_primitive {
                    Ok(Value::Int64(1))
                } else {
                    Ok(Value::Int64(0))
                }
            }
            ValueType::Decimal => {
                let self_val: Decimal = self.try_into()?;
                if self_val.fract().is_zero() {
                    let self_val_as_target_primitive: i64 = self_val
                        .to_i64()
                        .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Int64(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::NaiveDate | ValueType::NaiveDateTime | ValueType::DateTime => {
                Err(mk_not_rep_err(self, target_type))
            }
        }
    }

    pub fn try_convert_to_int128(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::Int128;

        match self_type {
            ValueType::Char => {
                let self_val: char = self.try_into()?;
                match self_val.to_digit(DEFAULT_RADIX_10) {
                    Some(self_val_as_digit_u32) => match self_val_as_digit_u32.try_into() {
                        Ok(self_val_as_target_primitive) => {
                            Ok(Value::Int128(self_val_as_target_primitive)) // success!
                        }
                        Err(_) => Err(mk_not_rep_err(self, target_type)),
                    },
                    None => Err(mk_not_rep_err(self, target_type)),
                }
            }
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    let self_val_as_target_primitive: i128 = self_val
                        .parse::<i128>()
                        .map_err(|_err| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Int128(self_val_as_target_primitive))
                }
            }
            ValueType::Int8 => {
                let self_val_primitive: i8 = self.try_into()?;
                let self_val_as_target_primitive: i128 = self_val_primitive.into();
                Ok(Value::Int128(self_val_as_target_primitive))
            }
            ValueType::Int16 => {
                let self_val_primitive: i16 = self.try_into()?;
                let self_val_as_target_primitive: i128 = self_val_primitive.into();
                Ok(Value::Int128(self_val_as_target_primitive))
            }
            ValueType::Int32 => {
                let self_val_primitive: i32 = self.try_into()?;
                let self_val_as_target_primitive: i128 = self_val_primitive.into();
                Ok(Value::Int128(self_val_as_target_primitive))
            }
            ValueType::Int64 => {
                let self_val_primitive: i64 = self.try_into()?;
                let self_val_as_target_primitive: i128 = self_val_primitive.into();
                Ok(Value::Int128(self_val_as_target_primitive))
            }
            ValueType::Int128 => Ok(self.clone()),
            ValueType::UInt8 => {
                let self_val_primitive: u8 = self.try_into()?;
                let self_val_as_target_primitive: i128 = self_val_primitive.into();
                Ok(Value::Int128(self_val_as_target_primitive))
            }
            ValueType::UInt16 => {
                let self_val_primitive: u16 = self.try_into()?;
                let self_val_as_target_primitive: i128 = self_val_primitive.into();
                Ok(Value::Int128(self_val_as_target_primitive))
            }
            ValueType::UInt32 => {
                let self_val_primitive: u32 = self.try_into()?;
                let self_val_as_target_primitive: i128 = self_val_primitive.into();
                Ok(Value::Int128(self_val_as_target_primitive))
            }
            ValueType::UInt64 => {
                let self_val_primitive: u64 = self.try_into()?;
                let self_val_as_target_primitive: i128 = self_val_primitive.into();
                Ok(Value::Int128(self_val_as_target_primitive))
            }
            ValueType::UInt128 => {
                let self_val_primitive: u128 = self.try_into()?;
                let self_val_as_target_primitive: i128 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::Int128(self_val_as_target_primitive))
            }
            ValueType::Float32 => {
                let self_val_primitive: f32 = self.try_into()?;
                // We get the biggest/smallest i128 representable by a f32, then we compare against that.
                // If our f32 value is bigger/smaller than these limits, we cannot represent it as a i128
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive <= i128::MAX as f32
                    && self_val_primitive >= i128::MIN as f32
                {
                    Ok(Value::Int128(self_val_primitive as i128))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Float64 => {
                let self_val_primitive: f64 = self.try_into()?;
                // We get the biggest/smallest i128 representable by a f64, then we compare against that.
                // If our f64 value is bigger/smaller than these limits, we cannot represent it as a i128
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive <= i128::MAX as f64
                    && self_val_primitive >= i128::MIN as f64
                {
                    Ok(Value::Int128(self_val_primitive as i128))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Bool => {
                // TODO: is this really a good idea? Do we assume / imply too much here?
                let self_val_primitive: bool = self.try_into()?;
                if self_val_primitive {
                    Ok(Value::Int128(1))
                } else {
                    Ok(Value::Int128(0))
                }
            }
            ValueType::Decimal => {
                let self_val: Decimal = self.try_into()?;
                if self_val.fract().is_zero() {
                    let self_val_as_target_primitive: i128 = self_val
                        .to_i128()
                        .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Int128(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::NaiveDate | ValueType::NaiveDateTime | ValueType::DateTime => {
                Err(mk_not_rep_err(self, target_type))
            }
        }
    }

    pub fn try_convert_to_uint8(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::UInt8;

        match self_type {
            ValueType::Char => {
                let self_val: char = self.try_into()?;
                match self_val.to_digit(DEFAULT_RADIX_10) {
                    Some(self_val_as_digit_u32) => match self_val_as_digit_u32.try_into() {
                        Ok(self_val_as_target_primitive) => {
                            Ok(Value::UInt8(self_val_as_target_primitive)) // success!
                        }
                        Err(_) => Err(mk_not_rep_err(self, target_type)),
                    },
                    None => Err(mk_not_rep_err(self, target_type)),
                }
            }
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    let self_val_as_target_primitive: u8 =
                        self_val.parse::<u8>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::UInt8(self_val_as_target_primitive))
                }
            }
            ValueType::Int8 => {
                let self_val_primitive: i8 = self.try_into()?;
                let self_val_as_target_primitive: u8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt8(self_val_as_target_primitive))
            }
            ValueType::Int16 => {
                let self_val_primitive: i16 = self.try_into()?;
                let self_val_as_target_primitive: u8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt8(self_val_as_target_primitive))
            }
            ValueType::Int32 => {
                let self_val_primitive: i32 = self.try_into()?;
                let self_val_as_target_primitive: u8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt8(self_val_as_target_primitive))
            }
            ValueType::Int64 => {
                let self_val_primitive: i64 = self.try_into()?;
                let self_val_as_target_primitive: u8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt8(self_val_as_target_primitive))
            }
            ValueType::Int128 => {
                let self_val_primitive: i128 = self.try_into()?;
                let self_val_as_target_primitive: u8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt8(self_val_as_target_primitive))
            }
            ValueType::UInt8 => Ok(self.clone()),
            ValueType::UInt16 => {
                let self_val_primitive: u16 = self.try_into()?;
                let self_val_as_target_primitive: u8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt8(self_val_as_target_primitive))
            }
            ValueType::UInt32 => {
                let self_val_primitive: u32 = self.try_into()?;
                let self_val_as_target_primitive: u8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt8(self_val_as_target_primitive))
            }
            ValueType::UInt64 => {
                let self_val_primitive: u64 = self.try_into()?;
                let self_val_as_target_primitive: u8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt8(self_val_as_target_primitive))
            }
            ValueType::UInt128 => {
                let self_val_primitive: u128 = self.try_into()?;
                let self_val_as_target_primitive: u8 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt8(self_val_as_target_primitive))
            }
            ValueType::Float32 => {
                let self_val_primitive: f32 = self.try_into()?;
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive >= 0.0
                    && self_val_primitive <= u8::MAX.into()
                {
                    Ok(Value::UInt8(self_val_primitive as u8))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Float64 => {
                let self_val_primitive: f64 = self.try_into()?;
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive >= 0.0
                    && self_val_primitive <= u8::MAX.into()
                {
                    Ok(Value::UInt8(self_val_primitive as u8))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Bool => {
                // TODO: is this really a good idea? Do we assume / imply too much here?
                let self_val_primitive: bool = self.try_into()?;
                if self_val_primitive {
                    Ok(Value::UInt8(1))
                } else {
                    Ok(Value::UInt8(0))
                }
            }
            ValueType::Decimal => {
                let self_val: Decimal = self.try_into()?;
                if self_val.is_sign_positive() && self_val.fract().is_zero() {
                    let self_val_as_target_primitive: u8 = self_val
                        .to_u8()
                        .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                    Ok(Value::UInt8(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::NaiveDate | ValueType::NaiveDateTime | ValueType::DateTime => {
                Err(mk_not_rep_err(self, target_type))
            }
        }
    }

    pub fn try_convert_to_uint16(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::UInt16;

        match self_type {
            ValueType::Char => {
                let self_val: char = self.try_into()?;
                match self_val.to_digit(DEFAULT_RADIX_10) {
                    Some(self_val_as_digit_u32) => match self_val_as_digit_u32.try_into() {
                        Ok(self_val_as_target_primitive) => {
                            Ok(Value::UInt16(self_val_as_target_primitive)) // success!
                        }
                        Err(_) => Err(mk_not_rep_err(self, target_type)),
                    },
                    None => Err(mk_not_rep_err(self, target_type)),
                }
            }
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    let self_val_as_target_primitive: u16 =
                        self_val.parse::<u16>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::UInt16(self_val_as_target_primitive))
                }
            }
            ValueType::Int8 => {
                let self_val_primitive: i8 = self.try_into()?;
                let self_val_as_target_primitive: u16 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt16(self_val_as_target_primitive))
            }
            ValueType::Int16 => {
                let self_val_primitive: i16 = self.try_into()?;
                let self_val_as_target_primitive: u16 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt16(self_val_as_target_primitive))
            }
            ValueType::Int32 => {
                let self_val_primitive: i32 = self.try_into()?;
                let self_val_as_target_primitive: u16 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt16(self_val_as_target_primitive))
            }
            ValueType::Int64 => {
                let self_val_primitive: i64 = self.try_into()?;
                let self_val_as_target_primitive: u16 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt16(self_val_as_target_primitive))
            }
            ValueType::Int128 => {
                let self_val_primitive: i128 = self.try_into()?;
                let self_val_as_target_primitive: u16 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt16(self_val_as_target_primitive))
            }
            ValueType::UInt8 => {
                let self_val_primitive: u8 = self.try_into()?;
                let self_val_as_target_primitive: u16 = self_val_primitive.into();
                Ok(Value::UInt16(self_val_as_target_primitive))
            }
            ValueType::UInt16 => Ok(self.clone()),
            ValueType::UInt32 => {
                let self_val_primitive: u32 = self.try_into()?;
                let self_val_as_target_primitive: u16 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt16(self_val_as_target_primitive))
            }
            ValueType::UInt64 => {
                let self_val_primitive: u64 = self.try_into()?;
                let self_val_as_target_primitive: u16 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt16(self_val_as_target_primitive))
            }
            ValueType::UInt128 => {
                let self_val_primitive: u128 = self.try_into()?;
                let self_val_as_target_primitive: u16 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt16(self_val_as_target_primitive))
            }
            ValueType::Float32 => {
                let self_val_primitive: f32 = self.try_into()?;
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive >= 0.0
                    && self_val_primitive <= u16::MAX.into()
                {
                    Ok(Value::UInt16(self_val_primitive as u16))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Float64 => {
                let self_val_primitive: f64 = self.try_into()?;
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive >= 0.0
                    && self_val_primitive <= u16::MAX.into()
                {
                    Ok(Value::UInt16(self_val_primitive as u16))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Bool => {
                // TODO: is this really a good idea? Do we assume / imply too much here?
                let self_val_primitive: bool = self.try_into()?;
                if self_val_primitive {
                    Ok(Value::UInt16(1))
                } else {
                    Ok(Value::UInt16(0))
                }
            }
            ValueType::Decimal => {
                let self_val: Decimal = self.try_into()?;
                if self_val.is_sign_positive() && self_val.fract().is_zero() {
                    let self_val_as_target_primitive: u16 = self_val
                        .to_u16()
                        .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                    Ok(Value::UInt16(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::NaiveDate | ValueType::NaiveDateTime | ValueType::DateTime => {
                Err(mk_not_rep_err(self, target_type))
            }
        }
    }

    pub fn try_convert_to_uint32(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::UInt32;

        match self_type {
            ValueType::Char => {
                let self_val: char = self.try_into()?;
                match self_val.to_digit(DEFAULT_RADIX_10) {
                    Some(self_val_as_digit_u32) => Ok(Value::UInt32(self_val_as_digit_u32)), // success!
                    None => Err(mk_not_rep_err(self, target_type)),
                }
            }
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    let self_val_as_target_primitive: u32 =
                        self_val.parse::<u32>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::UInt32(self_val_as_target_primitive))
                }
            }
            ValueType::Int8 => {
                let self_val_primitive: i8 = self.try_into()?;
                let self_val_as_target_primitive: u32 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt32(self_val_as_target_primitive))
            }
            ValueType::Int16 => {
                let self_val_primitive: i16 = self.try_into()?;
                let self_val_as_target_primitive: u32 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt32(self_val_as_target_primitive))
            }
            ValueType::Int32 => {
                let self_val_primitive: i32 = self.try_into()?;
                let self_val_as_target_primitive: u32 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt32(self_val_as_target_primitive))
            }
            ValueType::Int64 => {
                let self_val_primitive: i64 = self.try_into()?;
                let self_val_as_target_primitive: u32 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt32(self_val_as_target_primitive))
            }
            ValueType::Int128 => {
                let self_val_primitive: i128 = self.try_into()?;
                let self_val_as_target_primitive: u32 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt32(self_val_as_target_primitive))
            }
            ValueType::UInt8 => {
                let self_val_primitive: u8 = self.try_into()?;
                let self_val_as_target_primitive: u32 = self_val_primitive.into();
                Ok(Value::UInt32(self_val_as_target_primitive))
            }
            ValueType::UInt16 => {
                let self_val_primitive: u16 = self.try_into()?;
                let self_val_as_target_primitive: u32 = self_val_primitive.into();
                Ok(Value::UInt32(self_val_as_target_primitive))
            }
            ValueType::UInt32 => Ok(self.clone()),
            ValueType::UInt64 => {
                let self_val_primitive: u64 = self.try_into()?;
                let self_val_as_target_primitive: u32 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt32(self_val_as_target_primitive))
            }
            ValueType::UInt128 => {
                let self_val_primitive: u128 = self.try_into()?;
                let self_val_as_target_primitive: u32 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt32(self_val_as_target_primitive))
            }
            ValueType::Float32 => {
                let self_val_primitive: f32 = self.try_into()?;
                // We get the biggest u32 representable by a f32, then we compare against that.
                // If our f32 value is bigger than the limit, we cannot represent it as a u32
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive >= 0.0
                    && self_val_primitive <= u32::MAX as f32
                {
                    Ok(Value::UInt32(self_val_primitive as u32))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Float64 => {
                let self_val_primitive: f64 = self.try_into()?;
                // We get the biggest u32 representable by a f64, then we compare against that.
                // If our f64 value is bigger than the limit, we cannot represent it as a u32
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive >= 0.0
                    && self_val_primitive <= f64::from(u32::MAX)
                {
                    Ok(Value::UInt32(self_val_primitive as u32))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Bool => {
                // TODO: is this really a good idea? Do we assume / imply too much here?
                let self_val_primitive: bool = self.try_into()?;
                if self_val_primitive {
                    Ok(Value::UInt32(1))
                } else {
                    Ok(Value::UInt32(0))
                }
            }
            ValueType::Decimal => {
                let self_val: Decimal = self.try_into()?;
                if self_val.is_sign_positive() && self_val.fract().is_zero() {
                    let self_val_as_target_primitive: u32 = self_val
                        .to_u32()
                        .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                    Ok(Value::UInt32(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::NaiveDate | ValueType::NaiveDateTime | ValueType::DateTime => {
                Err(mk_not_rep_err(self, target_type))
            }
        }
    }

    pub fn try_convert_to_uint64(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::UInt64;

        match self_type {
            ValueType::Char => {
                let self_val: char = self.try_into()?;
                match self_val.to_digit(DEFAULT_RADIX_10) {
                    Some(self_val_as_digit_u32) => match self_val_as_digit_u32.try_into() {
                        Ok(self_val_as_target_primitive) => {
                            Ok(Value::UInt64(self_val_as_target_primitive)) // success!
                        }
                        Err(_) => Err(mk_not_rep_err(self, target_type)),
                    },
                    None => Err(mk_not_rep_err(self, target_type)),
                }
            }
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    let self_val_as_target_primitive: u64 =
                        self_val.parse::<u64>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::UInt64(self_val_as_target_primitive))
                }
            }
            ValueType::Int8 => {
                let self_val_primitive: i8 = self.try_into()?;
                let self_val_as_target_primitive: u64 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt64(self_val_as_target_primitive))
            }
            ValueType::Int16 => {
                let self_val_primitive: i16 = self.try_into()?;
                let self_val_as_target_primitive: u64 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt64(self_val_as_target_primitive))
            }
            ValueType::Int32 => {
                let self_val_primitive: i32 = self.try_into()?;
                let self_val_as_target_primitive: u64 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt64(self_val_as_target_primitive))
            }
            ValueType::Int64 => {
                let self_val_primitive: i64 = self.try_into()?;
                let self_val_as_target_primitive: u64 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt64(self_val_as_target_primitive))
            }
            ValueType::Int128 => {
                let self_val_primitive: i128 = self.try_into()?;
                let self_val_as_target_primitive: u64 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt64(self_val_as_target_primitive))
            }
            ValueType::UInt8 => {
                let self_val_primitive: u8 = self.try_into()?;
                let self_val_as_target_primitive: u64 = self_val_primitive.into();
                Ok(Value::UInt64(self_val_as_target_primitive))
            }
            ValueType::UInt16 => {
                let self_val_primitive: u16 = self.try_into()?;
                let self_val_as_target_primitive: u64 = self_val_primitive.into();
                Ok(Value::UInt64(self_val_as_target_primitive))
            }
            ValueType::UInt32 => {
                let self_val_primitive: u32 = self.try_into()?;
                let self_val_as_target_primitive: u64 = self_val_primitive.into();
                Ok(Value::UInt64(self_val_as_target_primitive))
            }
            ValueType::UInt64 => Ok(self.clone()),
            ValueType::UInt128 => {
                let self_val_primitive: u128 = self.try_into()?;
                let self_val_as_target_primitive: u64 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt64(self_val_as_target_primitive))
            }
            ValueType::Float32 => {
                let self_val_primitive: f32 = self.try_into()?;
                // We get the biggest u64 representable by a f32, then we compare against that.
                // If our f32 value is bigger than the limit, we cannot represent it as a u64
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive >= 0.0
                    && self_val_primitive <= u64::MAX as f32
                {
                    Ok(Value::UInt64(self_val_primitive as u64))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Float64 => {
                // We get the biggest u64 representable by a f64, then we compare against that.
                // If our f64 value is bigger than the limit, we cannot represent it as a u64
                let self_val_primitive: f64 = self.try_into()?;
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive >= 0.0
                    && self_val_primitive <= u64::MAX as f64
                {
                    Ok(Value::UInt64(self_val_primitive as u64))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Bool => {
                // TODO: is this really a good idea? Do we assume / imply too much here?
                let self_val_primitive: bool = self.try_into()?;
                if self_val_primitive {
                    Ok(Value::UInt64(1))
                } else {
                    Ok(Value::UInt64(0))
                }
            }
            ValueType::Decimal => {
                let self_val: Decimal = self.try_into()?;
                if self_val.is_sign_positive() && self_val.fract().is_zero() {
                    let self_val_as_target_primitive: u64 = self_val
                        .to_u64()
                        .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                    Ok(Value::UInt64(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::NaiveDate | ValueType::NaiveDateTime | ValueType::DateTime => {
                Err(mk_not_rep_err(self, target_type))
            }
        }
    }

    pub fn try_convert_to_uint128(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::UInt128;

        match self_type {
            ValueType::Char => {
                let self_val: char = self.try_into()?;
                match self_val.to_digit(DEFAULT_RADIX_10) {
                    Some(self_val_as_digit_u32) => match self_val_as_digit_u32.try_into() {
                        Ok(self_val_as_target_primitive) => {
                            Ok(Value::UInt128(self_val_as_target_primitive))
                            // success!
                        }
                        Err(_) => Err(mk_not_rep_err(self, target_type)),
                    },
                    None => Err(mk_not_rep_err(self, target_type)),
                }
            }
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    let self_val_as_target_primitive: u128 =
                        self_val.parse::<u128>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::UInt128(self_val_as_target_primitive))
                }
            }
            ValueType::Int8 => {
                let self_val_primitive: i8 = self.try_into()?;
                let self_val_as_target_primitive: u128 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt128(self_val_as_target_primitive))
            }
            ValueType::Int16 => {
                let self_val_primitive: i16 = self.try_into()?;
                let self_val_as_target_primitive: u128 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt128(self_val_as_target_primitive))
            }
            ValueType::Int32 => {
                let self_val_primitive: i32 = self.try_into()?;
                let self_val_as_target_primitive: u128 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt128(self_val_as_target_primitive))
            }
            ValueType::Int64 => {
                let self_val_primitive: i64 = self.try_into()?;
                let self_val_as_target_primitive: u128 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt128(self_val_as_target_primitive))
            }
            ValueType::Int128 => {
                let self_val_primitive: i128 = self.try_into()?;
                let self_val_as_target_primitive: u128 = self_val_primitive
                    .try_into()
                    .map_err(|_err| mk_not_rep_err(self, target_type))?;
                Ok(Value::UInt128(self_val_as_target_primitive))
            }
            ValueType::UInt8 => {
                let self_val_primitive: u8 = self.try_into()?;
                let self_val_as_target_primitive: u128 = self_val_primitive.into();
                Ok(Value::UInt128(self_val_as_target_primitive))
            }
            ValueType::UInt16 => {
                let self_val_primitive: u16 = self.try_into()?;
                let self_val_as_target_primitive: u128 = self_val_primitive.into();
                Ok(Value::UInt128(self_val_as_target_primitive))
            }
            ValueType::UInt32 => {
                let self_val_primitive: u32 = self.try_into()?;
                let self_val_as_target_primitive: u128 = self_val_primitive.into();
                Ok(Value::UInt128(self_val_as_target_primitive))
            }
            ValueType::UInt64 => {
                let self_val_primitive: u64 = self.try_into()?;
                let self_val_as_target_primitive: u128 = self_val_primitive.into();
                Ok(Value::UInt128(self_val_as_target_primitive))
            }
            ValueType::UInt128 => Ok(self.clone()),
            ValueType::Float32 => {
                let self_val_primitive: f32 = self.try_into()?;
                if self_val_primitive.fract() == 0.0 && self_val_primitive > 0.0 {
                    Ok(Value::UInt128(self_val_primitive as u128))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Float64 => {
                let self_val_primitive: f64 = self.try_into()?;
                // We get the biggest u128 representable by a f64, then we compare against that.
                // If our f64 value is bigger than the limit, we cannot represent it as a u128
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive >= 0.0
                    && self_val_primitive <= u128::MAX as f64
                {
                    Ok(Value::UInt128(self_val_primitive as u128))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Bool => {
                // TODO: is this really a good idea? Do we assume / imply too much here?
                let self_val_primitive: bool = self.try_into()?;
                if self_val_primitive {
                    Ok(Value::UInt128(1))
                } else {
                    Ok(Value::UInt128(0))
                }
            }
            ValueType::Decimal => {
                let self_val: Decimal = self.try_into()?;
                if self_val.is_sign_positive() && self_val.fract().is_zero() {
                    let self_val_as_target_primitive: u128 = self_val
                        .to_u128()
                        .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                    Ok(Value::UInt128(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::NaiveDate | ValueType::NaiveDateTime | ValueType::DateTime => {
                Err(mk_not_rep_err(self, target_type))
            }
        }
    }

    pub fn try_convert_to_float32(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::Float32;

        match self_type {
            ValueType::Char => Err(mk_not_rep_err(self, target_type)),
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    let self_val_as_target_primitive: f32 = self_val
                        .parse::<f32>()
                        .map_err(|_err| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Float32(self_val_as_target_primitive))
                }
            }
            ValueType::Int8 => {
                let self_val_primitive: i8 = self.try_into()?;
                let self_val_as_target_primitive: f32 = self_val_primitive.into();
                Ok(Value::Float32(self_val_as_target_primitive))
            }
            ValueType::Int16 => {
                let self_val_primitive: i16 = self.try_into()?;
                let self_val_as_target_primitive: f32 = self_val_primitive.into();
                Ok(Value::Float32(self_val_as_target_primitive))
            }
            ValueType::Int32 => {
                let self_val_primitive: i32 = self.try_into()?;
                let self_val_as_interm_primitive: f64 = self_val_primitive.into();

                if self_val_as_interm_primitive <= f32::MAX.into()
                    && self_val_as_interm_primitive >= f32::MIN.into()
                {
                    Ok(Value::Float32(self_val_as_interm_primitive as f32))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Int64 => {
                let self_val_primitive: i64 = self.try_into()?; // what we have
                let self_val_as_target_primitive: f32 = self_val_primitive as f32; // what we want, if possible

                // check that the conversion was losless by casting it back to the source type and checking for equality
                if self_val_primitive == self_val_as_target_primitive as i64 {
                    Ok(Value::Float32(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Int128 => {
                let self_val_primitive: i128 = self.try_into()?;
                let self_val_as_target_primitive: f32 = self_val_primitive as f32;

                if self_val_primitive == self_val_as_target_primitive as i128 {
                    Ok(Value::Float32(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::UInt8 => {
                let self_val_primitive: u8 = self.try_into()?;
                let self_val_as_target_primitive: f32 = self_val_primitive.into();
                Ok(Value::Float32(self_val_as_target_primitive))
            }
            ValueType::UInt16 => {
                let self_val_primitive: u16 = self.try_into()?;
                let self_val_as_target_primitive: f32 = self_val_primitive.into();
                Ok(Value::Float32(self_val_as_target_primitive))
            }
            ValueType::UInt32 => {
                let self_val_primitive: u32 = self.try_into()?;
                let self_val_as_target_primitive: f32 = self_val_primitive as f32;

                if self_val_primitive == self_val_as_target_primitive as u32 {
                    Ok(Value::Float32(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::UInt64 => {
                let self_val_primitive: u64 = self.try_into()?;
                let self_val_as_target_primitive: f32 = self_val_primitive as f32;

                if self_val_primitive == self_val_as_target_primitive as u64 {
                    Ok(Value::Float32(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::UInt128 => {
                let self_val_primitive: u128 = self.try_into()?;
                let self_val_as_target_primitive: f32 = self_val_primitive as f32;

                if self_val_primitive == self_val_as_target_primitive as u128 {
                    Ok(Value::Float32(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Float32 => Ok(self.clone()),
            ValueType::Float64 => {
                let self_val_primitive: f64 = self.try_into()?;
                if self_val_primitive.fract() == 0.0
                    && self_val_primitive <= f32::MAX.into()
                    && self_val_primitive >= f32::MIN.into()
                {
                    Ok(Value::Float32(self_val_primitive as f32))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Bool => Err(mk_not_rep_err(self, target_type)),
            ValueType::Decimal => {
                let self_val_primitive: Decimal = self.try_into()?;
                let self_val_as_target_primitive: f32 = self_val_primitive
                    .to_f32()
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Float32(self_val_as_target_primitive))
            }
            ValueType::NaiveDate | ValueType::NaiveDateTime | ValueType::DateTime => {
                Err(mk_not_rep_err(self, target_type))
            }
        }
    }

    pub fn try_convert_to_float64(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::Float64;

        match self_type {
            ValueType::Char => Err(mk_not_rep_err(self, target_type)),
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    let self_val_as_target_primitive: f64 =
                        self_val.parse::<f64>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::Float64(self_val_as_target_primitive))
                }
            }
            ValueType::Int8 => {
                let self_val_primitive: i8 = self.try_into()?;
                let self_val_as_target_primitive: f64 = self_val_primitive.into();
                Ok(Value::Float64(self_val_as_target_primitive))
            }
            ValueType::Int16 => {
                let self_val_primitive: i16 = self.try_into()?;
                let self_val_as_target_primitive: f64 = self_val_primitive.into();
                Ok(Value::Float64(self_val_as_target_primitive))
            }
            ValueType::Int32 => {
                let self_val_primitive: i32 = self.try_into()?;
                let self_val_as_target_primitive: f64 = self_val_primitive.into();
                Ok(Value::Float64(self_val_as_target_primitive))
            }
            ValueType::Int64 => {
                let self_val_primitive: i64 = self.try_into()?;
                let self_val_as_target_primitive: f64 = self_val_primitive as f64;

                if self_val_primitive == self_val_as_target_primitive as i64 {
                    Ok(Value::Float64(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Int128 => {
                let self_val_primitive: i128 = self.try_into()?;
                let self_val_as_target_primitive: f64 = self_val_primitive as f64;

                if self_val_primitive == self_val_as_target_primitive as i128 {
                    Ok(Value::Float64(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::UInt8 => {
                let self_val_primitive: u8 = self.try_into()?;
                let self_val_as_target_primitive: f64 = self_val_primitive.into();
                Ok(Value::Float64(self_val_as_target_primitive))
            }
            ValueType::UInt16 => {
                let self_val_primitive: u16 = self.try_into()?;
                let self_val_as_target_primitive: f64 = self_val_primitive.into();
                Ok(Value::Float64(self_val_as_target_primitive))
            }
            ValueType::UInt32 => {
                let self_val_primitive: u32 = self.try_into()?;
                let self_val_as_target_primitive: f64 = self_val_primitive.into();
                Ok(Value::Float64(self_val_as_target_primitive))
            }
            ValueType::UInt64 => {
                let self_val_primitive: u64 = self.try_into()?;
                let self_val_as_target_primitive: f64 = self_val_primitive as f64;

                if self_val_primitive == self_val_as_target_primitive as u64 {
                    Ok(Value::Float64(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::UInt128 => {
                let self_val_primitive: u128 = self.try_into()?;
                let self_val_as_target_primitive: f64 = self_val_primitive as f64;

                if self_val_primitive == self_val_as_target_primitive as u128 {
                    Ok(Value::Float64(self_val_as_target_primitive))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Float32 => {
                let self_val_primitive: f32 = self.try_into()?;
                let self_val_as_target_primitive: f64 = self_val_primitive.into();
                Ok(Value::Float64(self_val_as_target_primitive))
            }
            ValueType::Float64 => Ok(self.clone()),
            ValueType::Bool => Err(mk_not_rep_err(self, target_type)),
            ValueType::Decimal => {
                let self_val_primitive: Decimal = self.try_into()?;
                let self_val_as_target_primitive: f64 = self_val_primitive
                    .to_f64()
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Float64(self_val_as_target_primitive))
            }
            ValueType::NaiveDate | ValueType::NaiveDateTime | ValueType::DateTime => {
                Err(mk_not_rep_err(self, target_type))
            }
        }
    }

    pub fn try_convert_to_bool(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::Bool;

        match self_type {
            ValueType::Char => Err(mk_not_rep_err(self, target_type)),
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    let self_val_as_target_primitive: bool = self_val
                        .parse::<bool>()
                        .map_err(|_err| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Bool(self_val_as_target_primitive))
                }
            }
            ValueType::Int8 => {
                let self_val: i8 = self.try_into()?;
                if self_val == 1 {
                    Ok(Value::Bool(true))
                } else if self_val == 0 {
                    Ok(Value::Bool(false))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Int16 => {
                let self_val: i16 = self.try_into()?;
                if self_val == 1 {
                    Ok(Value::Bool(true))
                } else if self_val == 0 {
                    Ok(Value::Bool(false))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Int32 => {
                let self_val: i32 = self.try_into()?;
                if self_val == 1 {
                    Ok(Value::Bool(true))
                } else if self_val == 0 {
                    Ok(Value::Bool(false))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Int64 => {
                let self_val: i64 = self.try_into()?;
                if self_val == 1 {
                    Ok(Value::Bool(true))
                } else if self_val == 0 {
                    Ok(Value::Bool(false))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Int128 => {
                let self_val: i128 = self.try_into()?;
                if self_val == 1 {
                    Ok(Value::Bool(true))
                } else if self_val == 0 {
                    Ok(Value::Bool(false))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::UInt8 => {
                let self_val: u8 = self.try_into()?;
                if self_val == 1 {
                    Ok(Value::Bool(true))
                } else if self_val == 0 {
                    Ok(Value::Bool(false))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::UInt16 => {
                let self_val: u16 = self.try_into()?;
                if self_val == 1 {
                    Ok(Value::Bool(true))
                } else if self_val == 0 {
                    Ok(Value::Bool(false))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::UInt32 => {
                let self_val: u32 = self.try_into()?;
                if self_val == 1 {
                    Ok(Value::Bool(true))
                } else if self_val == 0 {
                    Ok(Value::Bool(false))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::UInt64 => {
                let self_val: u64 = self.try_into()?;
                if self_val == 1 {
                    Ok(Value::Bool(true))
                } else if self_val == 0 {
                    Ok(Value::Bool(false))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::UInt128 => {
                let self_val: u128 = self.try_into()?;
                if self_val == 1 {
                    Ok(Value::Bool(true))
                } else if self_val == 0 {
                    Ok(Value::Bool(false))
                } else {
                    Err(mk_not_rep_err(self, target_type))
                }
            }
            ValueType::Float32 | ValueType::Float64 => Err(mk_not_rep_err(self, target_type)),
            ValueType::Bool => Ok(self.clone()),
            ValueType::Decimal => Err(mk_not_rep_err(self, target_type)),
            ValueType::NaiveDate | ValueType::NaiveDateTime | ValueType::DateTime => {
                Err(mk_not_rep_err(self, target_type))
            }
        }
    }

    pub fn try_convert_to_decimal(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::Decimal;

        match self_type {
            // TODO: debatable if we should convert, e.g. '1' to 1.0
            ValueType::Char => Err(mk_not_rep_err(self, target_type)),
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    let self_val_as_target_type = Decimal::from_str_exact(&self_val)
                        .map_err(|_err| mk_not_rep_err(self, target_type))?;
                    Ok(Value::Decimal(self_val_as_target_type))
                }
            }
            ValueType::Int8 => {
                let self_val_primitive: i8 = self.try_into()?;
                let self_val_as_target_type = Decimal::from_i8(self_val_primitive)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Decimal(self_val_as_target_type))
            }
            ValueType::Int16 => {
                let self_val_primitive: i16 = self.try_into()?;
                let self_val_as_target_type = Decimal::from_i16(self_val_primitive)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Decimal(self_val_as_target_type))
            }
            ValueType::Int32 => {
                let self_val_primitive: i32 = self.try_into()?;
                let self_val_as_target_type = Decimal::from_i32(self_val_primitive)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Decimal(self_val_as_target_type))
            }
            ValueType::Int64 => {
                let self_val_primitive: i64 = self.try_into()?;
                let self_val_as_target_type = Decimal::from_i64(self_val_primitive)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Decimal(self_val_as_target_type))
            }
            ValueType::Int128 => {
                let self_val_primitive: i128 = self.try_into()?;
                let self_val_as_target_type = Decimal::from_i128(self_val_primitive)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Decimal(self_val_as_target_type))
            }
            ValueType::UInt8 => {
                let self_val_primitive: u8 = self.try_into()?;
                let self_val_as_target_type = Decimal::from_u8(self_val_primitive)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Decimal(self_val_as_target_type))
            }
            ValueType::UInt16 => {
                let self_val_primitive: u16 = self.try_into()?;
                let self_val_as_target_type = Decimal::from_u16(self_val_primitive)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Decimal(self_val_as_target_type))
            }
            ValueType::UInt32 => {
                let self_val_primitive: u32 = self.try_into()?;
                let self_val_as_target_type = Decimal::from_u32(self_val_primitive)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Decimal(self_val_as_target_type))
            }
            ValueType::UInt64 => {
                let self_val_primitive: u64 = self.try_into()?;
                let self_val_as_target_type = Decimal::from_u64(self_val_primitive)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Decimal(self_val_as_target_type))
            }
            ValueType::UInt128 => {
                let self_val_primitive: u128 = self.try_into()?;
                let self_val_as_target_type = Decimal::from_u128(self_val_primitive)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Decimal(self_val_as_target_type))
            }
            ValueType::Float32 => {
                let self_val_primitive: f32 = self.try_into()?;
                let self_val_as_target_type = Decimal::from_f32(self_val_primitive)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Decimal(self_val_as_target_type))
            }
            ValueType::Float64 => {
                let self_val_primitive: f64 = self.try_into()?;
                let self_val_as_target_type = Decimal::from_f64(self_val_primitive)
                    .ok_or_else(|| mk_not_rep_err(self, target_type))?;
                Ok(Value::Decimal(self_val_as_target_type))
            }
            ValueType::Bool => Err(mk_not_rep_err(self, target_type)),
            ValueType::Decimal => Ok(self.clone()),
            ValueType::NaiveDate | ValueType::NaiveDateTime | ValueType::DateTime => {
                Err(mk_not_rep_err(self, target_type))
            }
        }
    }

    pub fn try_convert_to_naive_date(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::NaiveDate;

        match self_type {
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    Value::from_str_and_type(&self_val, &target_type)
                        .map_err(|_err| mk_not_rep_err(self, target_type))
                }
            }
            ValueType::NaiveDate => Ok(self.clone()),
            _ => Err(mk_not_rep_err(self, target_type)),
        }
    }

    pub fn try_convert_to_naive_date_time(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::NaiveDateTime;

        match self_type {
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    Value::from_str_and_type(&self_val, &target_type)
                        .map_err(|_err| mk_not_rep_err(self, target_type))
                }
            }
            ValueType::NaiveDateTime => Ok(self.clone()),
            _ => Err(mk_not_rep_err(self, target_type)),
        }
    }

    pub fn try_convert_to_date_time(&self) -> Result<Value> {
        let self_type = ValueType::try_from(self)?; // TODO: wrap error
        let target_type = ValueType::DateTime;

        match self_type {
            ValueType::String => {
                let self_val: String = self.try_into()?;
                if self_val.is_empty() {
                    Err(mk_not_rep_err(self, target_type))
                } else {
                    Value::from_str_and_type(&self_val, &target_type)
                        .map_err(|_err| mk_not_rep_err(self, target_type))
                }
            }
            ValueType::DateTime => Ok(self.clone()),
            _ => Err(mk_not_rep_err(self, target_type)),
        }
    }

    pub fn try_convert_to(&self, target_type: ValueType) -> Result<Value> {
        match target_type {
            ValueType::Char => Self::try_convert_to_char(self),
            ValueType::String => Self::try_convert_to_string(self),
            ValueType::Int8 => Self::try_convert_to_int8(self),
            ValueType::Int16 => Self::try_convert_to_int16(self),
            ValueType::Int32 => Self::try_convert_to_int32(self),
            ValueType::Int64 => Self::try_convert_to_int64(self),
            ValueType::Int128 => Self::try_convert_to_int128(self),
            ValueType::UInt8 => Self::try_convert_to_uint8(self),
            ValueType::UInt16 => Self::try_convert_to_uint16(self),
            ValueType::UInt32 => Self::try_convert_to_uint32(self),
            ValueType::UInt64 => Self::try_convert_to_uint64(self),
            ValueType::UInt128 => Self::try_convert_to_uint128(self),
            ValueType::Float32 => Self::try_convert_to_float32(self),
            ValueType::Float64 => Self::try_convert_to_float64(self),
            ValueType::Bool => Self::try_convert_to_bool(self),
            ValueType::Decimal => Self::try_convert_to_decimal(self),
            ValueType::NaiveDate => Self::try_convert_to_naive_date(self),
            ValueType::NaiveDateTime => Self::try_convert_to_naive_date_time(self),
            ValueType::DateTime => Self::try_convert_to_date_time(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod try_convert_to_char {
        use super::*;

        #[test]
        fn from_char() {
            assert_eq!(
                Value::Char('a'),
                Value::Char('a').try_convert_to_char().unwrap()
            );
        }

        #[test]
        fn from_string() {
            assert_eq!(
                Value::Char('a'),
                Value::String(String::from("a"))
                    .try_convert_to_char()
                    .unwrap()
            );
        }

        #[test]
        fn from_uint8() {
            assert_eq!(
                Value::Char('1'),
                Value::UInt8(1).try_convert_to_char().unwrap()
            );
        }
    }

    mod try_convert_to_string {
        use chrono::TimeZone;

        use super::*;

        #[test]
        fn from_char() {
            assert_eq!(
                Value::String(String::from("a")),
                Value::Char('a').try_convert_to_string().unwrap()
            );
        }

        #[test]
        fn from_string() {
            assert_eq!(
                Value::String(String::from("abc")),
                Value::String(String::from("abc"))
                    .try_convert_to_string()
                    .unwrap()
            );
        }

        #[test]
        fn from_uint8() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::UInt8(1).try_convert_to_string().unwrap()
            );
        }

        #[test]
        fn from_uint16() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::UInt16(1).try_convert_to_string().unwrap()
            );
        }

        #[test]
        fn from_uint32() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::UInt32(1).try_convert_to_string().unwrap()
            );
        }

        #[test]
        fn from_uint64() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::UInt64(1).try_convert_to_string().unwrap()
            );
        }

        #[test]
        fn from_uint128() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::UInt128(1).try_convert_to_string().unwrap()
            );
        }

        #[test]
        fn from_int8() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::Int8(1).try_convert_to_string().unwrap()
            );
        }

        #[test]
        fn from_int16() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::Int16(1).try_convert_to_string().unwrap()
            );
        }

        #[test]
        fn from_int32() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::Int32(1).try_convert_to_string().unwrap()
            );
        }

        #[test]
        fn from_int64() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::Int64(1).try_convert_to_string().unwrap()
            );
        }

        #[test]
        fn from_int128() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::Int128(1).try_convert_to_string().unwrap()
            );
        }

        #[test]
        fn from_float32() {
            assert_eq!(
                Value::String(String::from("123")),
                Value::Float32(123.0).try_convert_to_string().unwrap()
            );
            assert_eq!(
                Value::String(String::from("123.456")),
                Value::Float32(123.456).try_convert_to_string().unwrap()
            );
        }

        #[test]
        fn from_float64() {
            assert_eq!(
                Value::String(String::from("123")),
                Value::Float64(123.0).try_convert_to_string().unwrap()
            );
            assert_eq!(
                Value::String(String::from("123.456")),
                Value::Float64(123.456).try_convert_to_string().unwrap()
            );
        }

        #[test]
        fn from_bool() {
            assert_eq!(
                Value::String(String::from("true")),
                Value::Bool(true).try_convert_to_string().unwrap()
            );
            assert_eq!(
                Value::String(String::from("false")),
                Value::Bool(false).try_convert_to_string().unwrap()
            );
        }

        #[test]
        fn from_decimal() {
            assert_eq!(
                Value::String(String::from("123.456")),
                Value::Decimal(Decimal::new(123456, 3))
                    .try_convert_to_string()
                    .unwrap()
            );
        }

        #[test]
        fn from_naive_date() {
            assert_eq!(
                Value::String(String::from("2022-12-31")),
                Value::NaiveDate(NaiveDate::from_ymd(2022, 12, 31))
                    .try_convert_to_string()
                    .unwrap()
            );
        }

        #[test]
        fn from_naive_date_time() {
            assert_eq!(
                Value::String(String::from("2022-12-31 10:00:00")),
                Value::NaiveDateTime(NaiveDate::from_ymd(2022, 12, 31).and_hms(10, 0, 0))
                    .try_convert_to_string()
                    .unwrap()
            );
        }

        #[test]
        fn from_naive_date_time_with_millies() {
            assert_eq!(
                Value::String(String::from("2022-12-31 10:00:00.100")),
                Value::NaiveDateTime(
                    NaiveDate::from_ymd(2022, 12, 31).and_hms_milli(10, 0, 0, 100)
                )
                .try_convert_to_string()
                .unwrap()
            );
        }

        #[test]
        fn from_naive_date_time_with_millies_empty() {
            assert_eq!(
                Value::String(String::from("2022-12-31 10:00:00.000")),
                Value::NaiveDateTime(
                    NaiveDate::from_ymd(2022, 12, 31).and_hms_milli(10, 0, 0, 000)
                )
                .try_convert_to_string()
                .unwrap()
            );
        }

        #[test]
        fn from_date_time() {
            assert_eq!(
                Value::String(String::from("2022-12-31 10:00:00.100 +02:00")),
                Value::DateTime(
                    FixedOffset::east(2 * 3600)
                        .ymd(2022, 12, 31)
                        .and_hms_milli(10, 0, 0, 100)
                )
                .try_convert_to_string()
                .unwrap()
            );
        }

        #[test]
        fn from_date_time_empty_millies() {
            assert_eq!(
                Value::String(String::from("2022-12-31 10:00:00.000 +02:00")),
                Value::DateTime(
                    FixedOffset::east(2 * 3600)
                        .ymd(2022, 12, 31)
                        .and_hms_milli(10, 0, 0, 0)
                )
                .try_convert_to_string()
                .unwrap()
            );
        }
    }

    mod try_convert_to_uint8 {
        use chrono::TimeZone;

        use super::*;

        #[test]
        fn from_char() {
            assert_eq!(
                Value::UInt8(8),
                Value::Char('8').try_convert_to_uint8().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_char_err_no_num() {
            Value::Char('a').try_convert_to_uint8().unwrap();
        }

        #[test]
        fn from_string() {
            assert_eq!(
                Value::UInt8(8),
                Value::String(String::from("8"))
                    .try_convert_to_uint8()
                    .unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_string_err_no_num() {
            Value::String(String::from("abc"))
                .try_convert_to_uint8()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_string_err_neg_num() {
            Value::String(String::from("-1"))
                .try_convert_to_uint8()
                .unwrap();
        }

        #[test]
        fn from_uint8() {
            assert_eq!(
                Value::UInt8(8),
                Value::UInt8(8).try_convert_to_uint8().unwrap()
            );
        }

        #[test]
        fn from_uint16() {
            assert_eq!(
                Value::UInt8(8),
                Value::UInt16(8).try_convert_to_uint8().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_uint16_err_val_too_big() {
            Value::UInt16(u16::MAX).try_convert_to_uint8().unwrap();
        }

        #[test]
        fn from_uint32() {
            assert_eq!(
                Value::UInt8(8),
                Value::UInt32(8).try_convert_to_uint8().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_uint32_err_val_too_big() {
            Value::UInt32(u32::MAX).try_convert_to_uint8().unwrap();
        }

        #[test]
        fn from_uint64() {
            assert_eq!(
                Value::UInt8(8),
                Value::UInt64(8).try_convert_to_uint8().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_uint64_err_val_too_big() {
            Value::UInt64(u64::MAX).try_convert_to_uint8().unwrap();
        }

        #[test]
        fn from_uint128() {
            assert_eq!(
                Value::UInt8(8),
                Value::UInt128(8).try_convert_to_uint8().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_uint128_err_val_too_big() {
            Value::UInt128(u128::MAX).try_convert_to_uint8().unwrap();
        }

        #[test]
        fn from_int8() {
            assert_eq!(
                Value::UInt8(8),
                Value::Int8(8).try_convert_to_uint8().unwrap()
            );
        }

        // there is no: from_int8_err_val_too_big. A positive i8 will always fit into a u8

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int8_err_val_too_small() {
            Value::Int8(-1).try_convert_to_uint8().unwrap();
        }

        #[test]
        fn from_int16() {
            assert_eq!(
                Value::UInt8(8),
                Value::Int16(8).try_convert_to_uint8().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int16_err_val_too_big() {
            Value::Int16(i16::MAX).try_convert_to_uint8().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int16_err_val_too_small() {
            Value::Int16(-1).try_convert_to_uint8().unwrap();
        }

        #[test]
        fn from_int32() {
            assert_eq!(
                Value::UInt8(8),
                Value::Int32(8).try_convert_to_uint8().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int32_err_val_too_big() {
            Value::Int32(i32::MAX).try_convert_to_uint8().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int32_err_val_too_small() {
            Value::Int32(-1).try_convert_to_uint8().unwrap();
        }

        #[test]
        fn from_int64() {
            assert_eq!(
                Value::UInt8(8),
                Value::Int64(8).try_convert_to_uint8().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int64_err_val_too_big() {
            Value::Int64(i64::MAX).try_convert_to_uint8().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int64_err_val_too_small() {
            Value::Int64(-1).try_convert_to_uint8().unwrap();
        }

        #[test]
        fn from_int128() {
            assert_eq!(
                Value::UInt8(8),
                Value::Int128(8).try_convert_to_uint8().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int128_err_val_too_big() {
            Value::Int128(i128::MAX).try_convert_to_uint8().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int128_err_val_too_small() {
            Value::Int128(-1).try_convert_to_uint8().unwrap();
        }

        #[test]
        fn from_float32() {
            assert_eq!(
                Value::UInt8(8),
                Value::Float32(8.0).try_convert_to_uint8().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float32_err_val_too_big() {
            Value::Float32(12345678.0).try_convert_to_uint8().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float32_err_val_too_small() {
            Value::Float32(-1.0).try_convert_to_uint8().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float32_err_val_uneven() {
            Value::Float32(1.5).try_convert_to_uint8().unwrap();
        }

        #[test]
        fn from_float64() {
            assert_eq!(
                Value::UInt8(8),
                Value::Float64(8.0).try_convert_to_uint8().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_too_big() {
            Value::Float64(f64::MAX as u128 as f64)
                .try_convert_to_uint8()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_too_small() {
            Value::Float64(-1.0).try_convert_to_uint8().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_uneven() {
            Value::Float64(1.5).try_convert_to_uint8().unwrap();
        }

        #[test]
        fn from_bool() {
            assert_eq!(
                Value::UInt8(1),
                Value::Bool(true).try_convert_to_uint8().unwrap()
            );
            assert_eq!(
                Value::UInt8(0),
                Value::Bool(false).try_convert_to_uint8().unwrap()
            );
        }

        #[test]
        fn from_decimal() {
            assert_eq!(
                Value::UInt8(123),
                Value::Decimal(Decimal::new(123, 0))
                    .try_convert_to_uint8()
                    .unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_decimal_err_val_too_big() {
            Value::Decimal(Decimal::new(123456, 0))
                .try_convert_to_uint8()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_decimal_err_val_too_small() {
            Value::Decimal(Decimal::new(-1, 0))
                .try_convert_to_uint8()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_decimal_err_val_uneven() {
            Value::Decimal(Decimal::new(15, 1))
                .try_convert_to_uint8()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_naive_date() {
            Value::NaiveDate(NaiveDate::from_ymd(2022, 12, 31))
                .try_convert_to_uint8()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_naive_date_time() {
            Value::NaiveDateTime(NaiveDate::from_ymd(2022, 12, 31).and_hms(10, 0, 0))
                .try_convert_to_uint8()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_date_time() {
            Value::DateTime(
                FixedOffset::east(2 * 3600)
                    .ymd(2022, 12, 31)
                    .and_hms_milli(10, 0, 0, 100),
            )
            .try_convert_to_uint8()
            .unwrap();
        }
    }

    mod try_convert_to_uint16 {
        use chrono::TimeZone;

        use super::*;

        #[test]
        fn from_char() {
            assert_eq!(
                Value::UInt16(1),
                Value::Char('1').try_convert_to_uint16().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_char_err_no_num() {
            Value::Char('a').try_convert_to_uint16().unwrap();
        }

        #[test]
        fn from_string() {
            assert_eq!(
                Value::UInt16(16),
                Value::String(String::from("16"))
                    .try_convert_to_uint16()
                    .unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_string_err_no_num() {
            Value::String(String::from("abc"))
                .try_convert_to_uint16()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_string_err_neg_num() {
            Value::String(String::from("-1"))
                .try_convert_to_uint16()
                .unwrap();
        }

        #[test]
        fn from_uint8() {
            assert_eq!(
                Value::UInt16(16),
                Value::UInt8(16).try_convert_to_uint16().unwrap()
            );
        }

        #[test]
        fn from_uint16() {
            assert_eq!(
                Value::UInt16(16),
                Value::UInt16(16).try_convert_to_uint16().unwrap()
            );
        }

        #[test]
        fn from_uint32() {
            assert_eq!(
                Value::UInt16(16),
                Value::UInt32(16).try_convert_to_uint16().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_uint32_err_val_too_big() {
            Value::UInt32(u32::MAX).try_convert_to_uint16().unwrap();
        }

        #[test]
        fn from_uint64() {
            assert_eq!(
                Value::UInt16(16),
                Value::UInt64(16).try_convert_to_uint16().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_uint64_err_val_too_big() {
            Value::UInt64(u64::MAX).try_convert_to_uint16().unwrap();
        }

        #[test]
        fn from_uint128() {
            assert_eq!(
                Value::UInt16(16),
                Value::UInt128(16).try_convert_to_uint16().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_uint128_err_val_too_big() {
            Value::UInt128(u128::MAX).try_convert_to_uint16().unwrap();
        }

        #[test]
        fn from_int8() {
            assert_eq!(
                Value::UInt16(16),
                Value::Int8(16).try_convert_to_uint16().unwrap()
            );
        }

        // there is no: from_int8_err_val_too_big. A positive i8 will always fit into a u16

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int8_err_val_too_small() {
            Value::Int8(-1).try_convert_to_uint16().unwrap();
        }

        #[test]
        fn from_int16() {
            assert_eq!(
                Value::UInt16(16),
                Value::Int16(16).try_convert_to_uint16().unwrap()
            );
        }

        // there is no: from_int16_err_val_too_big. A positive i16 will always fit into a u16

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int16_err_val_too_small() {
            Value::Int16(-1).try_convert_to_uint16().unwrap();
        }

        #[test]
        fn from_int32() {
            assert_eq!(
                Value::UInt16(16),
                Value::Int32(16).try_convert_to_uint16().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int32_err_val_too_big() {
            Value::Int32(i32::MAX).try_convert_to_uint16().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int32_err_val_too_small() {
            Value::Int32(-1).try_convert_to_uint16().unwrap();
        }

        #[test]
        fn from_int64() {
            assert_eq!(
                Value::UInt16(16),
                Value::Int64(16).try_convert_to_uint16().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int64_err_val_too_big() {
            Value::Int64(i64::MAX).try_convert_to_uint16().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int64_err_val_too_small() {
            Value::Int64(-1).try_convert_to_uint16().unwrap();
        }

        #[test]
        fn from_int128() {
            assert_eq!(
                Value::UInt16(16),
                Value::Int128(16).try_convert_to_uint16().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int128_err_val_too_big() {
            Value::Int128(i128::MAX).try_convert_to_uint16().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int128_err_val_too_small() {
            Value::Int128(-1).try_convert_to_uint16().unwrap();
        }

        #[test]
        fn from_float32() {
            assert_eq!(
                Value::UInt16(16),
                Value::Float32(16.0).try_convert_to_uint16().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float32_err_val_too_big() {
            Value::Float32(12345678.0).try_convert_to_uint16().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float32_err_val_too_small() {
            Value::Float32(-1.0).try_convert_to_uint16().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float32_err_val_uneven() {
            Value::Float32(1.5).try_convert_to_uint16().unwrap();
        }

        #[test]
        fn from_float64() {
            assert_eq!(
                Value::UInt16(16),
                Value::Float64(16.0).try_convert_to_uint16().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_too_big() {
            Value::Float64(123456789.0).try_convert_to_uint16().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_too_small() {
            Value::Float64(-1.0).try_convert_to_uint16().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_uneven() {
            Value::Float64(1.5).try_convert_to_uint16().unwrap();
        }

        #[test]
        fn from_bool() {
            assert_eq!(
                Value::UInt16(1),
                Value::Bool(true).try_convert_to_uint16().unwrap()
            );
            assert_eq!(
                Value::UInt16(0),
                Value::Bool(false).try_convert_to_uint16().unwrap()
            );
        }

        #[test]
        fn from_decimal() {
            assert_eq!(
                Value::UInt16(123),
                Value::Decimal(Decimal::new(123, 0))
                    .try_convert_to_uint16()
                    .unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_decimal_err_val_too_big() {
            Value::Decimal(Decimal::new(123456789, 0))
                .try_convert_to_uint16()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_decimal_err_val_too_small() {
            Value::Decimal(Decimal::new(-1, 0))
                .try_convert_to_uint16()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_decimal_err_val_uneven() {
            Value::Decimal(Decimal::new(15, 1))
                .try_convert_to_uint16()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_naive_date() {
            Value::NaiveDate(NaiveDate::from_ymd(2022, 12, 31))
                .try_convert_to_uint16()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_naive_date_time() {
            Value::NaiveDateTime(NaiveDate::from_ymd(2022, 12, 31).and_hms(10, 0, 0))
                .try_convert_to_uint16()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_date_time() {
            Value::DateTime(
                FixedOffset::east(2 * 3600)
                    .ymd(2022, 12, 31)
                    .and_hms_milli(10, 0, 0, 100),
            )
            .try_convert_to_uint16()
            .unwrap();
        }
    }

    mod try_convert_to_uint32 {
        use chrono::TimeZone;

        use super::*;

        #[test]
        fn from_char() {
            assert_eq!(
                Value::UInt32(1),
                Value::Char('1').try_convert_to_uint32().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_char_err_no_num() {
            Value::Char('a').try_convert_to_uint32().unwrap();
        }

        #[test]
        fn from_string() {
            assert_eq!(
                Value::UInt32(32),
                Value::String(String::from("32"))
                    .try_convert_to_uint32()
                    .unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_string_err_no_num() {
            Value::String(String::from("abc"))
                .try_convert_to_uint32()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_string_err_neg_num() {
            Value::String(String::from("-1"))
                .try_convert_to_uint32()
                .unwrap();
        }

        #[test]
        fn from_uint8() {
            assert_eq!(
                Value::UInt32(32),
                Value::UInt8(32).try_convert_to_uint32().unwrap()
            );
        }

        #[test]
        fn from_uint16() {
            assert_eq!(
                Value::UInt32(32),
                Value::UInt16(32).try_convert_to_uint32().unwrap()
            );
        }

        #[test]
        fn from_uint32() {
            assert_eq!(
                Value::UInt32(32),
                Value::UInt32(32).try_convert_to_uint32().unwrap()
            );
        }

        #[test]
        fn from_uint64() {
            assert_eq!(
                Value::UInt32(32),
                Value::UInt64(32).try_convert_to_uint32().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_uint64_err_val_too_big() {
            Value::UInt64(u64::MAX).try_convert_to_uint32().unwrap();
        }

        #[test]
        fn from_uint128() {
            assert_eq!(
                Value::UInt32(32),
                Value::UInt128(32).try_convert_to_uint32().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_uint128_err_val_too_big() {
            Value::UInt128(u128::MAX).try_convert_to_uint32().unwrap();
        }

        #[test]
        fn from_int8() {
            assert_eq!(
                Value::UInt32(32),
                Value::Int8(32).try_convert_to_uint32().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int8_err_val_too_small() {
            Value::Int8(-1).try_convert_to_uint32().unwrap();
        }

        #[test]
        fn from_int16() {
            assert_eq!(
                Value::UInt32(32),
                Value::Int16(32).try_convert_to_uint32().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int16_err_val_too_small() {
            Value::Int16(-1).try_convert_to_uint32().unwrap();
        }

        #[test]
        fn from_int32() {
            assert_eq!(
                Value::UInt32(32),
                Value::Int32(32).try_convert_to_uint32().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int32_err_val_too_small() {
            Value::Int32(-1).try_convert_to_uint32().unwrap();
        }

        #[test]
        fn from_int64() {
            assert_eq!(
                Value::UInt32(32),
                Value::Int64(32).try_convert_to_uint32().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int64_err_val_too_big() {
            Value::Int64(i64::MAX).try_convert_to_uint32().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int64_err_val_too_small() {
            Value::Int64(-1).try_convert_to_uint32().unwrap();
        }

        #[test]
        fn from_int128() {
            assert_eq!(
                Value::UInt32(32),
                Value::Int128(32).try_convert_to_uint32().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int128_err_val_too_big() {
            Value::Int128(i128::MAX).try_convert_to_uint32().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int128_err_val_too_small() {
            Value::Int128(-1).try_convert_to_uint32().unwrap();
        }

        #[test]
        fn from_float32() {
            assert_eq!(
                Value::UInt32(32),
                Value::Float32(32.0).try_convert_to_uint32().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float32_err_val_too_big() {
            Value::Float32((f32::MAX as u64) as f32)
                .try_convert_to_uint32()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float32_err_val_too_small() {
            Value::Float32(-1.0).try_convert_to_uint32().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float32_err_val_uneven() {
            Value::Float32(1.5).try_convert_to_uint32().unwrap();
        }

        #[test]
        fn from_float64() {
            assert_eq!(
                Value::UInt32(32),
                Value::Float64(32.0).try_convert_to_uint32().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_too_big() {
            Value::Float64((f64::MAX as u128) as f64)
                .try_convert_to_uint32()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_too_small() {
            Value::Float64(-1.0).try_convert_to_uint32().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_uneven() {
            Value::Float64(1.5).try_convert_to_uint32().unwrap();
        }

        #[test]
        fn from_bool() {
            assert_eq!(
                Value::UInt32(1),
                Value::Bool(true).try_convert_to_uint32().unwrap()
            );
            assert_eq!(
                Value::UInt32(0),
                Value::Bool(false).try_convert_to_uint32().unwrap()
            );
        }

        #[test]
        fn from_decimal() {
            assert_eq!(
                Value::UInt32(123),
                Value::Decimal(Decimal::new(123, 0))
                    .try_convert_to_uint32()
                    .unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_decimal_err_val_too_big() {
            Value::Decimal(Decimal::new(123456789123456789, 0))
                .try_convert_to_uint32()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_decimal_err_val_too_small() {
            Value::Decimal(Decimal::new(-1, 0))
                .try_convert_to_uint32()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_decimal_err_val_uneven() {
            Value::Decimal(Decimal::new(15, 1))
                .try_convert_to_uint32()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_naive_date() {
            Value::NaiveDate(NaiveDate::from_ymd(2022, 12, 31))
                .try_convert_to_uint32()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_naive_date_time() {
            Value::NaiveDateTime(NaiveDate::from_ymd(2022, 12, 31).and_hms(10, 0, 0))
                .try_convert_to_uint32()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_date_time() {
            Value::DateTime(
                FixedOffset::east(2 * 3600)
                    .ymd(2022, 12, 31)
                    .and_hms_milli(10, 0, 0, 100),
            )
            .try_convert_to_uint32()
            .unwrap();
        }
    }

    mod try_convert_to_uint64 {
        use chrono::TimeZone;

        use super::*;

        #[test]
        fn from_char() {
            assert_eq!(
                Value::UInt64(1),
                Value::Char('1').try_convert_to_uint64().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_char_err_no_num() {
            Value::Char('a').try_convert_to_uint64().unwrap();
        }

        #[test]
        fn from_string() {
            assert_eq!(
                Value::UInt64(64),
                Value::String(String::from("64"))
                    .try_convert_to_uint64()
                    .unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_string_err_no_num() {
            Value::String(String::from("abc"))
                .try_convert_to_uint64()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_string_err_neg_num() {
            Value::String(String::from("-1"))
                .try_convert_to_uint64()
                .unwrap();
        }

        #[test]
        fn from_uint8() {
            assert_eq!(
                Value::UInt64(64),
                Value::UInt8(64).try_convert_to_uint64().unwrap()
            );
        }

        #[test]
        fn from_uint16() {
            assert_eq!(
                Value::UInt64(64),
                Value::UInt16(64).try_convert_to_uint64().unwrap()
            );
        }

        #[test]
        fn from_uint32() {
            assert_eq!(
                Value::UInt64(64),
                Value::UInt32(64).try_convert_to_uint64().unwrap()
            );
        }

        #[test]
        fn from_uint64() {
            assert_eq!(
                Value::UInt64(64),
                Value::UInt64(64).try_convert_to_uint64().unwrap()
            );
        }

        #[test]
        fn from_uint128() {
            assert_eq!(
                Value::UInt64(64),
                Value::UInt128(64).try_convert_to_uint64().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_uint128_err_val_too_big() {
            Value::UInt128(u128::MAX).try_convert_to_uint64().unwrap();
        }

        #[test]
        fn from_int8() {
            assert_eq!(
                Value::UInt64(64),
                Value::Int8(64).try_convert_to_uint64().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int8_err_val_too_small() {
            Value::Int8(-1).try_convert_to_uint64().unwrap();
        }

        #[test]
        fn from_int16() {
            assert_eq!(
                Value::UInt64(64),
                Value::Int16(64).try_convert_to_uint64().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int16_err_val_too_small() {
            Value::Int16(-1).try_convert_to_uint64().unwrap();
        }

        #[test]
        fn from_int32() {
            assert_eq!(
                Value::UInt64(64),
                Value::Int32(64).try_convert_to_uint64().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int32_err_val_too_small() {
            Value::Int32(-1).try_convert_to_uint64().unwrap();
        }

        #[test]
        fn from_int64() {
            assert_eq!(
                Value::UInt64(64),
                Value::Int64(64).try_convert_to_uint64().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int64_err_val_too_small() {
            Value::Int64(-1).try_convert_to_uint64().unwrap();
        }

        #[test]
        fn from_int128() {
            assert_eq!(
                Value::UInt64(64),
                Value::Int128(64).try_convert_to_uint64().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int128_err_val_too_big() {
            Value::Int128(i128::MAX).try_convert_to_uint64().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int128_err_val_too_small() {
            Value::Int128(-1).try_convert_to_uint64().unwrap();
        }

        #[test]
        fn from_float32() {
            assert_eq!(
                Value::UInt64(64),
                Value::Float32(64.0).try_convert_to_uint64().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float32_err_val_too_small() {
            Value::Float32(-1.0).try_convert_to_uint64().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float32_err_val_uneven() {
            Value::Float32(1.5).try_convert_to_uint64().unwrap();
        }

        #[test]
        fn from_float64() {
            assert_eq!(
                Value::UInt64(64),
                Value::Float64(64.0).try_convert_to_uint64().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_too_big() {
            Value::Float64((f64::MAX as u128) as f64)
                .try_convert_to_uint64()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_too_small() {
            Value::Float64(-1.0).try_convert_to_uint64().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_uneven() {
            Value::Float64(1.5).try_convert_to_uint64().unwrap();
        }

        #[test]
        fn from_bool() {
            assert_eq!(
                Value::UInt64(1),
                Value::Bool(true).try_convert_to_uint64().unwrap()
            );
            assert_eq!(
                Value::UInt64(0),
                Value::Bool(false).try_convert_to_uint64().unwrap()
            );
        }

        #[test]
        fn from_decimal() {
            assert_eq!(
                Value::UInt64(123),
                Value::Decimal(Decimal::new(123, 0))
                    .try_convert_to_uint64()
                    .unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_decimal_err_val_too_small() {
            Value::Decimal(Decimal::new(-1, 0))
                .try_convert_to_uint64()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_decimal_err_val_uneven() {
            Value::Decimal(Decimal::new(15, 1))
                .try_convert_to_uint64()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_naive_date() {
            Value::NaiveDate(NaiveDate::from_ymd(2022, 12, 31))
                .try_convert_to_uint64()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_naive_date_time() {
            Value::NaiveDateTime(NaiveDate::from_ymd(2022, 12, 31).and_hms(10, 0, 0))
                .try_convert_to_uint64()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_date_time() {
            Value::DateTime(
                FixedOffset::east(2 * 3600)
                    .ymd(2022, 12, 31)
                    .and_hms_milli(10, 0, 0, 100),
            )
            .try_convert_to_uint64()
            .unwrap();
        }
    }

    mod try_convert_to_uint128 {
        use chrono::TimeZone;

        use super::*;

        #[test]
        fn from_char() {
            assert_eq!(
                Value::UInt128(1),
                Value::Char('1').try_convert_to_uint128().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_char_err_no_num() {
            Value::Char('a').try_convert_to_uint128().unwrap();
        }

        #[test]
        fn from_string() {
            assert_eq!(
                Value::UInt128(128),
                Value::String(String::from("128"))
                    .try_convert_to_uint128()
                    .unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_string_err_no_num() {
            Value::String(String::from("abc"))
                .try_convert_to_uint128()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_string_err_neg_num() {
            Value::String(String::from("-1"))
                .try_convert_to_uint128()
                .unwrap();
        }

        #[test]
        fn from_uint8() {
            assert_eq!(
                Value::UInt128(128),
                Value::UInt8(128).try_convert_to_uint128().unwrap()
            );
        }

        #[test]
        fn from_uint16() {
            assert_eq!(
                Value::UInt128(128),
                Value::UInt16(128).try_convert_to_uint128().unwrap()
            );
        }

        #[test]
        fn from_uint32() {
            assert_eq!(
                Value::UInt128(128),
                Value::UInt32(128).try_convert_to_uint128().unwrap()
            );
        }

        #[test]
        fn from_uint64() {
            assert_eq!(
                Value::UInt128(128),
                Value::UInt64(128).try_convert_to_uint128().unwrap()
            );
        }

        #[test]
        fn from_uint128() {
            assert_eq!(
                Value::UInt128(128),
                Value::UInt128(128).try_convert_to_uint128().unwrap()
            );
        }

        #[test]
        fn from_int8() {
            assert_eq!(
                Value::UInt128(127),
                Value::Int8(127).try_convert_to_uint128().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int8_err_val_too_small() {
            Value::Int8(-1).try_convert_to_uint128().unwrap();
        }

        #[test]
        fn from_int16() {
            assert_eq!(
                Value::UInt128(128),
                Value::Int16(128).try_convert_to_uint128().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int16_err_val_too_small() {
            Value::Int16(-1).try_convert_to_uint128().unwrap();
        }

        #[test]
        fn from_int32() {
            assert_eq!(
                Value::UInt128(128),
                Value::Int32(128).try_convert_to_uint128().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int32_err_val_too_small() {
            Value::Int32(-1).try_convert_to_uint128().unwrap();
        }

        #[test]
        fn from_int64() {
            assert_eq!(
                Value::UInt128(128),
                Value::Int64(128).try_convert_to_uint128().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int64_err_val_too_small() {
            Value::Int64(-1).try_convert_to_uint128().unwrap();
        }

        #[test]
        fn from_int128() {
            assert_eq!(
                Value::UInt128(128),
                Value::Int128(128).try_convert_to_uint128().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int128_err_val_too_small() {
            Value::Int128(-1).try_convert_to_uint128().unwrap();
        }

        #[test]
        fn from_float32() {
            assert_eq!(
                Value::UInt128(128),
                Value::Float32(128.0).try_convert_to_uint128().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float32_err_val_too_small() {
            Value::Float32(-1.0).try_convert_to_uint128().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float32_err_val_uneven() {
            Value::Float32(1.5).try_convert_to_uint128().unwrap();
        }

        #[test]
        fn from_float64() {
            assert_eq!(
                Value::UInt128(128),
                Value::Float64(128.0).try_convert_to_uint128().unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_too_big() {
            Value::Float64(f64::MAX.round())
                .try_convert_to_uint128()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_too_small() {
            Value::Float64(-1.0).try_convert_to_uint128().unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_uneven() {
            Value::Float64(1.5).try_convert_to_uint128().unwrap();
        }

        #[test]
        fn from_bool() {
            assert_eq!(
                Value::UInt128(1),
                Value::Bool(true).try_convert_to_uint128().unwrap()
            );
            assert_eq!(
                Value::UInt128(0),
                Value::Bool(false).try_convert_to_uint128().unwrap()
            );
        }

        #[test]
        fn from_decimal() {
            assert_eq!(
                Value::UInt128(128),
                Value::Decimal(Decimal::new(128, 0))
                    .try_convert_to_uint128()
                    .unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_decimal_err_val_too_small() {
            Value::Decimal(Decimal::new(-1, 0))
                .try_convert_to_uint128()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_decimal_err_val_uneven() {
            Value::Decimal(Decimal::new(15, 1))
                .try_convert_to_uint128()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_naive_date() {
            Value::NaiveDate(NaiveDate::from_ymd(2022, 12, 31))
                .try_convert_to_uint128()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_naive_date_time() {
            Value::NaiveDateTime(NaiveDate::from_ymd(2022, 12, 31).and_hms(10, 0, 0))
                .try_convert_to_uint128()
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_date_time() {
            Value::DateTime(
                FixedOffset::east(2 * 3600)
                    .ymd(2022, 12, 31)
                    .and_hms_milli(10, 0, 0, 100),
            )
            .try_convert_to_uint128()
            .unwrap();
        }
    }

    // mod try_convert_to_int8 {
    //     use chrono::TimeZone;

    //     use super::*;

    //     #[test]
    //     fn from_char() {
    //         assert_eq!(
    //             Value::Int8(8),
    //             Value::Char('8').try_convert_to_int8().unwrap()
    //         );
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_char_err_no_num() {
    //         Value::Char('a').try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     fn from_string() {
    //         assert_eq!(
    //             Value::Int8(8),
    //             Value::String(String::from("8"))
    //                 .try_convert_to_int8()
    //                 .unwrap()
    //         );
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_string_err_no_num() {
    //         Value::String(String::from("abc"))
    //             .try_convert_to_int8()
    //             .unwrap();
    //     }

    //     #[test]
    //     fn from_uint8() {
    //         assert_eq!(
    //             Value::Int8(8),
    //             Value::Int8(8).try_convert_to_int8().unwrap()
    //         );
    //     }

    //     #[test]
    //     fn from_uint16() {
    //         assert_eq!(
    //             Value::Int8(8),
    //             Value::UInt16(8).try_convert_to_int8().unwrap()
    //         );
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_uint16_err_val_too_big() {
    //         Value::UInt16(u16::MAX).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     fn from_uint32() {
    //         assert_eq!(
    //             Value::Int8(8),
    //             Value::UInt32(8).try_convert_to_int8().unwrap()
    //         );
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_uint32_err_val_too_big() {
    //         Value::UInt32(u32::MAX).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     fn from_uint64() {
    //         assert_eq!(
    //             Value::Int8(8),
    //             Value::UInt64(8).try_convert_to_int8().unwrap()
    //         );
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_uint64_err_val_too_big() {
    //         Value::UInt64(u64::MAX).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     fn from_uint128() {
    //         assert_eq!(
    //             Value::Int8(8),
    //             Value::UInt128(8).try_convert_to_int8().unwrap()
    //         );
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_uint128_err_val_too_big() {
    //         Value::UInt128(u128::MAX).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     fn from_int8() {
    //         assert_eq!(
    //             Value::Int8(8),
    //             Value::Int8(8).try_convert_to_int8().unwrap()
    //         );
    //     }

    //     // there is no: from_int8_err_val_too_big. A positive i8 will always fit into a u8

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_int8_err_val_too_small() {
    //         Value::Int8(-1).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     fn from_int16() {
    //         assert_eq!(
    //             Value::Int8(8),
    //             Value::Int16(8).try_convert_to_int8().unwrap()
    //         );
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_int16_err_val_too_big() {
    //         Value::Int16(i16::MAX).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_int16_err_val_too_small() {
    //         Value::Int16(-1).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     fn from_int32() {
    //         assert_eq!(
    //             Value::Int8(8),
    //             Value::Int32(8).try_convert_to_int8().unwrap()
    //         );
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_int32_err_val_too_big() {
    //         Value::Int32(i32::MAX).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_int32_err_val_too_small() {
    //         Value::Int32(-1).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     fn from_int64() {
    //         assert_eq!(
    //             Value::Int8(8),
    //             Value::Int64(8).try_convert_to_int8().unwrap()
    //         );
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_int64_err_val_too_big() {
    //         Value::Int64(i64::MAX).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_int64_err_val_too_small() {
    //         Value::Int64(-1).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     fn from_int128() {
    //         assert_eq!(
    //             Value::Int8(8),
    //             Value::Int128(8).try_convert_to_int8().unwrap()
    //         );
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_int128_err_val_too_big() {
    //         Value::Int128(i128::MAX).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_int128_err_val_too_small() {
    //         Value::Int128(-1).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     fn from_float32() {
    //         assert_eq!(
    //             Value::Int8(8),
    //             Value::Float32(8.0).try_convert_to_int8().unwrap()
    //         );
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_float32_err_val_too_big() {
    //         Value::Float32(12345678.0).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_float32_err_val_too_small() {
    //         Value::Float32(-1.0).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_float32_err_val_uneven() {
    //         Value::Float32(1.5).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     fn from_float64() {
    //         assert_eq!(
    //             Value::Int8(8),
    //             Value::Float64(8.0).try_convert_to_int8().unwrap()
    //         );
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_float64_err_val_too_big() {
    //         Value::Float64(f64::MAX as u128 as f64)
    //             .try_convert_to_int8()
    //             .unwrap();
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_float64_err_val_too_small() {
    //         Value::Float64(-1.0).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_float64_err_val_uneven() {
    //         Value::Float64(1.5).try_convert_to_int8().unwrap();
    //     }

    //     #[test]
    //     fn from_bool() {
    //         assert_eq!(
    //             Value::Int8(1),
    //             Value::Bool(true).try_convert_to_int8().unwrap()
    //         );
    //         assert_eq!(
    //             Value::Int8(0),
    //             Value::Bool(false).try_convert_to_int8().unwrap()
    //         );
    //     }

    //     #[test]
    //     fn from_decimal() {
    //         assert_eq!(
    //             Value::Int8(123),
    //             Value::Decimal(Decimal::new(123, 0))
    //                 .try_convert_to_int8()
    //                 .unwrap()
    //         );
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_decimal_err_val_too_big() {
    //         Value::Decimal(Decimal::new(123456, 0))
    //             .try_convert_to_int8()
    //             .unwrap();
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_decimal_err_val_too_small() {
    //         Value::Decimal(Decimal::new(-1, 0))
    //             .try_convert_to_int8()
    //             .unwrap();
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_decimal_err_val_uneven() {
    //         Value::Decimal(Decimal::new(15, 1))
    //             .try_convert_to_int8()
    //             .unwrap();
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_naive_date() {
    //         Value::NaiveDate(NaiveDate::from_ymd(2022, 12, 31))
    //             .try_convert_to_int8()
    //             .unwrap();
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_naive_date_time() {
    //         Value::NaiveDateTime(NaiveDate::from_ymd(2022, 12, 31).and_hms(10, 0, 0))
    //             .try_convert_to_int8()
    //             .unwrap();
    //     }

    //     #[test]
    //     #[should_panic(expected = "Conversion(NotRepresentableAs")]
    //     fn from_date_time() {
    //         Value::DateTime(
    //             FixedOffset::east(2 * 3600)
    //                 .ymd(2022, 12, 31)
    //                 .and_hms_milli(10, 0, 0, 100),
    //         )
    //         .try_convert_to_int8()
    //         .unwrap();
    //     }
    // }
}
