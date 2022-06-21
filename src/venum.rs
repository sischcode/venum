use std::convert::From;

use crate::errors::{ConversionError, ParseError, Result, VenumError};

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};
use rust_decimal::{prelude::FromPrimitive, Decimal};

use strum_macros::Display; // used to generate names for the enum variants. Used only for error messages (as of now).

const VAL_ENUM_NAME: &str = "Value::";
const ENUM_VAR_ND: &str = "NaiveDate";
const ENUM_VAR_NDT: &str = "NaiveDateTime";
const ENUM_VAR_DT: &str = "DateTime";

#[derive(Display, Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    // TODO: do we need Char(char) as well?
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

macro_rules! impl_from_type_for_value {
    ($enum_type:ident, $from_type:ty) => {
        impl From<$from_type> for Value {
            fn from(item: $from_type) -> Self {
                Value::$enum_type(item)
            }
        }
    };
}
impl_from_type_for_value!(String, String);
impl_from_type_for_value!(Int8, i8);
impl_from_type_for_value!(Int16, i16);
impl_from_type_for_value!(Int32, i32);
impl_from_type_for_value!(Int64, i64);
impl_from_type_for_value!(Int128, i128);
impl_from_type_for_value!(UInt8, u8);
impl_from_type_for_value!(UInt16, u16);
impl_from_type_for_value!(UInt32, u32);
impl_from_type_for_value!(UInt64, u64);
impl_from_type_for_value!(UInt128, u128);
impl_from_type_for_value!(Float32, f32);
impl_from_type_for_value!(Float64, f64);
impl_from_type_for_value!(Bool, bool);
impl_from_type_for_value!(Decimal, Decimal);
impl_from_type_for_value!(NaiveDate, NaiveDate);
impl_from_type_for_value!(NaiveDateTime, NaiveDateTime);
impl_from_type_for_value!(DateTime, DateTime<FixedOffset>);

macro_rules! impl_try_from_value_for_type {
    ($enum_type:ident, $for_type:ty) => {
        impl TryFrom<Value> for $for_type {
            type Error = VenumError;
            fn try_from(item: Value) -> Result<Self> {
                match item {
                    Value::$enum_type(v) => Ok(v),
                    _ => Err(VenumError::Conversion(ConversionError::WrongType {
                        src_value: format!("{:?}", item), // i.e. Bool(true)
                        src_type: format!("{}{}", VAL_ENUM_NAME, item), // i.e. Value::Bool, where 'Bool' is generated by strum through the display trait
                        target_type: String::from(stringify!($for_type)),
                        opt_info: None,
                    })),
                }
            }
        }
    };
}
impl_try_from_value_for_type!(String, String);
impl_try_from_value_for_type!(Int8, i8);
impl_try_from_value_for_type!(Int16, i16);
impl_try_from_value_for_type!(Int32, i32);
impl_try_from_value_for_type!(Int64, i64);
impl_try_from_value_for_type!(Int128, i128);
impl_try_from_value_for_type!(UInt8, u8);
impl_try_from_value_for_type!(UInt16, u16);
impl_try_from_value_for_type!(UInt32, u32);
impl_try_from_value_for_type!(UInt64, u64);
impl_try_from_value_for_type!(UInt128, u128);
impl_try_from_value_for_type!(Float32, f32);
impl_try_from_value_for_type!(Float64, f64);
impl_try_from_value_for_type!(Bool, bool);
impl_try_from_value_for_type!(Decimal, Decimal);
impl_try_from_value_for_type!(NaiveDate, NaiveDate);
impl_try_from_value_for_type!(NaiveDateTime, NaiveDateTime);
impl_try_from_value_for_type!(DateTime, DateTime<FixedOffset>);

macro_rules! from_type_string {
    ($fn_name:ident, $enum_type:ident, $for_type:ty) => {
        pub fn $fn_name(v: &str) -> Result<Value> {
            let temp = v.parse::<$for_type>().map_err(|_| {
                VenumError::Parsing(ParseError::ValueFromStringFailed {
                    src_value: String::from(v),
                    target_type: format!("{}{}", VAL_ENUM_NAME, stringify!($enum_type)),
                    opt_info: None,
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
        Value::DateTime(DateTime::parse_from_rfc3339("1970-01-01T00:00:00+00:00").unwrap())
    }

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
    from_type_string!(parse_float32_from_str, Float32, f32);
    from_type_string!(parse_float64_from_str, Float64, f64);
    from_type_string!(parse_bool_from_str, Bool, bool);
    pub fn parse_decimal_from_str(v: &str) -> Result<Value> {
        let temp = Decimal::from_str_exact(v).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, "Decimal"),
                opt_info: Some(format!("Original error: {oe}")),
            })
        })?;
        Ok(Value::Decimal(temp))
    }
    pub fn parse_naive_date_from_str(v: &str, chrono_pattern: &str) -> Result<Value> {
        // e.g pattern "%Y-%m-%d" to parse "2015-09-05"
        let temp = NaiveDate::parse_from_str(v, chrono_pattern).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, ENUM_VAR_ND),
                opt_info: Some(format!(
                    "Chrono pattern: {chrono_pattern}. Original error: {oe}"
                )),
            })
        })?;
        Ok(Value::NaiveDate(temp))
    }
    pub fn parse_naive_date_from_str_iso8601_ymd(v: &str) -> Result<Value> {
        // e.g pattern "%F" (which is "%Y-%m-%d") to parse "2015-09-05"
        let chrono_pattern = "%F";
        let temp = NaiveDate::parse_from_str(v, chrono_pattern).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, ENUM_VAR_ND),
                opt_info: Some(format!(
                    "Chrono pattern: {chrono_pattern}. Original error: {oe}"
                )),
            })
        })?;
        Ok(Value::NaiveDate(temp))
    }
    pub fn parse_naive_date_time_from_str(v: &str, chrono_pattern: &str) -> Result<Value> {
        // e.g pattern "%F %T" (which is "%Y-%m-%d %H:%M:%S") to parse "2015-09-05 23:56:04"
        let temp = NaiveDateTime::parse_from_str(v, chrono_pattern).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, ENUM_VAR_NDT),
                opt_info: Some(format!(
                    "Chrono pattern: {chrono_pattern}. Original error: {oe}"
                )),
            })
        })?;
        Ok(Value::NaiveDateTime(temp))
    }
    pub fn parse_naive_date_time_from_str_iso8601_ymdhms(v: &str) -> Result<Value> {
        // e.g pattern "%F %T" (which is "%Y-%m-%d %H:%M:%S") to parse "2015-09-05 23:56:04"
        let chrono_pattern = "%F %T";
        let temp = NaiveDateTime::parse_from_str(v, chrono_pattern).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, ENUM_VAR_NDT),
                opt_info: Some(format!(
                    "Chrono pattern: {chrono_pattern}. Original error: {oe}"
                )),
            })
        })?;
        Ok(Value::NaiveDateTime(temp))
    }
    pub fn parse_date_time_from_str(v: &str, chrono_pattern: &str) -> Result<Value> {
        let temp = DateTime::parse_from_str(v, chrono_pattern).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, ENUM_VAR_DT),
                opt_info: Some(format!(
                    "Chrono pattern: {chrono_pattern}. Original error: {oe}"
                )),
            })
        })?;
        Ok(Value::DateTime(temp))
    }
    pub fn parse_date_time_from_str_rfc2822(v: &str) -> Result<Value> {
        // e.g date as: "Tue, 1 Jul 2003 10:52:37 +0200"
        let temp = DateTime::parse_from_rfc2822(v).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, ENUM_VAR_DT),
                opt_info: Some(format!("Original error: {oe}")),
            })
        })?;
        Ok(Value::DateTime(temp))
    }
    pub fn parse_date_time_from_str_rfc3339(v: &str) -> Result<Value> {
        // e.g date as: "1996-12-19T16:39:57-08:00"
        let temp = DateTime::parse_from_rfc3339(v).map_err(|oe| {
            VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(v),
                target_type: format!("{}{}", VAL_ENUM_NAME, ENUM_VAR_DT),
                opt_info: Some(format!("Original error: {oe}")),
            })
        })?;
        Ok(Value::DateTime(temp))
    }

    pub fn decimal_from_i8(v: i8) -> Value {
        Value::Decimal(Decimal::from_i16(v as i16).unwrap()) // I can't think of a case where an i8 cannot be represented by a decimal
    }
    pub fn decimal_from_i16(v: i16) -> Value {
        Value::Decimal(Decimal::from_i16(v).unwrap()) // I can't think of a case where an i16 cannot be represented by a decimal
    }
    pub fn decimal_from_i32(v: i32) -> Value {
        Value::Decimal(Decimal::from_i32(v).unwrap()) // I can't think of a case where an i32 cannot be represented by a decimal
    }
    pub fn decimal_from_i64(v: i64) -> Value {
        Value::Decimal(Decimal::from_i64(v).unwrap()) // I can't think of a case where an i64 cannot be represented by a decimal
    }
    pub fn decimal_from_i128(v: i128) -> Value {
        Value::Decimal(Decimal::from_i128(v).unwrap()) // I can't think of a case where an i128 cannot be represented by a decimal
    }
    pub fn decimal_from_u8(v: u8) -> Value {
        Value::Decimal(Decimal::from_u16(v as u16).unwrap()) // I can't think of a case where an i8 cannot be represented by a decimal
    }
    pub fn decimal_from_u16(v: u16) -> Value {
        Value::Decimal(Decimal::from_u16(v).unwrap()) // I can't think of a case where an i16 cannot be represented by a decimal
    }
    pub fn decimal_from_u32(v: u32) -> Value {
        Value::Decimal(Decimal::from_u32(v).unwrap()) // I can't think of a case where an i32 cannot be represented by a decimal
    }
    pub fn decimal_from_u64(v: u64) -> Value {
        Value::Decimal(Decimal::from_u64(v).unwrap()) // I can't think of a case where an i64 cannot be represented by a decimal
    }
    pub fn decimal_from_u128(v: u128) -> Value {
        Value::Decimal(Decimal::from_u128(v).unwrap()) // I can't think of a case where an i128 cannot be represented by a decimal
    }
    pub fn decimal_from_f32(v: f32) -> Value {
        Value::Decimal(Decimal::from_f32(v).unwrap()) // I can't think of a case where a f32 cannot be represented by a decimal
    }
    pub fn decimal_from_f64(v: i64) -> Value {
        Value::Decimal(Decimal::from_i64(v).unwrap()) // I can't think of a case where a f64 cannot be represented by a decimal
    }

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

    pub fn get_default_of_self_variant(&self) -> Value {
        match self {
            Value::String(_) => Value::string_default(),
            Value::Int8(_) => Value::int8_default(),
            Value::Int16(_) => Value::int16_default(),
            Value::Int32(_) => Value::int32_default(),
            Value::Int64(_) => Value::int64_default(),
            Value::Int128(_) => Value::int128_default(),
            Value::UInt8(_) => Value::uint8_default(),
            Value::UInt16(_) => Value::uint16_default(),
            Value::UInt32(_) => Value::uint32_default(),
            Value::UInt64(_) => Value::uint64_default(),
            Value::UInt128(_) => Value::uint128_default(),
            Value::Float32(_) => Value::float32_default(),
            Value::Float64(_) => Value::float64_default(),
            Value::Bool(_) => Value::bool_default(),
            Value::Decimal(_) => Value::decimal_default(),
            Value::NaiveDate(_) => Value::naive_date_default(),
            Value::NaiveDateTime(_) => Value::naive_date_time_default(),
            Value::DateTime(_) => Value::date_time_default(),
        }
    }

    /// NOTE: We decided agains Option<String> here as the type of the value since the intention is to create a typed version of a stringy-input we read from some CSV.
    ///       In that case, when a CSV column contains a "" as an entry, e.g. like this: `a,,c` or this `"a","","c"`, where the middle column would translate to empty / "",
    ///       we map it to a None internally, representing the absence of data.
    /// NOTE2: For date types (NaiveDate, NaiveDateTime, DateTime) only the most common cases (iso8601_ymd, iso8601_ymdhms and rfc3339) have been implemented. Every other
    ///        format _will_ error!
    pub fn from_string_with_templ(value: &str, templ_type: &Value) -> Result<Option<Value>> {
        if value == "" || value.to_lowercase() == "null" {
            return Ok(None);
        }
        match templ_type {
            Value::String(_) => Ok(Some(Value::String(value.into()))), // even a string value of "" will be a real value, since it's not explicitly None (...i.e. coming from a "null")

            Value::Int8(_) => Ok(Some(Value::parse_int8_from_str(value)?)),
            Value::Int16(_) => Ok(Some(Value::parse_int16_from_str(value)?)),
            Value::Int32(_) => Ok(Some(Value::parse_int32_from_str(value)?)),
            Value::Int64(_) => Ok(Some(Value::parse_int64_from_str(value)?)),
            Value::Int128(_) => Ok(Some(Value::parse_int128_from_str(value)?)),

            Value::UInt8(_) => Ok(Some(Value::parse_uint8_from_str(value)?)),
            Value::UInt16(_) => Ok(Some(Value::parse_uint16_from_str(value)?)),
            Value::UInt32(_) => Ok(Some(Value::parse_uint32_from_str(value)?)),
            Value::UInt64(_) => Ok(Some(Value::parse_uint64_from_str(value)?)),
            Value::UInt128(_) => Ok(Some(Value::parse_uint128_from_str(value)?)),

            Value::Float32(_) => Ok(Some(Value::parse_float32_from_str(value)?)),
            Value::Float64(_) => Ok(Some(Value::parse_float64_from_str(value)?)),

            Value::Bool(_) => Ok(Some(Value::parse_bool_from_str(value)?)),
            Value::Decimal(_) => Ok(Some(Value::parse_decimal_from_str(value)?)),

            Value::NaiveDate(_) => Ok(Some(Value::parse_naive_date_from_str_iso8601_ymd(value)?)),
            Value::NaiveDateTime(_) => Ok(Some(
                Value::parse_naive_date_time_from_str_iso8601_ymdhms(value)?,
            )),
            Value::DateTime(_) => Ok(Some(Value::parse_date_time_from_str_rfc3339(value)?)),
        }
    }

    pub fn datetype_from_string_with_templ_and_chrono_pattern(
        value: &str,
        templ_type: &Value,
        chrono_pattern: &str,
    ) -> Result<Option<Value>> {
        if value == "" || value.to_lowercase() == "null" {
            return Ok(None);
        }
        match templ_type {
            Value::NaiveDate(_) => Ok(Some(Value::parse_naive_date_from_str(
                value,
                chrono_pattern,
            )?)),
            Value::NaiveDateTime(_) => Ok(Some(Value::parse_naive_date_time_from_str(
                value,
                chrono_pattern,
            )?)),
            Value::DateTime(_) => Ok(Some(Value::parse_date_time_from_str(
                value,
                chrono_pattern,
            )?)),
            _ => Err(VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from(value),
                target_type: format!("{}{}", VAL_ENUM_NAME, templ_type),
                opt_info: Some(format!("Chrono pattern: {chrono_pattern}")),
            })),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{FixedOffset, TimeZone};

    // ----------------------------------------------------------------------------------------------------------------------------------------
    // --------------------------- from_type_string (i.e. all parse_*_from_str functions, except Decimal and Dates) ---------------------------
    // ----------------------------------------------------------------------------------------------------------------------------------------
    #[test]
    pub fn test_parse_int8_from_str() {
        assert_eq!(Ok(Value::Int8(-1)), Value::parse_int8_from_str("-1"));
    }

    #[test]
    pub fn test_parse_int16_from_str() {
        assert_eq!(Ok(Value::Int16(-1)), Value::parse_int16_from_str("-1"));
    }

    #[test]
    pub fn test_parse_int32_from_str() {
        assert_eq!(Ok(Value::Int32(-1)), Value::parse_int32_from_str("-1"));
    }

    #[test]
    pub fn test_parse_int32_from_str_err() {
        let err_res = Value::parse_int32_from_str("foobar");

        let exp = Err(VenumError::Parsing(ParseError::ValueFromStringFailed {
            src_value: String::from("foobar"),
            target_type: String::from("Value::Int32"),
            opt_info: None,
        }));
        assert_eq!(exp, err_res);
    }

    #[test]
    pub fn test_parse_int64_from_str() {
        assert_eq!(Ok(Value::Int64(-1)), Value::parse_int64_from_str("-1"));
    }

    #[test]
    pub fn test_parse_int128_from_str() {
        assert_eq!(Ok(Value::Int128(-1)), Value::parse_int128_from_str("-1"));
    }

    #[test]
    pub fn test_parse_uint8_from_str() {
        assert_eq!(
            Ok(Value::UInt8(1)),
            Value::parse_uint8_from_str(&String::from("1"))
        );
    }

    #[test]
    pub fn test_parse_uint16_from_str() {
        assert_eq!(Ok(Value::UInt16(1)), Value::parse_uint16_from_str("1"));
    }

    #[test]
    pub fn test_parse_uint32_from_str() {
        assert_eq!(Ok(Value::UInt32(1)), Value::parse_uint32_from_str("1"));
    }

    #[test]
    pub fn test_parse_uint64_from_str() {
        assert_eq!(Ok(Value::UInt64(1)), Value::parse_uint64_from_str("1"));
    }

    #[test]
    pub fn test_parse_uint128_from_str() {
        assert_eq!(Ok(Value::UInt128(1)), Value::parse_uint128_from_str("1"));
    }

    #[test]
    pub fn test_parse_float32_from_str() {
        assert_eq!(Ok(Value::Float32(1.0)), Value::parse_float32_from_str("1"));
    }

    #[test]
    pub fn test_parse_float64_from_str() {
        assert_eq!(Ok(Value::Float64(1.0)), Value::parse_float64_from_str("1"));
    }

    #[test]
    pub fn test_parse_bool_from_str() {
        assert_eq!(Ok(Value::Bool(true)), Value::parse_bool_from_str("true"));
    }

    #[test]
    #[should_panic(
        expected = "Parsing(ValueFromStringFailed { src_value: \"1.123\", target_type: \"Value::Int32\", opt_info: None })"
    )]
    pub fn test_parse_i32_from_str_err() {
        Value::parse_int32_from_str("1.123").unwrap();
    }

    #[test]
    #[should_panic(
        expected = "Parsing(ValueFromStringFailed { src_value: \"1.123\", target_type: \"Value::UInt32\", opt_info: None })"
    )]
    pub fn test_parse_u32_from_str_err() {
        Value::parse_uint32_from_str("1.123").unwrap();
    }

    #[test]
    #[should_panic(
        expected = "Parsing(ValueFromStringFailed { src_value: \"foobar\", target_type: \"Value::Float32\", opt_info: None })"
    )]
    pub fn test_parse_f32_from_str_err() {
        Value::parse_float32_from_str("foobar").unwrap();
    }

    #[test]
    #[should_panic(
        expected = "Parsing(ValueFromStringFailed { src_value: \"foobar\", target_type: \"Value::Bool\", opt_info: None })"
    )]
    pub fn test_parse_bool_from_str_err() {
        Value::parse_bool_from_str("foobar").unwrap();
    }

    // ----------------------------------------------------------------------------------------------------------------------------------------
    // ---------------------------------------------- Decimal and Date (parse_*_from_str) testing ---------------------------------------------
    // ----------------------------------------------------------------------------------------------------------------------------------------
    #[test]
    pub fn test_parse_decimal_from_str() {
        assert_eq!(
            Ok(Value::Decimal(Decimal::new(1123, 3))),
            Value::parse_decimal_from_str("1.123")
        );
    }

    #[test]
    #[should_panic(
        expected = "Parsing(ValueFromStringFailed { src_value: \"foobar\", target_type: \"Value::Decimal\", opt_info: Some(\"Original error: Invalid decimal: unknown character\") })"
    )]
    pub fn test_parse_decimal_from_str_err() {
        Value::parse_decimal_from_str("foobar").unwrap();
    }

    #[test]
    pub fn test_parse_naive_date_from_str_w_pattern() {
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
        expected = "Parsing(ValueFromStringFailed { src_value: \"2022-12-31 00:00\", target_type: \"Value::NaiveDate\", opt_info: Some(\"Chrono pattern: %Y-%m-%d. Original error: trailing input\") })"
    )]
    pub fn test_parse_naive_date_from_str_w_pattern_err_trailing_inp() {
        Value::parse_naive_date_from_str("2022-12-31 00:00", "%Y-%m-%d").unwrap();
    }

    #[test]
    #[should_panic(
        expected = "Parsing(ValueFromStringFailed { src_value: \"2022-12-31\", target_type: \"Value::NaiveDate\", opt_info: Some(\"Chrono pattern: %Y %m %d. Original error: input contains invalid characters\") })"
    )]
    pub fn test_parse_naive_date_from_str_w_pattern_err_invalid_chars() {
        Value::parse_naive_date_from_str("2022-12-31", "%Y %m %d").unwrap();
    }

    #[test]
    pub fn test_parse_naive_date_from_str_iso8601_ymd() {
        let exp = NaiveDate::from_ymd(2022, 12, 31);

        assert_eq!(
            Ok(Value::NaiveDate(exp)),
            Value::parse_naive_date_from_str_iso8601_ymd("2022-12-31")
        );
    }

    #[test]
    pub fn test_parse_naive_date_time_from_str_w_pattern() {
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
        expected = "Parsing(ValueFromStringFailed { src_value: \"2022-12-31 12:11:10 000\", target_type: \"Value::NaiveDateTime\", opt_info: Some(\"Chrono pattern: %Y-%m-%d %H:%M:%S. Original error: trailing input\") }"
    )]
    pub fn test_parse_naive_date_time_from_str_w_pattern_err_trailing_inp() {
        Value::parse_naive_date_time_from_str("2022-12-31 12:11:10 000", "%Y-%m-%d %H:%M:%S")
            .unwrap();
    }

    #[test]
    #[should_panic(
        expected = "Parsing(ValueFromStringFailed { src_value: \"2022-12-31 12:11:10\", target_type: \"Value::NaiveDateTime\", opt_info: Some(\"Chrono pattern: %Y-%m-%dT%H:%M:%S. Original error: input contains invalid characters\") }"
    )]
    pub fn test_parse_naive_date_time_from_str_w_pattern_err_invalid_chars() {
        Value::parse_naive_date_time_from_str("2022-12-31 12:11:10", "%Y-%m-%dT%H:%M:%S").unwrap();
    }

    #[test]
    pub fn test_parse_naive_date_time_from_str_iso8601_ymdhms() {
        let exp = NaiveDate::from_ymd(2022, 12, 31).and_hms(12, 11, 10);

        assert_eq!(
            Ok(Value::NaiveDateTime(exp)),
            Value::parse_naive_date_time_from_str_iso8601_ymdhms("2022-12-31 12:11:10")
        );
    }

    #[test]
    pub fn test_parse_date_time_from_str_w_pattern() {
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
        expected = "Parsing(ValueFromStringFailed { src_value: \"2022-12-31T06:00:00\", target_type: \"Value::DateTime\", opt_info: Some(\"Chrono pattern: %FT%T%:z. Original error: premature end of input\") }"
    )]
    pub fn test_parse_date_time_from_str_w_pattern_err_prem_end_of_input() {
        Value::parse_date_time_from_str("2022-12-31T06:00:00", "%FT%T%:z").unwrap();
    }

    #[test]
    #[should_panic(
        expected = "Parsing(ValueFromStringFailed { src_value: \"2022-12-31T06:00:00\", target_type: \"Value::DateTime\", opt_info: Some(\"Chrono pattern: %FT%T. Original error: input is not enough for unique date and time\") })"
    )]
    pub fn test_parse_date_time_from_str_w_pattern_err_invalid_chars() {
        Value::parse_date_time_from_str("2022-12-31T06:00:00", "%FT%T").unwrap();
    }

    #[test]
    #[should_panic(
        expected = "Parsing(ValueFromStringFailed { src_value: \"2022-12-31T06:00:00+05:00\", target_type: \"Value::DateTime\", opt_info: Some(\"Chrono pattern: %FT%T%. Original error: bad or unsupported format string\") }"
    )]
    pub fn test_parse_date_time_from_str_w_pattern_err_bad_format_string() {
        Value::parse_date_time_from_str("2022-12-31T06:00:00+05:00", "%FT%T%").unwrap();
    }

    #[test]
    pub fn test_parse_date_time_from_str_rfc3339() {
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
    pub fn test_parse_date_time_from_str_rfc2822() {
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

    // ----------------------------------------------------------------------------------------------------------------------------------------
    // --------------------------------------- Testing that the default values are the expected ones ------------------------------------------
    // ----------------------------------------------------------------------------------------------------------------------------------------
    #[test]
    pub fn test_string_default() {
        assert_eq!(Value::String("".to_string()), Value::string_default());
    }

    #[test]
    pub fn test_int8_default() {
        assert_eq!(Value::Int8(0), Value::int8_default());
    }

    #[test]
    pub fn test_int16_default() {
        assert_eq!(Value::Int16(0), Value::int16_default());
    }

    #[test]
    pub fn test_int32_default() {
        assert_eq!(Value::Int32(0), Value::int32_default());
    }

    #[test]
    pub fn test_int64_default() {
        assert_eq!(Value::Int64(0), Value::int64_default());
    }

    #[test]
    pub fn test_int128_default() {
        assert_eq!(Value::Int128(0), Value::int128_default());
    }

    #[test]
    pub fn test_uint8_default() {
        assert_eq!(Value::UInt8(0), Value::uint8_default());
    }

    #[test]
    pub fn test_uint16_default() {
        assert_eq!(Value::UInt16(0), Value::uint16_default());
    }

    #[test]
    pub fn test_uint32_default() {
        assert_eq!(Value::UInt32(0), Value::uint32_default());
    }

    #[test]
    pub fn test_uint64_default() {
        assert_eq!(Value::UInt64(0), Value::uint64_default());
    }

    #[test]
    pub fn test_uint128_default() {
        assert_eq!(Value::UInt128(0), Value::uint128_default());
    }

    #[test]
    pub fn test_float32_default() {
        assert_eq!(Value::Float32(0.0), Value::float32_default());
    }

    #[test]
    pub fn test_float64_default() {
        assert_eq!(Value::Float64(0.0), Value::float64_default());
    }

    #[test]
    pub fn test_bool_default() {
        assert_eq!(Value::Bool(false), Value::bool_default());
    }

    #[test]
    pub fn test_decimal_default() {
        assert_eq!(
            Value::Decimal(Decimal::new(00, 1)),
            Value::decimal_default()
        );
    }

    #[test]
    pub fn test_naive_date_default() {
        assert_eq!(
            Value::NaiveDate(NaiveDate::from_ymd(1970, 01, 01)),
            Value::naive_date_default()
        );
    }

    #[test]
    pub fn test_naive_date_time_default() {
        assert_eq!(
            Value::NaiveDateTime(NaiveDate::from_ymd(1970, 01, 01).and_hms(00, 00, 00)),
            Value::naive_date_time_default()
        );
    }

    #[test]
    pub fn test_date_time_default() {
        let exp: DateTime<FixedOffset> = FixedOffset::east(0) // east = +; west = -
            .ymd(1970, 01, 01)
            .and_hms(0, 0, 0);
        assert_eq!(Value::DateTime(exp), Value::date_time_default());
    }

    // ----------------------------------------------------------------------------------------------------------------------------------------
    // ------------------------------------------------- Testing conversions (from impls) -----------------------------------------------------
    // ----------------------------------------------------------------------------------------------------------------------------------------
    #[test]
    pub fn test_impl_from_type_for_value() {
        assert_eq!(
            Value::String(String::from("foobar")),
            Value::from(String::from("foobar"))
        );

        assert_eq!(Value::Int8(0), Value::from(0i8));
        assert_eq!(Value::Int16(0), Value::from(0i16));
        assert_eq!(Value::Int32(0), Value::from(0i32));
        assert_eq!(Value::Int64(0), Value::from(0i64));
        assert_eq!(Value::Int128(0), Value::from(0i128));

        assert_eq!(Value::UInt8(0), Value::from(0u8));
        assert_eq!(Value::UInt16(0), Value::from(0u16));
        assert_eq!(Value::UInt32(0), Value::from(0u32));
        assert_eq!(Value::UInt64(0), Value::from(0u64));
        assert_eq!(Value::UInt128(0), Value::from(0u128));

        assert_eq!(Value::Float32(0.0), Value::from(0.0f32));
        assert_eq!(Value::Float64(0.0), Value::from(0.0f64));

        assert_eq!(Value::Bool(true), Value::from(true));

        assert_eq!(
            Value::Decimal(Decimal::new(00, 1)),
            Value::from(Decimal::new(00, 1))
        );

        assert_eq!(
            Value::NaiveDate(NaiveDate::from_ymd(2022, 12, 31)),
            Value::from(NaiveDate::from_ymd(2022, 12, 31))
        );
        assert_eq!(
            Value::NaiveDateTime(NaiveDate::from_ymd(2022, 12, 31).and_hms(12, 0, 0)),
            Value::from(NaiveDate::from_ymd(2022, 12, 31).and_hms(12, 0, 0))
        );

        let dt: DateTime<FixedOffset> = FixedOffset::east(5 * 3600) // east = +; west = -
            .ymd(2022, 12, 31)
            .and_hms(6, 0, 0);
        assert_eq!(Value::DateTime(dt), Value::from(dt));
    }

    #[test]
    pub fn test_impl_try_from_value_for_type() {
        assert_eq!(
            String::from("foobar"),
            String::try_from(Value::String(String::from("foobar"))).unwrap()
        );

        assert_eq!(0i8, i8::try_from(Value::Int8(0i8)).unwrap());
        assert_eq!(0i16, i16::try_from(Value::Int16(0i16)).unwrap());
        assert_eq!(0i32, i32::try_from(Value::Int32(0i32)).unwrap());
        assert_eq!(0i64, i64::try_from(Value::Int64(0i64)).unwrap());
        assert_eq!(0i128, i128::try_from(Value::Int128(0i128)).unwrap());

        assert_eq!(0u8, u8::try_from(Value::UInt8(0u8)).unwrap());
        assert_eq!(0u16, u16::try_from(Value::UInt16(0u16)).unwrap());
        assert_eq!(0u32, u32::try_from(Value::UInt32(0u32)).unwrap());
        assert_eq!(0u64, u64::try_from(Value::UInt64(0u64)).unwrap());
        assert_eq!(0u128, u128::try_from(Value::UInt128(0u128)).unwrap());

        assert_eq!(0.0f32, f32::try_from(Value::Float32(0.0f32)).unwrap());
        assert_eq!(0.0f64, f64::try_from(Value::Float64(0.0f64)).unwrap());

        assert_eq!(true, bool::try_from(Value::Bool(true)).unwrap());

        assert_eq!(
            Decimal::new(00, 1),
            Decimal::try_from(Value::Decimal(Decimal::new(00, 1))).unwrap()
        );

        assert_eq!(
            NaiveDate::from_ymd(2022, 12, 31),
            NaiveDate::try_from(Value::NaiveDate(NaiveDate::from_ymd(2022, 12, 31))).unwrap()
        );
        assert_eq!(
            NaiveDate::from_ymd(2022, 12, 31).and_hms(12, 00, 00),
            NaiveDateTime::try_from(Value::NaiveDateTime(
                NaiveDate::from_ymd(2022, 12, 31).and_hms(12, 00, 00)
            ))
            .unwrap()
        );

        let dt: DateTime<FixedOffset> = FixedOffset::east(5 * 3600) // east = +; west = -
            .ymd(2022, 12, 31)
            .and_hms(6, 0, 0);
        assert_eq!(dt, DateTime::try_from(Value::DateTime(dt)).unwrap());
    }

    #[test]
    #[should_panic(
        expected = "Conversion(WrongType { src_value: \"Int8(0)\", src_type: \"Value::Int8\", target_type: \"bool\", opt_info: None })"
    )]
    pub fn string_to_bool_err() {
        bool::try_from(Value::Int8(0i8)).unwrap();
    }

    #[test]
    pub fn int8_from_str_and_templ_ok() {
        let test = Value::from_string_with_templ("10", &Value::int8_default());
        assert_eq!(Ok(Some(Value::Int8(10))), test);
    }

    #[test]
    #[should_panic(
        expected = "Parsing(ValueFromStringFailed { src_value: \"false\", target_type: \"Value::Int8\", opt_info: None })"
    )]
    pub fn int8_from_str_and_templ_err() {
        Value::from_string_with_templ("false", &Value::int8_default()).unwrap();
    }
}
