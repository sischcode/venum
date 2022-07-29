use std::convert::From;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use strum_macros::Display; // used to generate names for the enum variants. Used only for error messages (as of now).

use crate::errors::{ConversionError, ParseError, Result, VenumError};

const VAL_ENUM_NAME: &str = "Value::";
const ENUM_VAR_ND: &str = "NaiveDate";
const ENUM_VAR_NDT: &str = "NaiveDateTime";
const ENUM_VAR_DT: &str = "DateTime";

#[derive(Default, Display, Debug, Clone, PartialEq, PartialOrd)]
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

macro_rules! impl_from_type_for_value {
    ($enum_type:ident, $from_type:ty) => {
        impl From<$from_type> for Value {
            fn from(item: $from_type) -> Self {
                Value::$enum_type(item)
            }
        }
    };
}
impl_from_type_for_value!(Char, char);
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
                        details: None,
                    })),
                }
            }
        }
    };
}
impl_try_from_value_for_type!(Char, char);
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

macro_rules! impl_try_from_value_ref_for_copy_type {
    ($enum_type:ident, $for_type:ty) => {
        impl TryFrom<&Value> for $for_type {
            type Error = VenumError;
            fn try_from(item: &Value) -> Result<Self> {
                match item {
                    Value::$enum_type(v) => Ok(*v),
                    _ => Err(VenumError::Conversion(ConversionError::WrongType {
                        src_value: format!("{:?}", item), // i.e. Bool(true)
                        src_type: format!("{}{}", VAL_ENUM_NAME, item), // i.e. Value::Bool, where 'Bool' is generated by strum through the display trait
                        target_type: String::from(stringify!($for_type)),
                        details: None,
                    })),
                }
            }
        }
    };
}
impl_try_from_value_ref_for_copy_type!(Char, char);
impl_try_from_value_ref_for_copy_type!(Int8, i8);
impl_try_from_value_ref_for_copy_type!(Int16, i16);
impl_try_from_value_ref_for_copy_type!(Int32, i32);
impl_try_from_value_ref_for_copy_type!(Int64, i64);
impl_try_from_value_ref_for_copy_type!(Int128, i128);
impl_try_from_value_ref_for_copy_type!(UInt8, u8);
impl_try_from_value_ref_for_copy_type!(UInt16, u16);
impl_try_from_value_ref_for_copy_type!(UInt32, u32);
impl_try_from_value_ref_for_copy_type!(UInt64, u64);
impl_try_from_value_ref_for_copy_type!(UInt128, u128);
impl_try_from_value_ref_for_copy_type!(Float32, f32);
impl_try_from_value_ref_for_copy_type!(Float64, f64);
impl_try_from_value_ref_for_copy_type!(Bool, bool);

macro_rules! impl_try_from_value_ref_for_clone_type {
    ($enum_type:ident, $for_type:ty) => {
        impl TryFrom<&Value> for $for_type {
            type Error = VenumError;
            fn try_from(item: &Value) -> Result<Self> {
                match item {
                    Value::$enum_type(v) => Ok(v.clone()),
                    _ => Err(VenumError::Conversion(ConversionError::WrongType {
                        src_value: format!("{:?}", item), // i.e. Bool(true)
                        src_type: format!("{}{}", VAL_ENUM_NAME, item), // i.e. Value::Bool, where 'Bool' is generated by strum through the display trait
                        target_type: String::from(stringify!($for_type)),
                        details: None,
                    })),
                }
            }
        }
    };
}
impl_try_from_value_ref_for_clone_type!(String, String);
impl_try_from_value_ref_for_clone_type!(Decimal, Decimal);
impl_try_from_value_ref_for_clone_type!(NaiveDate, NaiveDate);
impl_try_from_value_ref_for_clone_type!(NaiveDateTime, NaiveDateTime);
impl_try_from_value_ref_for_clone_type!(DateTime, DateTime<FixedOffset>);

macro_rules! from_type_string {
    ($fn_name:ident, $enum_type:ident, $for_type:ty) => {
        pub fn $fn_name(v: &str) -> Result<Value> {
            // We don't do something like:
            // if v.is_empty() {
            //     return Ok(Value::None);
            // }
            // here, with the reasoning that this should rather fail than to
            // magically give back a Value::None

            let temp = v.parse::<$for_type>().map_err(|_| {
                VenumError::Parsing(ParseError::ValueFromStringFailed {
                    src_value: String::from(v),
                    target_type: format!("{}{}", VAL_ENUM_NAME, stringify!($enum_type)),
                    details: None,
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

impl From<&ValueType> for Value {
    fn from(vvvn: &ValueType) -> Self {
        match vvvn {
            ValueType::Char => Value::char_default(),
            ValueType::String => Value::string_default(),
            ValueType::Int8 => Value::int8_default(),
            ValueType::Int16 => Value::int16_default(),
            ValueType::Int32 => Value::int32_default(),
            ValueType::Int64 => Value::int64_default(),
            ValueType::Int128 => Value::int128_default(),
            ValueType::UInt8 => Value::uint8_default(),
            ValueType::UInt16 => Value::uint16_default(),
            ValueType::UInt32 => Value::uint32_default(),
            ValueType::UInt64 => Value::uint64_default(),
            ValueType::UInt128 => Value::uint128_default(),
            ValueType::Float32 => Value::float32_default(),
            ValueType::Float64 => Value::float64_default(),
            ValueType::Bool => Value::bool_default(),
            ValueType::Decimal => Value::decimal_default(),
            ValueType::NaiveDate => Value::naive_date_default(),
            ValueType::NaiveDateTime => Value::naive_date_time_default(),
            ValueType::DateTime => Value::date_time_default(),
        }
    }
}

impl From<ValueType> for Value {
    fn from(vvvn: ValueType) -> Self {
        Value::from(&vvvn)
    }
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
    from_type_string!(parse_float32_from_str, Float32, f32);
    from_type_string!(parse_float64_from_str, Float64, f64);
    from_type_string!(parse_bool_from_str, Bool, bool);

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
    pub fn parse_naive_date_time_from_str_iso8601_ymdhms(v: &str) -> Result<Value> {
        // e.g pattern "%F %T" (which is "%Y-%m-%d %H:%M:%S") to parse "2015-09-05 23:56:04"
        // Is this really a good idea?
        if v.is_empty() {
            return Ok(Value::None);
        }
        let chrono_pattern = "%F %T";
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
        // Is this really a good idea?
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
        // Is this really a good idea?
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
        // Is this really a good idea?
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
            if !none_check_vals.is_empty() {
                if none_check_vals.contains(&value) {
                    return Ok(Value::None); // Caller must remember the desired type!
                }
            }
        }
        if let Some(chrono_pattern) = chrono_pattern {
            match target_value_type {
                ValueType::NaiveDate => Value::parse_naive_date_from_str(value, chrono_pattern),
                ValueType::NaiveDateTime => {
                    Value::parse_naive_date_time_from_str(value, &chrono_pattern)
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
                    Value::parse_naive_date_time_from_str_iso8601_ymdhms(value)
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

    mod value_type {
        use crate::venum::{Value, ValueType};

        #[test]
        fn from_venum_value_variant_name_for_value() {
            assert_eq!(Value::char_default(), ValueType::Char.into());
            assert_eq!(Value::string_default(), ValueType::String.into());
            assert_eq!(Value::int8_default(), ValueType::Int8.into());
            assert_eq!(Value::int16_default(), ValueType::Int16.into());
            assert_eq!(Value::int32_default(), ValueType::Int32.into());
            assert_eq!(Value::int64_default(), ValueType::Int64.into());
            assert_eq!(Value::int128_default(), ValueType::Int128.into());
            assert_eq!(Value::uint8_default(), ValueType::UInt8.into());
            assert_eq!(Value::uint16_default(), ValueType::UInt16.into());
            assert_eq!(Value::uint32_default(), ValueType::UInt32.into());
            assert_eq!(Value::uint64_default(), ValueType::UInt64.into());
            assert_eq!(Value::uint128_default(), ValueType::UInt128.into());
            assert_eq!(Value::float32_default(), ValueType::Float32.into());
            assert_eq!(Value::float64_default(), ValueType::Float64.into());
            assert_eq!(Value::bool_default(), ValueType::Bool.into());
            assert_eq!(Value::decimal_default(), ValueType::Decimal.into());
            assert_eq!(Value::naive_date_default(), ValueType::NaiveDate.into());
            assert_eq!(
                Value::naive_date_time_default(),
                ValueType::NaiveDateTime.into()
            );
            assert_eq!(Value::date_time_default(), ValueType::DateTime.into());
        }

        #[test]
        fn from_venum_value_variant_name_ref_for_value() {
            assert_eq!(Value::char_default(), (&ValueType::Char).into());
            assert_eq!(Value::string_default(), (&ValueType::String).into());
            assert_eq!(Value::int8_default(), (&ValueType::Int8).into());
            assert_eq!(Value::int16_default(), (&ValueType::Int16).into());
            assert_eq!(Value::int32_default(), (&ValueType::Int32).into());
            assert_eq!(Value::int64_default(), (&ValueType::Int64).into());
            assert_eq!(Value::int128_default(), (&ValueType::Int128).into());
            assert_eq!(Value::uint8_default(), (&ValueType::UInt8).into());
            assert_eq!(Value::uint16_default(), (&ValueType::UInt16).into());
            assert_eq!(Value::uint32_default(), (&ValueType::UInt32).into());
            assert_eq!(Value::uint64_default(), (&ValueType::UInt64).into());
            assert_eq!(Value::uint128_default(), (&ValueType::UInt128).into());
            assert_eq!(Value::float32_default(), (&ValueType::Float32).into());
            assert_eq!(Value::float64_default(), (&ValueType::Float64).into());
            assert_eq!(Value::bool_default(), (&ValueType::Bool).into());
            assert_eq!(Value::decimal_default(), (&ValueType::Decimal).into());
            assert_eq!(Value::naive_date_default(), (&ValueType::NaiveDate).into());
            assert_eq!(
                Value::naive_date_time_default(),
                (&ValueType::NaiveDateTime).into()
            );
            assert_eq!(Value::date_time_default(), (&ValueType::DateTime).into());
        }
    }

    mod parse_from_str {
        use super::*;

        #[test]
        pub fn parse_int8_from_str() {
            assert_eq!(Ok(Value::Int8(-1)), Value::parse_int8_from_str("-1"));
        }

        #[test]
        pub fn parse_int16_from_str() {
            assert_eq!(Ok(Value::Int16(-1)), Value::parse_int16_from_str("-1"));
        }

        #[test]
        pub fn parse_int32_from_str() {
            assert_eq!(Ok(Value::Int32(-1)), Value::parse_int32_from_str("-1"));
        }

        #[test]
        pub fn parse_int32_from_str_err() {
            let err_res = Value::parse_int32_from_str("foobar");

            let exp = Err(VenumError::Parsing(ParseError::ValueFromStringFailed {
                src_value: String::from("foobar"),
                target_type: String::from("Value::Int32"),
                details: None,
            }));
            assert_eq!(exp, err_res);
        }

        #[test]
        pub fn parse_int64_from_str() {
            assert_eq!(Ok(Value::Int64(-1)), Value::parse_int64_from_str("-1"));
        }

        #[test]
        pub fn parse_int128_from_str() {
            assert_eq!(Ok(Value::Int128(-1)), Value::parse_int128_from_str("-1"));
        }

        #[test]
        pub fn parse_uint8_from_str() {
            assert_eq!(
                Ok(Value::UInt8(1)),
                Value::parse_uint8_from_str(&String::from("1"))
            );
        }

        #[test]
        pub fn parse_uint16_from_str() {
            assert_eq!(Ok(Value::UInt16(1)), Value::parse_uint16_from_str("1"));
        }

        #[test]
        pub fn parse_uint32_from_str() {
            assert_eq!(Ok(Value::UInt32(1)), Value::parse_uint32_from_str("1"));
        }

        #[test]
        pub fn parse_uint64_from_str() {
            assert_eq!(Ok(Value::UInt64(1)), Value::parse_uint64_from_str("1"));
        }

        #[test]
        pub fn parse_uint128_from_str() {
            assert_eq!(Ok(Value::UInt128(1)), Value::parse_uint128_from_str("1"));
        }

        #[test]
        pub fn parse_float32_from_str() {
            assert_eq!(Ok(Value::Float32(1.0)), Value::parse_float32_from_str("1"));
        }

        #[test]
        pub fn parse_float64_from_str() {
            assert_eq!(Ok(Value::Float64(1.0)), Value::parse_float64_from_str("1"));
        }

        #[test]
        pub fn parse_bool_from_str() {
            assert_eq!(Ok(Value::Bool(true)), Value::parse_bool_from_str("true"));
        }

        #[test]
        #[should_panic(
            expected = "Parsing(ValueFromStringFailed { src_value: \"1.123\", target_type: \"Value::Int32\", details: None })"
        )]
        pub fn parse_i32_from_str_err() {
            Value::parse_int32_from_str("1.123").unwrap();
        }

        #[test]
        #[should_panic(
            expected = "Parsing(ValueFromStringFailed { src_value: \"1.123\", target_type: \"Value::UInt32\", details: None })"
        )]
        pub fn parse_u32_from_str_err() {
            Value::parse_uint32_from_str("1.123").unwrap();
        }

        #[test]
        #[should_panic(
            expected = "Parsing(ValueFromStringFailed { src_value: \"foobar\", target_type: \"Value::Float32\", details: None })"
        )]
        pub fn parse_f32_from_str_err() {
            Value::parse_float32_from_str("foobar").unwrap();
        }

        #[test]
        #[should_panic(
            expected = "Parsing(ValueFromStringFailed { src_value: \"foobar\", target_type: \"Value::Bool\", details: None })"
        )]
        pub fn parse_bool_from_str_err() {
            Value::parse_bool_from_str("foobar").unwrap();
        }

        #[test]
        pub fn parse_char_from_str() {
            assert_eq!(Ok(Value::Char('a')), Value::parse_char_from_str("a"));
        }

        #[test]
        #[should_panic(
            expected = "Err(Parsing(ValueFromStringFailed { src_value: \"abc\", target_type: \"Value::Char\", details: None }))"
        )]
        pub fn parse_char_from_str_err() {
            assert_eq!(Ok(Value::Char('a')), Value::parse_char_from_str("abc"));
        }
    }

    mod parse_from_str_decimal {
        use rust_decimal::Decimal;

        use crate::venum::Value;

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

        use crate::venum::Value;

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
        pub fn parse_naive_date_time_from_str_iso8601_ymdhms() {
            let exp = NaiveDate::from_ymd(2022, 12, 31).and_hms(12, 11, 10);

            assert_eq!(
                Ok(Value::NaiveDateTime(exp)),
                Value::parse_naive_date_time_from_str_iso8601_ymdhms("2022-12-31 12:11:10")
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
        pub fn parse_date_time_from_str_rfc3339() {
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

        use crate::venum::Value;

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
            errors::{ConversionError, VenumError},
            venum::Value,
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
        use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone};
        use rust_decimal::Decimal;

        use crate::venum::{Value, ValueType};

        #[test]
        pub fn impl_from_type_for_value() {
            assert_eq!(
                Value::String(String::from("foobar")),
                Value::from(String::from("foobar"))
            );

            assert_eq!(Value::Char('a'), Value::from('a'));

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
        pub fn impl_try_from_value_for_type() {
            assert_eq!(
                String::from("foobar"),
                String::try_from(Value::String(String::from("foobar"))).unwrap()
            );

            assert_eq!('a', char::try_from(Value::Char('a')).unwrap());

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
        pub fn impl_try_from_value_ref_for_copy_type() {
            assert_eq!('a', char::try_from(&Value::Char('a')).unwrap());

            assert_eq!(0i8, i8::try_from(&Value::Int8(0i8)).unwrap());
            assert_eq!(0i16, i16::try_from(&Value::Int16(0i16)).unwrap());
            assert_eq!(0i32, i32::try_from(&Value::Int32(0i32)).unwrap());
            assert_eq!(0i64, i64::try_from(&Value::Int64(0i64)).unwrap());
            assert_eq!(0i128, i128::try_from(&Value::Int128(0i128)).unwrap());

            assert_eq!(0u8, u8::try_from(&Value::UInt8(0u8)).unwrap());
            assert_eq!(0u16, u16::try_from(&Value::UInt16(0u16)).unwrap());
            assert_eq!(0u32, u32::try_from(&Value::UInt32(0u32)).unwrap());
            assert_eq!(0u64, u64::try_from(&Value::UInt64(0u64)).unwrap());
            assert_eq!(0u128, u128::try_from(&Value::UInt128(0u128)).unwrap());

            assert_eq!(0.0f32, f32::try_from(&Value::Float32(0.0f32)).unwrap());
            assert_eq!(0.0f64, f64::try_from(&Value::Float64(0.0f64)).unwrap());

            assert_eq!(true, bool::try_from(&Value::Bool(true)).unwrap());
        }

        #[test]
        pub fn impl_try_from_value_ref_for_clone_type() {
            assert_eq!(
                String::from("foobar"),
                String::try_from(&Value::String(String::from("foobar"))).unwrap()
            );

            assert_eq!(
                Decimal::new(00, 1),
                Decimal::try_from(&Value::Decimal(Decimal::new(00, 1))).unwrap()
            );

            assert_eq!(
                NaiveDate::from_ymd(2022, 12, 31),
                NaiveDate::try_from(&Value::NaiveDate(NaiveDate::from_ymd(2022, 12, 31))).unwrap()
            );
            assert_eq!(
                NaiveDate::from_ymd(2022, 12, 31).and_hms(12, 00, 00),
                NaiveDateTime::try_from(&Value::NaiveDateTime(
                    NaiveDate::from_ymd(2022, 12, 31).and_hms(12, 00, 00)
                ))
                .unwrap()
            );

            let dt: DateTime<FixedOffset> = FixedOffset::east(5 * 3600) // east = +; west = -
                .ymd(2022, 12, 31)
                .and_hms(6, 0, 0);
            assert_eq!(dt, DateTime::try_from(&Value::DateTime(dt)).unwrap());
        }

        #[test]
        #[should_panic(
            expected = "Conversion(WrongType { src_value: \"Int8(0)\", src_type: \"Value::Int8\", target_type: \"bool\", details: None })"
        )]
        pub fn string_to_bool_err() {
            bool::try_from(Value::Int8(0i8)).unwrap();
        }

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
        #[should_panic(
            expected = "Parsing(ValueFromStringFailed { src_value: \"false\", target_type: \"Value::Int8\", details: None })"
        )]
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
