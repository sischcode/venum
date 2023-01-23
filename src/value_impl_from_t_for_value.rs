use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;

use crate::value::Value;
use std::convert::From;

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

#[cfg(test)]
mod tests {
    use super::*;

    use rust_decimal::Decimal;

    use crate::value::Value;

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
            Value::NaiveDate(NaiveDate::from_ymd_opt(2022, 12, 31).unwrap()),
            Value::from(NaiveDate::from_ymd_opt(2022, 12, 31).unwrap())
        );
        assert_eq!(
            Value::NaiveDateTime(
                NaiveDate::from_ymd_opt(2022, 12, 31)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap()
            ),
            Value::from(
                NaiveDate::from_ymd_opt(2022, 12, 31)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap()
            )
        );

        let d = NaiveDate::from_ymd_opt(2022, 12, 31)
            .unwrap() // This date exists for sure. Unwrap is safe here
            .and_hms_milli_opt(6, 0, 0, 0)
            .unwrap() // This time exists for sure. Unwrap is safe here
            .and_local_timezone(FixedOffset::east_opt(5 * 3600).unwrap())
            .unwrap(); // This timezone (UTC) exists for sure. Unwrap is safe here

        assert_eq!(Value::DateTime(d), Value::from(d));
    }
}
