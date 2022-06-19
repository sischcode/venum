use crate::errors::{ParseError, Result, VenumError};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Utc};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use std::convert::From;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    String(String), // TODO: do we need Char(char) as well?
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
    ($enum_type:ident, $type:ty) => {
        impl From<$type> for Value {
            fn from(item: $type) -> Self {
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

macro_rules! impl_from_value_for_result {
    ($enum_type:ident, $type:ty) => {
        impl From<Value> for Result<$type> {
            fn from(item: Value) -> Self {
                match item {
                    Value::$enum_type(v) => Ok(v),
                    _ => Err(VenumError::Conversion(ParseError::UnwrapToBaseTypeFailed {
                        src_value: format!("{:?}", item),
                        basic_type: stringify!($type),
                    })),
                }
            }
        }
    };
}
impl_from_value_for_result!(String, String);
impl_from_value_for_result!(Int8, i8);
impl_from_value_for_result!(Int16, i16);
impl_from_value_for_result!(Int32, i32);
impl_from_value_for_result!(Int64, i64);
impl_from_value_for_result!(Int128, i128);
impl_from_value_for_result!(UInt8, u8);
impl_from_value_for_result!(UInt16, u16);
impl_from_value_for_result!(UInt32, u32);
impl_from_value_for_result!(UInt64, u64);
impl_from_value_for_result!(UInt128, u128);
impl_from_value_for_result!(Float32, f32);
impl_from_value_for_result!(Float64, f64);
impl_from_value_for_result!(Bool, bool);
impl_from_value_for_result!(Decimal, Decimal);
impl_from_value_for_result!(NaiveDate, NaiveDate);
impl_from_value_for_result!(NaiveDateTime, NaiveDateTime);
impl_from_value_for_result!(DateTime, DateTime<FixedOffset>);

macro_rules! from_type_string {
    ($fn_name:ident, $enum_type:ident, $type:ty) => {
        pub fn $fn_name(v: &str) -> Result<Value> {
            let temp = v.parse::<$type>().map_err(|_| {
                VenumError::Conversion(ParseError::ValueFromStringFailed {
                    src_value: v.to_string(),
                    target_type: stringify!($enum_type),
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
    pub fn naive_date_default() -> Value {
        Value::NaiveDate(NaiveDate::from_ymd(1970, 1, 1))
    }
    pub fn naive_date_time_default() -> Value {
        Value::NaiveDateTime(NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0))
    }
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
            VenumError::Conversion(ParseError::ValueFromStringFailed {
                src_value: v.to_string(),
                target_type: "Decimal",
                opt_info: Some(format!("Original error: {oe}")),
            })
        })?;
        Ok(Value::Decimal(temp))
    }

    pub fn parse_naive_date_from_str(v: &str, chrono_pattern: &str) -> Result<Value> {
        // e.g pattern "%Y-%m-%d" to parse "2015-09-05"
        let temp = NaiveDate::parse_from_str(v, chrono_pattern).map_err(|oe| {
            VenumError::Conversion(ParseError::ValueFromStringFailed {
                src_value: v.to_string(),
                target_type: "NaiveDate",
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
            VenumError::Conversion(ParseError::ValueFromStringFailed {
                src_value: v.to_string(),
                target_type: "NaiveDate",
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
            VenumError::Conversion(ParseError::ValueFromStringFailed {
                src_value: v.to_string(),
                target_type: "NaiveDateTime",
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
            VenumError::Conversion(ParseError::ValueFromStringFailed {
                src_value: v.to_string(),
                target_type: "NaiveDateTime",
                opt_info: Some(format!(
                    "Chrono pattern: {chrono_pattern}. Original error: {oe}"
                )),
            })
        })?;
        Ok(Value::NaiveDateTime(temp))
    }
    pub fn parse_date_time_from_str(v: &str, chrono_pattern: &str) -> Result<Value> {
        // e.g pattern "%Y-%m-%d %H:%M:%S" to parse "2015-09-05 23:56:04"
        let temp = DateTime::parse_from_str(v, chrono_pattern).map_err(|oe| {
            VenumError::Conversion(ParseError::ValueFromStringFailed {
                src_value: v.to_string(),
                target_type: "DateTime",
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
            VenumError::Conversion(ParseError::ValueFromStringFailed {
                src_value: v.to_string(),
                target_type: "DateTime",
                opt_info: Some(format!("Original error: {oe}")),
            })
        })?;
        Ok(Value::DateTime(temp))
    }
    pub fn parse_date_time_from_str_rfc3339(v: &str) -> Result<Value> {
        // e.g date as: "1996-12-19T16:39:57-08:00"
        let temp = DateTime::parse_from_rfc3339(v).map_err(|oe| {
            VenumError::Conversion(ParseError::ValueFromStringFailed {
                src_value: v.to_string(),
                target_type: "DateTime",
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

    pub fn get_default_of_self(&self) -> Value {
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
    pub fn from_string_with_templ(value: &str, templ_type: &Value) -> Result<Option<Value>> {
        if value == "" {
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
            //TODO
            _ => Ok(Some(Value::String("".to_string()))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // pub fn crhono() {
    //     let nd: NaiveDate = NaiveDate::from_ymd(2016, 7, 8);
    //     let ndt: NaiveDateTime = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);

    //     let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
    //     let dt2 = FixedOffset::Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);
    //     assert_eq!(dt, dt2);

    //     let rfc3339 = DateTime::parse_from_rfc3339("1970-01-01T00:00:00+00:00")?;
    // }

    #[test]
    pub fn test_int8_from_str() {
        assert_eq!(
            Ok(Value::Int8(-1)),
            Value::parse_int8_from_str(&String::from("-1"))
        );
    }
    #[test]
    pub fn test_int16_from_str() {
        assert_eq!(Ok(Value::Int16(-1)), Value::parse_int16_from_str("-1"));
    }
    #[test]
    pub fn test_int32_from_str() {
        assert_eq!(Ok(Value::Int32(-1)), Value::parse_int32_from_str("-1"));
    }
    #[test]
    pub fn test_int64_from_str() {
        assert_eq!(Ok(Value::Int64(-1)), Value::parse_int64_from_str("-1"));
    }
    #[test]
    pub fn test_int128_from_str() {
        assert_eq!(Ok(Value::Int128(-1)), Value::parse_int128_from_str("-1"));
    }
    #[test]
    pub fn test_uint8_from_str() {
        assert_eq!(
            Ok(Value::UInt8(1)),
            Value::parse_uint8_from_str(&String::from("1"))
        );
    }
    #[test]
    pub fn test_uint16_from_str() {
        assert_eq!(Ok(Value::UInt16(1)), Value::parse_uint16_from_str("1"));
    }
    #[test]
    pub fn test_uint32_from_str() {
        assert_eq!(Ok(Value::UInt32(1)), Value::parse_uint32_from_str("1"));
    }
    #[test]
    pub fn test_uint64_from_str() {
        assert_eq!(Ok(Value::UInt64(1)), Value::parse_uint64_from_str("1"));
    }
    #[test]
    pub fn test_uint128_from_str() {
        assert_eq!(Ok(Value::UInt128(1)), Value::parse_uint128_from_str("1"));
    }

    #[test]
    pub fn test_float32_from_str() {
        assert_eq!(Ok(Value::Float32(1.0)), Value::parse_float32_from_str("1"));
    }
    #[test]
    pub fn test_float64_from_str() {
        assert_eq!(Ok(Value::Float64(1.0)), Value::parse_float64_from_str("1"));
    }

    #[test]
    pub fn test_bool_from_str() {
        assert_eq!(Ok(Value::Bool(true)), Value::parse_bool_from_str("true"));
    }

    #[test]
    pub fn test_decimal_from_str() {
        assert_eq!(
            Ok(Value::Decimal(Decimal::new(1123, 3))),
            Value::parse_decimal_from_str("1.123")
        );
    }

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
    pub fn string_to_bool_err() {
        let res: Result<bool> = Value::String("test_data".into()).into();
        let exp = Err(VenumError::Conversion(ParseError::UnwrapToBaseTypeFailed {
            src_value: "String(\"test_data\")".into(),
            basic_type: "bool",
        }));
        assert_eq!(exp, res);
    }

    #[test]
    pub fn int8_from_str_and_templ_ok() {
        let test = Value::from_string_with_templ("10", &Value::int8_default());
        assert_eq!(Ok(Some(Value::Int8(10))), test);
    }

    #[test]
    pub fn int8_from_str_and_templ_err() {
        let test = Value::from_string_with_templ("false", &Value::int8_default());
        assert_eq!(
            Err(VenumError::Conversion(ParseError::ValueFromStringFailed {
                src_value: "false".to_string(),
                target_type: "Int8",
                opt_info: None
            })),
            test
        );
    }
}
