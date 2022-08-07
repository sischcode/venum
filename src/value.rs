use std::convert::From;
use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use strum_macros::Display; // used to generate names for the enum variants. Used only for error messages (as of now).

use crate::{
    errors_result::{ConversionError, ParseError, Result, VenumError},
    value_type::ValueType,
};

const VAL_ENUM_NAME: &str = "Value::";
const ENUM_VAR_ND: &str = "NaiveDate";
const ENUM_VAR_NDT: &str = "NaiveDateTime";
const ENUM_VAR_DT: &str = "DateTime";

#[derive(Display, Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    None,
    Char(char),
    String(String),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    Float32(f32),
    Float64(f64),
    Bool(bool),
    Decimal(Decimal),
    NaiveDate(NaiveDate),
    NaiveDateTime(NaiveDateTime),
    DateTime(DateTime<FixedOffset>),
}

macro_rules! from_type_string {
    ($fn_name:ident, $enum_type:ident, $for_type:ty) => {
        pub fn $fn_name(v: &str) -> Result<Value> {
            // We don't do something like:
            // if v.is_empty() {
            //     return Ok(Value::None);
            // }
            // here, with the reasoning that this should rather fail than to
            // magically give back a Value::None

            let temp = v.parse::<$for_type>().map_err(|e| {
                VenumError::Parsing(ParseError::ValueFromStringFailed {
                    src_value: String::from(v),
                    target_type: format!("{}{}", VAL_ENUM_NAME, stringify!($enum_type)),
                    details: Some(format!("{}", e)),
                })
            })?;
            Ok(Value::$enum_type(temp))
        }
    };
}

macro_rules! type_defaults {
    ($fn_name:ident, $enum_type:ident, $type:ty) => {
        pub fn $fn_name() -> Value {
            Value::$enum_type(<$type>::default())
        }
    };
}

macro_rules! is_type {
    ($fn_name:ident, $enum_type:ident) => {
        pub fn $fn_name(&self) -> bool {
            match self {
                Value::$enum_type(_) => true,
                _ => false,
            }
        }
    };
}

impl Value {
    type_defaults!(char_default, Char, char);
    type_defaults!(string_default, String, String);
    type_defaults!(int8_default, Int8, i8);
    type_defaults!(int16_default, Int16, i16);
    type_defaults!(int32_default, Int32, i32);
    type_defaults!(int64_default, Int64, i64);
    type_defaults!(int128_default, Int128, i128);
    type_defaults!(uint8_default, UInt8, u8);
    type_defaults!(uint16_default, UInt16, u16);
    type_defaults!(uint32_default, UInt32, u32);
    type_defaults!(uint64_default, UInt64, u64);
    type_defaults!(uint128_default, UInt128, u128);
    type_defaults!(float32_default, Float32, f32);
    type_defaults!(float64_default, Float64, f64);
    type_defaults!(bool_default, Bool, bool);
    type_defaults!(decimal_default, Decimal, Decimal);

    /// Default is: 1970-01-01
    pub fn naive_date_default() -> Value {
        Value::NaiveDate(NaiveDate::from_ymd(1970, 1, 1))
    }

    /// Default is: 1970-01-01 00:00:00
    pub fn naive_date_time_default() -> Value {
        Value::NaiveDateTime(NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0))
    }

    /// Default is: 1970-01-01 00:00:00 +00:00 (formatted as: 1970-01-01T00:00:00+00:00)
    pub fn date_time_default() -> Value {
        Value::DateTime(
            FixedOffset::east(0) // east = +; west = -
                .ymd(1970, 1, 1)
                .and_hms_milli(0, 0, 0, 0),
        )
    }

    from_type_string!(parse_char_from_str, Char, char);
    from_type_string!(parse_int8_from_str, Int8, i8);
    from_type_string!(parse_int16_from_str, Int16, i16);
    from_type_string!(parse_int32_from_str, Int32, i32);
    from_type_string!(parse_int64_from_str, Int64, i64);
    from_type_string!(parse_int128_from_str, Int128, i128);
    from_type_string!(parse_uint8_from_str, UInt8, u8);
    from_type_string!(parse_uint16_from_str, UInt16, u16);
    from_type_string!(parse_uint32_from_str, UInt32, u32);
    from_type_string!(parse_uint64_from_str, UInt64, u64);
    from_type_string!(parse_uint128_from_str, UInt128, u128);
    from_type_string!(parse_bool_from_str, Bool, bool);

    pub fn parse_float32_from_str_allow_inf_allow_nan(v: &str) -> Result<Value> {
        let temp: f32 = f32::from_str(v).map_err(|e| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, stringify!($enum_type)),
                details: Some(format!("{}", e)),
            })
        })?;
        Ok(Value::Float32(temp))
    }

    pub fn parse_float32_from_str(v: &str) -> Result<Value> {
        let temp: f32 = f32::from_str(v).map_err(|e| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, stringify!($enum_type)),
                details: Some(format!("{}", e)),
            })
        })?;
        if temp.is_finite() {
            Ok(Value::Float32(temp))
        } else {
            Err(VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, stringify!($enum_type)),
                details: Some(String::from(
                    "value is not finite. I.e. either 'inf', '-inf', '+infinity' or 'NaN'",
                )),
            }))
        }
    }

    pub fn parse_float64_from_str_allow_inf_allow_nan(v: &str) -> Result<Value> {
        let temp: f64 = f64::from_str(v).map_err(|e| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, stringify!($enum_type)),
                details: Some(format!("{}", e)),
            })
        })?;
        Ok(Value::Float64(temp))
    }

    pub fn parse_float64_from_str(v: &str) -> Result<Value> {
        let temp: f64 = f64::from_str(v).map_err(|e| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, stringify!($enum_type)),
                details: Some(format!("{}", e)),
            })
        })?;
        if temp.is_finite() {
            Ok(Value::Float64(temp))
        } else {
            Err(VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, stringify!($enum_type)),
                details: Some(String::from(
                    "value is not finite. I.e. either 'inf', '-inf', '+infinity' or 'NaN'",
                )),
            }))
        }
    }

    pub fn parse_decimal_from_str(v: &str) -> Result<Value> {
        // Is this really a good idea?
        if v.is_empty() {
            return Ok(Value::None);
        }
        let temp = Decimal::from_str_exact(v).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, "Decimal"),
                details: Some(format!("Original error: {oe}")),
            })
        })?;
        Ok(Value::Decimal(temp))
    }
    pub fn parse_naive_date_from_str(v: &str, chrono_pattern: &str) -> Result<Value> {
        // e.g pattern "%Y-%m-%d" to parse "2015-09-05"
        // Is this really a good idea?
        if v.is_empty() {
            return Ok(Value::None);
        }
        let temp = NaiveDate::parse_from_str(v, chrono_pattern).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, ENUM_VAR_ND),
                details: Some(format!(
                    "Chrono pattern: {chrono_pattern}. Original error: {oe}"
                )),
            })
        })?;
        Ok(Value::NaiveDate(temp))
    }
    pub fn parse_naive_date_from_str_iso8601_ymd(v: &str) -> Result<Value> {
        // e.g pattern "%F" (which is "%Y-%m-%d") to parse "2015-09-05"
        // Is this really a good idea?
        if v.is_empty() {
            return Ok(Value::None);
        }
        let chrono_pattern = "%F";
        let temp = NaiveDate::parse_from_str(v, chrono_pattern).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, ENUM_VAR_ND),
                details: Some(format!(
                    "Chrono pattern: {chrono_pattern}. Original error: {oe}"
                )),
            })
        })?;
        Ok(Value::NaiveDate(temp))
    }
    pub fn parse_naive_date_time_from_str(v: &str, chrono_pattern: &str) -> Result<Value> {
        // e.g pattern "%F %T" (which is "%Y-%m-%d %H:%M:%S") to parse "2015-09-05 23:56:04"
        // Is this really a good idea?
        if v.is_empty() {
            return Ok(Value::None);
        }
        let temp = NaiveDateTime::parse_from_str(v, chrono_pattern).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, ENUM_VAR_NDT),
                details: Some(format!(
                    "Chrono pattern: {chrono_pattern}. Original error: {oe}"
                )),
            })
        })?;
        Ok(Value::NaiveDateTime(temp))
    }
    pub fn parse_naive_date_time_from_str_iso8601_ymd_hms(v: &str) -> Result<Value> {
        // e.g for parsing "2015-09-05T23:56:04"
        if v.is_empty() {
            return Ok(Value::None);
        }
        let chrono_pattern = "%Y-%m-%dT%H:%M:%S"; // same as: "%FT%T";
        let temp = NaiveDateTime::parse_from_str(v, chrono_pattern).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, ENUM_VAR_NDT),
                details: Some(format!(
                    "Chrono pattern: {chrono_pattern}. Original error: {oe}"
                )),
            })
        })?;
        Ok(Value::NaiveDateTime(temp))
    }
    pub fn parse_naive_date_time_from_str_iso8601_ymd_hms_millies(v: &str) -> Result<Value> {
        // e.g for parsing "2015-09-05T23:56:04.100"
        if v.is_empty() {
            return Ok(Value::None);
        }
        let chrono_pattern = "%Y-%m-%dT%H:%M:%S%.3f";
        let temp = NaiveDateTime::parse_from_str(v, chrono_pattern).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, ENUM_VAR_NDT),
                details: Some(format!(
                    "Chrono pattern: {chrono_pattern}. Original error: {oe}"
                )),
            })
        })?;
        Ok(Value::NaiveDateTime(temp))
    }
    pub fn parse_date_time_from_str(v: &str, chrono_pattern: &str) -> Result<Value> {
        if v.is_empty() {
            return Ok(Value::None);
        }
        let temp = DateTime::parse_from_str(v, chrono_pattern).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, ENUM_VAR_DT),
                details: Some(format!(
                    "Chrono pattern: {chrono_pattern}. Original error: {oe}"
                )),
            })
        })?;
        Ok(Value::DateTime(temp))
    }
    pub fn parse_date_time_from_str_rfc2822(v: &str) -> Result<Value> {
        if v.is_empty() {
            return Ok(Value::None);
        }
        // e.g date as: "Tue, 1 Jul 2003 10:52:37 +0200"
        let temp = DateTime::parse_from_rfc2822(v).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, ENUM_VAR_DT),
                details: Some(format!("Original error: {oe}")),
            })
        })?;
        Ok(Value::DateTime(temp))
    }
    pub fn parse_date_time_from_str_rfc3339(v: &str) -> Result<Value> {
        if v.is_empty() {
            return Ok(Value::None);
        }
        // e.g date as: "1996-12-19T16:39:57-08:00"
        let temp = DateTime::parse_from_rfc3339(v).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, ENUM_VAR_DT),
                details: Some(format!("Original error: {oe}")),
            })
        })?;
        Ok(Value::DateTime(temp))
    }

    // For all decimal_from_XXX we assume it is better to error, instead of
    pub fn decimal_from_i8(v: i8) -> Value {
        Value::Decimal(Decimal::from(v))
    }
    pub fn decimal_from_i16(v: i16) -> Value {
        Value::Decimal(Decimal::from(v))
    }
    pub fn decimal_from_i32(v: i32) -> Value {
        Value::Decimal(Decimal::from(v))
    }
    pub fn decimal_from_i64(v: i64) -> Value {
        Value::Decimal(Decimal::from(v))
    }
    pub fn decimal_from_i128(v: i128) -> Result<Value> {
        let tmp = Decimal::from_i128(v).ok_or_else(|| {
            VenumError::Conversion(ConversionError::NotRepresentableAsDecimal {
                src_type: String::from("i128"),
                src_value: v.to_string(),
            })
        })?;
        Ok(Value::Decimal(tmp))
    }
    pub fn decimal_from_u8(v: u8) -> Value {
        Value::Decimal(Decimal::from(v))
    }
    pub fn decimal_from_u16(v: u16) -> Value {
        Value::Decimal(Decimal::from(v))
    }
    pub fn decimal_from_u32(v: u32) -> Value {
        Value::Decimal(Decimal::from(v))
    }
    pub fn decimal_from_u64(v: u64) -> Value {
        Value::Decimal(Decimal::from(v))
    }
    pub fn decimal_from_u128(v: u128) -> Result<Value> {
        let tmp = Decimal::from_u128(v).ok_or_else(|| {
            VenumError::Conversion(ConversionError::NotRepresentableAsDecimal {
                src_type: String::from("u128"),
                src_value: v.to_string(),
            })
        })?;
        Ok(Value::Decimal(tmp))
    }
    pub fn decimal_from_isize(v: isize) -> Value {
        Value::Decimal(Decimal::from(v))
    }
    pub fn decimal_from_usize(v: usize) -> Value {
        Value::Decimal(Decimal::from(v))
    }
    pub fn decimal_from_f32(v: f32) -> Result<Value> {
        let tmp = Decimal::from_f32(v).ok_or_else(|| {
            VenumError::Conversion(ConversionError::NotRepresentableAsDecimal {
                src_type: String::from("f32"),
                src_value: v.to_string(),
            })
        })?;
        Ok(Value::Decimal(tmp))
    }
    pub fn decimal_from_f64(v: f64) -> Result<Value> {
        let tmp = Decimal::from_f64(v).ok_or_else(|| {
            VenumError::Conversion(ConversionError::NotRepresentableAsDecimal {
                src_type: String::from("f64"),
                src_value: v.to_string(),
            })
        })?;
        Ok(Value::Decimal(tmp))
    }
    // retain access bits / approximation
    pub fn decimal_from_f32_retain(v: f32) -> Result<Value> {
        let tmp = Decimal::from_f32_retain(v).ok_or_else(|| {
            VenumError::Conversion(ConversionError::NotRepresentableAsDecimal {
                src_type: String::from("f32"),
                src_value: v.to_string(),
            })
        })?;
        Ok(Value::Decimal(tmp))
    }
    // retain access bits / approximation
    pub fn decimal_from_f64_retain(v: f64) -> Result<Value> {
        let tmp = Decimal::from_f64_retain(v).ok_or_else(|| {
            VenumError::Conversion(ConversionError::NotRepresentableAsDecimal {
                src_type: String::from("f64"),
                src_value: v.to_string(),
            })
        })?;
        Ok(Value::Decimal(tmp))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Value::None)
    }
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
    is_type!(is_char, Char);
    is_type!(is_string, String);
    is_type!(is_int8, Int8);
    is_type!(is_int16, Int16);
    is_type!(is_int32, Int32);
    is_type!(is_int64, Int64);
    is_type!(is_int128, Int128);
    is_type!(is_uint8, UInt8);
    is_type!(is_uint16, UInt16);
    is_type!(is_uint32, UInt32);
    is_type!(is_uint64, UInt64);
    is_type!(is_uint128, UInt128);
    is_type!(is_float32, Float32);
    is_type!(is_float64, Float64);
    is_type!(is_bool, Bool);
    is_type!(is_decimal, Decimal);
    is_type!(is_naive_date, NaiveDate);
    is_type!(is_naive_date_time, NaiveDateTime);
    is_type!(is_date_time, DateTime);

    /// NOTE: We decided against Option<String> here as the type of the value since the intention is to create a typed version of a stringy-input we read from some CSV.
    ///       In that case, when a CSV column contains a "" as an entry, e.g. like this: `a,,c` or this `"a","","c"`, where the middle column would translate to empty / "",
    ///       we map it to a None internally, representing the absence of data.
    /// NOTE2: For date types, when no chrono_pattern is supplied, parsing is still tried, using: iso8601_ymd, iso8601_ymdhms and rfc3339.
    pub fn from_str_and_type_with_chrono_pattern_with_none_map(
        value: &str,
        target_value_type: &ValueType,
        chrono_pattern: Option<&str>,
        as_none: Option<Vec<&str>>,
    ) -> Result<Value> {
        if value.is_empty() {
            return Ok(Value::None); // Caller must remember the desired type!
        }
        if let Some(none_check_vals) = as_none {
            if !none_check_vals.is_empty() && none_check_vals.contains(&value) {
                return Ok(Value::None); // Caller must remember the desired type!
            }
        }
        if let Some(chrono_pattern) = chrono_pattern {
            match target_value_type {
                ValueType::NaiveDate => Value::parse_naive_date_from_str(value, chrono_pattern),
                ValueType::NaiveDateTime => {
                    Value::parse_naive_date_time_from_str(value, chrono_pattern)
                }
                ValueType::DateTime => Value::parse_date_time_from_str(value, chrono_pattern),
                _ => Err(VenumError::Parsing(ParseError::ValueFromStringFailed {
                    src_value: String::from(value),
                    target_type: format!("{}{}", VAL_ENUM_NAME, target_value_type),
                    details: Some(format!("Chrono pattern: {chrono_pattern}")),
                })),
            }
        } else {
            match target_value_type {
                ValueType::Char => Value::parse_char_from_str(value),
                ValueType::String => Ok(Value::String(value.to_owned())),
                ValueType::Int8 => Value::parse_int8_from_str(value),
                ValueType::Int16 => Value::parse_int16_from_str(value),
                ValueType::Int32 => Value::parse_int32_from_str(value),
                ValueType::Int64 => Value::parse_int64_from_str(value),
                ValueType::Int128 => Value::parse_int128_from_str(value),
                ValueType::UInt8 => Value::parse_uint8_from_str(value),
                ValueType::UInt16 => Value::parse_uint16_from_str(value),
                ValueType::UInt32 => Value::parse_uint32_from_str(value),
                ValueType::UInt64 => Value::parse_uint64_from_str(value),
                ValueType::UInt128 => Value::parse_uint128_from_str(value),
                ValueType::Float32 => Value::parse_float32_from_str(value),
                ValueType::Float64 => Value::parse_float64_from_str(value),
                ValueType::Bool => Value::parse_bool_from_str(value),
                ValueType::Decimal => Value::parse_decimal_from_str(value),
                ValueType::NaiveDate => Value::parse_naive_date_from_str_iso8601_ymd(value),
                ValueType::NaiveDateTime => {
                    match Value::parse_naive_date_time_from_str_iso8601_ymd_hms(value) {
                        Ok(v) => Ok(v),
                        Err(_) => {
                            Value::parse_naive_date_time_from_str_iso8601_ymd_hms_millies(value)
                        }
                    }
                }
                ValueType::DateTime => Value::parse_date_time_from_str_rfc3339(value),
            }
        }
    }

    pub fn from_str_and_type(value: &str, target_value_type: &ValueType) -> Result<Value> {
        Self::from_str_and_type_with_chrono_pattern_with_none_map(
            value,
            target_value_type,
            None,
            None,
        )
    }

    pub fn from_str_and_type_with_chrono_pattern(
        value: &str,
        target_value_type: &ValueType,
        chrono_pattern: &str,
    ) -> Result<Value> {
        Self::from_str_and_type_with_chrono_pattern_with_none_map(
            value,
            target_value_type,
            Some(chrono_pattern),
            None,
        )
    }

    pub fn from_str_and_type_with_none_map(
        value: &str,
        target_value_type: &ValueType,
        as_none: Vec<&str>,
    ) -> Result<Value> {
        Self::from_str_and_type_with_chrono_pattern_with_none_map(
            value,
            target_value_type,
            None,
            Some(as_none),
        )
    }

    pub fn is_some_date_type(&self) -> bool {
        self.is_naive_date() || self.is_naive_date_time() || self.is_date_time()
    }

    pub fn is_some_signed_int_type(&self) -> bool {
        self.is_int8() || self.is_int16() || self.is_int32() || self.is_int64() || self.is_int128()
    }

    pub fn is_some_unsigned_int_type(&self) -> bool {
        self.is_uint8()
            || self.is_uint16()
            || self.is_uint32()
            || self.is_uint64()
            || self.is_uint128()
    }

    pub fn is_some_int_type(&self) -> bool {
        self.is_some_signed_int_type() || self.is_some_unsigned_int_type()
    }

    pub fn is_some_float_type(&self) -> bool {
        self.is_float32() || self.is_float64()
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Bool(false)
    }
}

pub type OptValue = Option<Value>;

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: more tests for from_type_string
    mod parse_from_str {
        use super::*;

        #[test]
        pub fn parse_int8_from_str() {
            assert_eq!(
                Ok(Value::Int8(i8::MAX)),
                Value::parse_int8_from_str(&i8::MAX.to_string())
            );
            assert_eq!(
                Ok(Value::Int8(i8::MIN)),
                Value::parse_int8_from_str(&i8::MIN.to_string())
            );
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_int8_from_str_err_val_too_big() {
            Value::parse_int8_from_str(&i16::MAX.to_string()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_int8_from_str_err_val_too_small() {
            Value::parse_int8_from_str(&i16::MIN.to_string()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_int8_from_str_err_not_an_int8() {
            Value::parse_int8_from_str("not-an-int8").unwrap();
        }

        #[test]
        pub fn parse_int16_from_str() {
            assert_eq!(
                Ok(Value::Int16(i16::MAX)),
                Value::parse_int16_from_str(&i16::MAX.to_string())
            );
            assert_eq!(
                Ok(Value::Int16(i16::MIN)),
                Value::parse_int16_from_str(&i16::MIN.to_string())
            );
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_int16_from_str_err_val_too_big() {
            Value::parse_int16_from_str(&i32::MAX.to_string()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_int16_from_str_err_val_too_small() {
            Value::parse_int16_from_str(&i32::MIN.to_string()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_int16_from_str_err_not_an_int16() {
            Value::parse_int16_from_str("not-an-int16").unwrap();
        }

        #[test]
        pub fn parse_int32_from_str() {
            assert_eq!(
                Ok(Value::Int32(i32::MAX)),
                Value::parse_int32_from_str(&i32::MAX.to_string())
            );
            assert_eq!(
                Ok(Value::Int32(i32::MIN)),
                Value::parse_int32_from_str(&i32::MIN.to_string())
            );
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_int32_from_str_err_val_too_big() {
            Value::parse_int32_from_str(&i64::MAX.to_string()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_int32_from_str_err_val_too_small() {
            Value::parse_int32_from_str(&i64::MIN.to_string()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_int32_from_str_err_not_an_int32() {
            Value::parse_int32_from_str("not-an-int32").unwrap();
        }

        #[test]
        pub fn parse_int64_from_str() {
            assert_eq!(
                Ok(Value::Int64(i64::MAX)),
                Value::parse_int64_from_str(&i64::MAX.to_string())
            );
            assert_eq!(
                Ok(Value::Int64(i64::MIN)),
                Value::parse_int64_from_str(&i64::MIN.to_string())
            );
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_int64_from_str_err_val_too_big() {
            Value::parse_int64_from_str(&i128::MAX.to_string()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_int64_from_str_err_val_too_small() {
            Value::parse_int64_from_str(&i128::MIN.to_string()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_int64_from_str_err_not_an_int64() {
            Value::parse_int64_from_str("not-an-int64").unwrap();
        }

        #[test]
        pub fn parse_int128_from_str() {
            assert_eq!(
                Ok(Value::Int128(i128::MAX)),
                Value::parse_int128_from_str(&i128::MAX.to_string())
            );
            assert_eq!(
                Ok(Value::Int128(i128::MIN)),
                Value::parse_int128_from_str(&i128::MIN.to_string())
            );
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_int128_from_str_err_val_too_big() {
            // max is: 170141183460469231731687303715884105727
            Value::parse_int128_from_str("170141183460469231731687303715884105728").unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_int128_from_str_err_val_too_small() {
            // min is: -170141183460469231731687303715884105728
            Value::parse_int128_from_str("-170141183460469231731687303715884105729").unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_int128_from_str_err_not_an_int128() {
            Value::parse_int128_from_str("not-an-int128").unwrap();
        }

        #[test]
        pub fn parse_uint8_from_str() {
            assert_eq!(
                Ok(Value::UInt8(u8::MAX)),
                Value::parse_uint8_from_str(&u8::MAX.to_string())
            );
            assert_eq!(
                Ok(Value::UInt8(u8::MIN)),
                Value::parse_uint8_from_str(&u8::MIN.to_string())
            );
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_uint8_from_str_err_val_too_big() {
            Value::parse_uint8_from_str(&u16::MAX.to_string()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_uint8_from_str_err_val_too_small() {
            Value::parse_uint8_from_str("-1").unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_uint8_from_str_err_not_an_uint8() {
            Value::parse_uint8_from_str("not-an-uint8").unwrap();
        }

        #[test]
        pub fn parse_uint16_from_str() {
            assert_eq!(
                Ok(Value::UInt16(u16::MAX)),
                Value::parse_uint16_from_str(&u16::MAX.to_string())
            );
            assert_eq!(
                Ok(Value::UInt16(u16::MIN)),
                Value::parse_uint16_from_str(&u16::MIN.to_string())
            );
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_uint16_from_str_err_val_too_big() {
            Value::parse_uint16_from_str(&u32::MAX.to_string()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_uint16_from_str_err_val_too_small() {
            Value::parse_uint16_from_str("-1").unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_uint16_from_str_err_not_an_uint16() {
            Value::parse_uint16_from_str("not-an-uint16").unwrap();
        }

        #[test]
        pub fn parse_uint32_from_str() {
            assert_eq!(
                Ok(Value::UInt32(u32::MAX)),
                Value::parse_uint32_from_str(&u32::MAX.to_string())
            );
            assert_eq!(
                Ok(Value::UInt32(u32::MIN)),
                Value::parse_uint32_from_str(&u32::MIN.to_string())
            );
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_uint32_from_str_err_val_too_big() {
            Value::parse_uint32_from_str(&u64::MAX.to_string()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_uint32_from_str_err_val_too_small() {
            Value::parse_uint32_from_str("-1").unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_uint32_from_str_err_not_an_uint32() {
            Value::parse_uint32_from_str("not-an-uint32").unwrap();
        }

        #[test]
        pub fn parse_uint64_from_str() {
            assert_eq!(
                Ok(Value::UInt64(u64::MAX)),
                Value::parse_uint64_from_str(&u64::MAX.to_string())
            );
            assert_eq!(
                Ok(Value::UInt64(u64::MIN)),
                Value::parse_uint64_from_str(&u64::MIN.to_string())
            );
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_uint64_from_str_err_val_too_big() {
            Value::parse_uint64_from_str(&u128::MAX.to_string()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_uint64_from_str_err_val_too_small() {
            Value::parse_uint64_from_str("-1").unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_uint64_from_str_err_not_an_uint64() {
            Value::parse_uint64_from_str("not-an-uint64").unwrap();
        }

        #[test]
        pub fn parse_uint128_from_str() {
            assert_eq!(
                Ok(Value::UInt128(u128::MAX)),
                Value::parse_uint128_from_str(&u128::MAX.to_string())
            );
            assert_eq!(
                Ok(Value::UInt128(u128::MIN)),
                Value::parse_uint128_from_str(&u128::MIN.to_string())
            );
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_uint128_from_str_err_val_too_big() {
            // max is: 340282366920938463463374607431768211455
            Value::parse_uint128_from_str("340282366920938463463374607431768211456").unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_uint128_from_str_err_val_too_small() {
            Value::parse_uint128_from_str("-1").unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_uint128_from_str_err_not_an_uint128() {
            Value::parse_uint128_from_str("not-an-uint128").unwrap();
        }

        #[test]
        pub fn parse_float32_from_str() {
            assert_eq!(
                Ok(Value::Float32(f32::MAX)),
                Value::parse_float32_from_str(&f32::MAX.to_string())
            );
            assert_eq!(
                Ok(Value::Float32(f32::MIN)),
                Value::parse_float32_from_str(&f32::MIN.to_string())
            );
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_float32_from_str_err_val_too_big() {
            Value::parse_float32_from_str(&f64::MAX.to_string()).unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_float32_from_str_err_val_too_small() {
            Value::parse_float32_from_str(&f64::MIN.to_string()).unwrap();
        }

        #[test]
        pub fn parse_float32_from_str_pos_inf() {
            assert_eq!(
                Value::Float32(f32::INFINITY),
                Value::parse_float32_from_str_allow_inf_allow_nan("1.7976931348623157E+309")
                    .unwrap(),
            );
        }

        #[test]
        pub fn parse_float32_from_str_neg_inf() {
            assert_eq!(
                Value::Float32(f32::NEG_INFINITY),
                Value::parse_float32_from_str_allow_inf_allow_nan("-1.7976931348623157E+309")
                    .unwrap(),
            );
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_float32_from_str_err_not_an_float32() {
            Value::parse_float32_from_str("not-an-float32").unwrap();
        }

        #[test]
        pub fn parse_float64_from_str() {
            assert_eq!(
                Ok(Value::Float64(f64::MAX)),
                Value::parse_float64_from_str(&f64::MAX.to_string())
            );
            assert_eq!(
                Ok(Value::Float64(f64::MIN)),
                Value::parse_float64_from_str(&f64::MIN.to_string())
            );
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_float64_from_str_err_val_too_big() {
            Value::parse_float64_from_str("1.7976931348623157E+309").unwrap();
        }

        #[test]
        pub fn parse_float64_from_str_pos_inf() {
            assert_eq!(
                Value::Float64(f64::INFINITY),
                Value::parse_float64_from_str_allow_inf_allow_nan("1.7976931348623157E+309")
                    .unwrap(),
            );
        }

        #[test]
        pub fn parse_float64_from_str_neg_inf() {
            assert_eq!(
                Value::Float64(f64::NEG_INFINITY),
                Value::parse_float64_from_str_allow_inf_allow_nan("-1.7976931348623157E+309")
                    .unwrap(),
            );
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_float64_from_str_err_val_too_small() {
            Value::parse_float64_from_str("-1.7976931348623157E+309").unwrap();
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_float64_from_str_err_not_an_float64() {
            Value::parse_float64_from_str("not-an-float64").unwrap();
        }

        #[test]
        pub fn parse_bool_from_str_true() {
            assert_eq!(Ok(Value::Bool(true)), Value::parse_bool_from_str("true"));
        }

        #[test]
        pub fn parse_bool_from_str_false() {
            assert_eq!(Ok(Value::Bool(false)), Value::parse_bool_from_str("false"));
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn parse_bool_from_str_err() {
            Value::parse_bool_from_str("not-a-bool").unwrap();
        }

        #[test]
        pub fn parse_char_from_str() {
            assert_eq!(Ok(Value::Char('a')), Value::parse_char_from_str("a"));
            assert_eq!(Ok(Value::Char('1')), Value::parse_char_from_str("1"));
        }

        #[test]
        #[should_panic(expected = "Err(Parsing(ValueFromStringFailed")]
        pub fn parse_char_from_str_err() {
            assert_eq!(Ok(Value::Char('a')), Value::parse_char_from_str("abc"));
        }
    }

    mod parse_from_str_decimal {
        use rust_decimal::Decimal;

        use crate::value::Value;

        #[test]
        pub fn parse_decimal_from_str() {
            assert_eq!(
                Ok(Value::Decimal(Decimal::new(1123, 3))),
                Value::parse_decimal_from_str("1.123")
            );
        }

        #[test]
        #[should_panic(
            expected = "Parsing(ValueFromStringFailed { src_value: \"foobar\", target_type: \"Value::Decimal\", details: Some(\"Original error: Invalid decimal: unknown character\") })"
        )]
        pub fn parse_decimal_from_str_err() {
            Value::parse_decimal_from_str("foobar").unwrap();
        }
    }

    mod parse_from_str_date {
        use chrono::{DateTime, FixedOffset, NaiveDate, TimeZone};

        use crate::value::Value;

        #[test]
        pub fn parse_naive_date_from_str_w_pattern() {
            let exp = NaiveDate::from_ymd(2022, 12, 31);

            assert_eq!(
                Ok(Value::NaiveDate(exp)),
                Value::parse_naive_date_from_str("2022-12-31", "%Y-%m-%d")
            );
            assert_eq!(
                Ok(Value::NaiveDate(exp)),
                Value::parse_naive_date_from_str("2022-12-31", "%F")
            );
        }

        #[test]
        #[should_panic(
            expected = "Parsing(ValueFromStringFailed { src_value: \"2022-12-31 00:00\", target_type: \"Value::NaiveDate\", details: Some(\"Chrono pattern: %Y-%m-%d. Original error: trailing input\") })"
        )]
        pub fn parse_naive_date_from_str_w_pattern_err_trailing_inp() {
            Value::parse_naive_date_from_str("2022-12-31 00:00", "%Y-%m-%d").unwrap();
        }

        #[test]
        #[should_panic(
            expected = "Parsing(ValueFromStringFailed { src_value: \"2022-12-31\", target_type: \"Value::NaiveDate\", details: Some(\"Chrono pattern: %Y %m %d. Original error: input contains invalid characters\") })"
        )]
        pub fn parse_naive_date_from_str_w_pattern_err_invalid_chars() {
            Value::parse_naive_date_from_str("2022-12-31", "%Y %m %d").unwrap();
        }

        #[test]
        pub fn parse_naive_date_from_str_iso8601_ymd() {
            let exp = NaiveDate::from_ymd(2022, 12, 31);

            assert_eq!(
                Ok(Value::NaiveDate(exp)),
                Value::parse_naive_date_from_str_iso8601_ymd("2022-12-31")
            );
        }

        #[test]
        pub fn parse_naive_date_time_from_str_w_pattern() {
            let exp = NaiveDate::from_ymd(2022, 12, 31).and_hms(12, 11, 10);

            assert_eq!(
                Ok(Value::NaiveDateTime(exp)),
                Value::parse_naive_date_time_from_str("2022-12-31 12:11:10", "%Y-%m-%d %H:%M:%S")
            );
            assert_eq!(
                Ok(Value::NaiveDateTime(exp)),
                Value::parse_naive_date_time_from_str("2022-12-31 12:11:10", "%F %T")
            );
        }

        #[test]
        #[should_panic(
            expected = "Parsing(ValueFromStringFailed { src_value: \"2022-12-31 12:11:10 000\", target_type: \"Value::NaiveDateTime\", details: Some(\"Chrono pattern: %Y-%m-%d %H:%M:%S. Original error: trailing input\") }"
        )]
        pub fn parse_naive_date_time_from_str_w_pattern_err_trailing_inp() {
            Value::parse_naive_date_time_from_str("2022-12-31 12:11:10 000", "%Y-%m-%d %H:%M:%S")
                .unwrap();
        }

        #[test]
        #[should_panic(
            expected = "Parsing(ValueFromStringFailed { src_value: \"2022-12-31 12:11:10\", target_type: \"Value::NaiveDateTime\", details: Some(\"Chrono pattern: %Y-%m-%dT%H:%M:%S. Original error: input contains invalid characters\") }"
        )]
        pub fn parse_naive_date_time_from_str_w_pattern_err_invalid_chars() {
            Value::parse_naive_date_time_from_str("2022-12-31 12:11:10", "%Y-%m-%dT%H:%M:%S")
                .unwrap();
        }

        #[test]
        pub fn parse_naive_date_time_from_str_iso8601_ymd_hms() {
            let exp = NaiveDate::from_ymd(2022, 12, 31).and_hms(12, 11, 10);
            assert_eq!(
                Ok(Value::NaiveDateTime(exp)),
                Value::parse_naive_date_time_from_str_iso8601_ymd_hms("2022-12-31T12:11:10")
            );
        }

        #[test]
        pub fn parse_naive_date_time_from_str_iso8601_ymd_hms_millies() {
            let exp = NaiveDate::from_ymd(2022, 12, 31).and_hms_milli(12, 11, 10, 100);
            assert_eq!(
                Ok(Value::NaiveDateTime(exp)),
                Value::parse_naive_date_time_from_str_iso8601_ymd_hms_millies(
                    "2022-12-31T12:11:10.100"
                )
            );

            let exp = NaiveDate::from_ymd(2022, 12, 31).and_hms_milli(12, 11, 10, 0);
            assert_eq!(
                Ok(Value::NaiveDateTime(exp)),
                Value::parse_naive_date_time_from_str_iso8601_ymd_hms_millies(
                    "2022-12-31T12:11:10.000"
                )
            );

            let exp = NaiveDate::from_ymd(2022, 12, 31).and_hms_milli(12, 11, 10, 0);
            assert_eq!(
                Ok(Value::NaiveDateTime(exp)),
                Value::parse_naive_date_time_from_str_iso8601_ymd_hms_millies(
                    "2022-12-31T12:11:10"
                )
            );
        }

        #[test]
        pub fn parse_date_time_from_str_w_pattern() {
            let hour_secs = 3600;
            let exp: DateTime<FixedOffset> = FixedOffset::east(5 * hour_secs) // east = +; west = -
                .ymd(2022, 12, 31)
                .and_hms(6, 0, 0);

            let date_str = "2022-12-31T06:00:00+05:00";
            let res = Value::parse_date_time_from_str(date_str, "%FT%T%:z");
            assert_eq!(Ok(Value::DateTime(exp)), res);

            let dt = DateTime::try_from(res.unwrap()).unwrap();
            assert_eq!(dt.to_rfc3339(), String::from(date_str));
        }

        #[test]
        #[should_panic(
            expected = "Parsing(ValueFromStringFailed { src_value: \"2022-12-31T06:00:00\", target_type: \"Value::DateTime\", details: Some(\"Chrono pattern: %FT%T%:z. Original error: premature end of input\") }"
        )]
        pub fn parse_date_time_from_str_w_pattern_err_prem_end_of_input() {
            Value::parse_date_time_from_str("2022-12-31T06:00:00", "%FT%T%:z").unwrap();
        }

        #[test]
        #[should_panic(
            expected = "Parsing(ValueFromStringFailed { src_value: \"2022-12-31T06:00:00\", target_type: \"Value::DateTime\", details: Some(\"Chrono pattern: %FT%T. Original error: input is not enough for unique date and time\") })"
        )]
        pub fn parse_date_time_from_str_w_pattern_err_invalid_chars() {
            Value::parse_date_time_from_str("2022-12-31T06:00:00", "%FT%T").unwrap();
        }

        #[test]
        #[should_panic(
            expected = "Parsing(ValueFromStringFailed { src_value: \"2022-12-31T06:00:00+05:00\", target_type: \"Value::DateTime\", details: Some(\"Chrono pattern: %FT%T%. Original error: bad or unsupported format string\") }"
        )]
        pub fn parse_date_time_from_str_w_pattern_err_bad_format_string() {
            Value::parse_date_time_from_str("2022-12-31T06:00:00+05:00", "%FT%T%").unwrap();
        }

        #[test]
        pub fn parse_date_time_from_str_rfc3339_wo_millies() {
            let hour_secs = 3600;
            let exp: DateTime<FixedOffset> = FixedOffset::east(5 * hour_secs) // east = +; west = -
                .ymd(2022, 12, 31)
                .and_hms(6, 0, 0);

            let date_str = "2022-12-31T06:00:00+05:00";
            let res = Value::parse_date_time_from_str_rfc3339(date_str);
            assert_eq!(Ok(Value::DateTime(exp)), res);

            let dt = DateTime::try_from(res.unwrap()).unwrap();
            assert_eq!(dt.to_rfc3339(), String::from(date_str));
        }

        #[test]
        pub fn parse_date_time_from_str_rfc3339_w_millies() {
            let hour_secs = 3600;
            let exp: DateTime<FixedOffset> = FixedOffset::east(5 * hour_secs) // east = +; west = -
                .ymd(2022, 12, 31)
                .and_hms_milli(6, 0, 0, 100);

            let date_str = "2022-12-31T06:00:00.100+05:00";
            let res = Value::parse_date_time_from_str_rfc3339(date_str);
            assert_eq!(Ok(Value::DateTime(exp)), res);

            let dt = DateTime::try_from(res.unwrap()).unwrap();
            assert_eq!(dt.to_rfc3339(), String::from(date_str));
        }

        #[test]
        pub fn parse_date_time_from_str_rfc3339_w_millies_empty() {
            let hour_secs = 3600;
            let exp: DateTime<FixedOffset> = FixedOffset::east(5 * hour_secs) // east = +; west = -
                .ymd(2022, 12, 31)
                .and_hms_milli(6, 0, 0, 0);

            let date_str = "2022-12-31T06:00:00.000+05:00";
            let res = Value::parse_date_time_from_str_rfc3339(date_str);
            assert_eq!(Ok(Value::DateTime(exp)), res);

            let dt = DateTime::try_from(res.unwrap()).unwrap();
            assert_eq!(
                dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                String::from(date_str)
            );
        }

        #[test]
        pub fn parse_date_time_from_str_rfc2822() {
            let hour_secs = 3600;
            let exp: DateTime<FixedOffset> = FixedOffset::east(2 * hour_secs) // east = +; west = -
                .ymd(2003, 7, 1)
                .and_hms(10, 52, 37);

            let date_str = "Tue, 01 Jul 2003 10:52:37 +0200";
            let res = Value::parse_date_time_from_str_rfc2822(date_str);
            assert_eq!(Ok(Value::DateTime(exp)), res);

            let dt = DateTime::try_from(res.unwrap()).unwrap();
            assert_eq!(dt.to_rfc2822(), String::from(date_str));
        }
    }

    mod default_values {
        use chrono::{DateTime, FixedOffset, NaiveDate, TimeZone};
        use rust_decimal::Decimal;

        use crate::value::Value;

        #[test]
        pub fn char_default() {
            assert_eq!(Value::Char('\0'), Value::char_default());
        }

        #[test]
        pub fn string_default() {
            assert_eq!(Value::String("".to_string()), Value::string_default());
        }

        #[test]
        pub fn int8_default() {
            assert_eq!(Value::Int8(0), Value::int8_default());
        }

        #[test]
        pub fn int16_default() {
            assert_eq!(Value::Int16(0), Value::int16_default());
        }

        #[test]
        pub fn int32_default() {
            assert_eq!(Value::Int32(0), Value::int32_default());
        }

        #[test]
        pub fn int64_default() {
            assert_eq!(Value::Int64(0), Value::int64_default());
        }

        #[test]
        pub fn int128_default() {
            assert_eq!(Value::Int128(0), Value::int128_default());
        }

        #[test]
        pub fn uint8_default() {
            assert_eq!(Value::UInt8(0), Value::uint8_default());
        }

        #[test]
        pub fn uint16_default() {
            assert_eq!(Value::UInt16(0), Value::uint16_default());
        }

        #[test]
        pub fn uint32_default() {
            assert_eq!(Value::UInt32(0), Value::uint32_default());
        }

        #[test]
        pub fn uint64_default() {
            assert_eq!(Value::UInt64(0), Value::uint64_default());
        }

        #[test]
        pub fn uint128_default() {
            assert_eq!(Value::UInt128(0), Value::uint128_default());
        }

        #[test]
        pub fn float32_default() {
            assert_eq!(Value::Float32(0.0), Value::float32_default());
        }

        #[test]
        pub fn float64_default() {
            assert_eq!(Value::Float64(0.0), Value::float64_default());
        }

        #[test]
        pub fn bool_default() {
            assert_eq!(Value::Bool(false), Value::bool_default());
        }

        #[test]
        pub fn decimal_default() {
            assert_eq!(
                Value::Decimal(Decimal::new(00, 1)),
                Value::decimal_default()
            );
        }

        #[test]
        pub fn naive_date_default() {
            assert_eq!(
                Value::NaiveDate(NaiveDate::from_ymd(1970, 01, 01)),
                Value::naive_date_default()
            );
        }

        #[test]
        pub fn naive_date_time_default() {
            assert_eq!(
                Value::NaiveDateTime(NaiveDate::from_ymd(1970, 01, 01).and_hms(00, 00, 00)),
                Value::naive_date_time_default()
            );
        }

        #[test]
        pub fn date_time_default() {
            let exp: DateTime<FixedOffset> = FixedOffset::east(0) // east = +; west = -
                .ymd(1970, 01, 01)
                .and_hms(0, 0, 0);
            assert_eq!(Value::DateTime(exp), Value::date_time_default());
        }
    }

    mod from_number_type_to_value_decimal {
        use rust_decimal::Decimal;

        use crate::{
            errors_result::{ConversionError, VenumError},
            value::Value,
        };

        #[test]
        pub fn from_signed_ints_max_to_value_decimal() {
            assert_eq!(
                Value::Decimal(Decimal::from(i8::MAX)),
                Value::decimal_from_i8(i8::MAX)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(i16::MAX)),
                Value::decimal_from_i16(i16::MAX)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(i32::MAX)),
                Value::decimal_from_i32(i32::MAX)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(i64::MAX)),
                Value::decimal_from_i64(i64::MAX)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(128_i128)),
                Value::decimal_from_i128(128_i128).unwrap()
            );
            assert_eq!(
                Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAsDecimal {
                        src_type: String::from("i128"),
                        src_value: i128::MAX.to_string()
                    }
                )),
                Value::decimal_from_i128(i128::MAX)
            );
        }

        #[test]
        pub fn from_signed_ints_min_to_value_decimal() {
            assert_eq!(
                Value::Decimal(Decimal::from(i8::MIN)),
                Value::decimal_from_i8(i8::MIN)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(i16::MIN)),
                Value::decimal_from_i16(i16::MIN)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(i32::MIN)),
                Value::decimal_from_i32(i32::MIN)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(i64::MIN)),
                Value::decimal_from_i64(i64::MIN)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(-128_i128)),
                Value::decimal_from_i128(-128_i128).unwrap()
            );
            // TODO: This panics, but I guess it shouldn't?
            // assert_eq!(
            //     Err(VenumError::Conversion(
            //         ConversionError::NotRepresentableAsDecimal {
            //             src_type: String::from("i128"),
            //             src_value: i128::MIN.to_string()
            //         }
            //     )),
            //     Value::decimal_from_i128(i128::MIN)
            // );
        }

        #[test]
        pub fn from_unsigned_ints_max_to_value_decimal() {
            assert_eq!(
                Value::Decimal(Decimal::from(u8::MAX)),
                Value::decimal_from_u8(u8::MAX)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(u16::MAX)),
                Value::decimal_from_u16(u16::MAX)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(u32::MAX)),
                Value::decimal_from_u32(u32::MAX)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(u64::MAX)),
                Value::decimal_from_u64(u64::MAX)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(128_u128)),
                Value::decimal_from_u128(128_u128).unwrap()
            );
            assert_eq!(
                Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAsDecimal {
                        src_type: String::from("u128"),
                        src_value: u128::MAX.to_string()
                    }
                )),
                Value::decimal_from_u128(u128::MAX)
            );
        }

        #[test]
        pub fn from_unsigned_ints_min_to_value_decimal() {
            assert_eq!(
                Value::Decimal(Decimal::from(u8::MIN)),
                Value::decimal_from_u8(u8::MIN)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(u16::MIN)),
                Value::decimal_from_u16(u16::MIN)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(u32::MIN)),
                Value::decimal_from_u32(u32::MIN)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(u64::MIN)),
                Value::decimal_from_u64(u64::MIN)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(u128::MIN)),
                Value::decimal_from_u128(u128::MIN).unwrap()
            );
        }

        #[test]
        pub fn from_usize_isize_min_max() {
            assert_eq!(
                Value::Decimal(Decimal::from(usize::MIN)),
                Value::decimal_from_usize(usize::MIN)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(usize::MAX)),
                Value::decimal_from_usize(usize::MAX)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(isize::MIN)),
                Value::decimal_from_isize(isize::MIN)
            );
            assert_eq!(
                Value::Decimal(Decimal::from(isize::MAX)),
                Value::decimal_from_isize(isize::MAX)
            );
        }
    }

    mod conversions {
        use chrono::NaiveDate;

        use crate::{value::Value, value_type::ValueType};

        #[test]
        pub fn from_str_and_type_int8_ok_none() {
            let test = Value::from_str_and_type("", &ValueType::Int8);
            assert_eq!(Ok(Value::None), test);
        }

        #[test]
        pub fn from_str_and_type_int8_ok() {
            let test = Value::from_str_and_type("10", &ValueType::Int8);
            assert_eq!(Ok(Value::Int8(10)), test);
        }

        #[test]
        #[should_panic(expected = "Parsing(ValueFromStringFailed")]
        pub fn from_str_and_type_int8_err() {
            Value::from_str_and_type("false", &ValueType::Int8).unwrap();
        }

        #[test]
        pub fn from_str_and_type_with_chrono_naive_date_ok_none() {
            let test =
                Value::from_str_and_type_with_chrono_pattern("", &ValueType::NaiveDate, "%d.%m.%Y");
            assert_eq!(Ok(Value::None), test);
        }

        #[test]
        pub fn from_str_and_type_with_chrono_naive_date_ok() {
            let test = Value::from_str_and_type_with_chrono_pattern(
                "31.12.2022",
                &ValueType::NaiveDate,
                "%d.%m.%Y",
            );
            assert_eq!(
                Ok(Value::NaiveDate(NaiveDate::from_ymd(2022, 12, 31))),
                test
            );
        }

        #[test]
        #[should_panic(
            expected = "Parsing(ValueFromStringFailed { src_value: \"foobar\", target_type: \"Value::NaiveDate\", details: Some(\"Chrono pattern: %d.%m.%Y. Original error:"
        )]
        pub fn from_str_and_type_with_chrono_naive_date_err() {
            Value::from_str_and_type_with_chrono_pattern(
                "foobar",
                &ValueType::NaiveDate,
                "%d.%m.%Y",
            )
            .unwrap();
        }
    }
}
