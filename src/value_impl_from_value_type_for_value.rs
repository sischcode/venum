use crate::{value::Value, value_type::ValueType};
use std::convert::From;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn venum_value_variant_default_val_from_value_type_variant() {
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
    fn venum_value_variant_default_val_from_value_type_variant_ref() {
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
