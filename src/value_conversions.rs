use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;

use crate::errors_result::{ConversionError, Result, VenumError};
use crate::value::Value;
use crate::value_type::ValueType;

const DEFAULT_RADIX_10: u32 = 10;

impl Value {
    // TODO: docu

    // TODO: correct impls for from_char...

    pub fn try_convert_to(&self, target_type: ValueType) -> Result<Value> {
        let from_type = ValueType::try_from(self)?; // TODO: wrap error
        match target_type {
            ValueType::Char => match from_type {
                ValueType::Char => Ok(self.clone()),
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    let mut self_val_chars_iter = self_val.chars();
                    let self_val_char = self_val_chars_iter.next().unwrap(); // there must at least be something, initially!
                    match self_val_chars_iter.next() {
                        // however, if there is more, we have an error, as it is not a single char
                        Some(_) => Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            },
                        )),
                        None => Ok(Value::Char(self_val_char)),
                    }
                }
                ValueType::UInt8 => {
                    let self_val: u8 = self.try_into()?; // should never fail!
                    let self_val_as_u32: u32 = self_val.into();
                    let self_val_as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                        .ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Char(self_val_as_char))
                }
                ValueType::UInt16 => {
                    let self_val: u16 = self.try_into()?; // should never fail!
                    let self_val_as_u32: u32 = self_val.into();
                    let self_val_as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                        .ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Char(self_val_as_char))
                }
                ValueType::UInt32 => {
                    let self_val: u32 = self.try_into()?; // should never fail!
                    let self_val_as_char = char::from_digit(self_val, DEFAULT_RADIX_10)
                        .ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
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
                        .ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
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
                        .ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Char(self_val_as_char))
                }
                ValueType::Int8 => {
                    let self_val: i8 = self.try_into()?; // should never fail!
                    if self_val >= 0 {
                        let self_val_as_u32 = u32::try_from(self_val).unwrap(); // checked above and a positive signed int must fit into an usigned one, if the value is positive
                        let self_val_as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                            .ok_or_else(|| {
                                VenumError::Conversion(ConversionError::NotRepresentableAs {
                                    src: self.clone(),
                                    target_type,
                                })
                            })?;
                        Ok(Value::Char(self_val_as_char))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Int16 => {
                    let self_val: i16 = self.try_into()?; // should never fail!
                    if self_val >= 0 {
                        let self_val_as_u32 = u32::try_from(self_val).unwrap(); // checked above and a positive signed int must fit into an usigned one, if the value is positive
                        let self_val_as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                            .ok_or_else(|| {
                                VenumError::Conversion(ConversionError::NotRepresentableAs {
                                    src: self.clone(),
                                    target_type,
                                })
                            })?;
                        Ok(Value::Char(self_val_as_char))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Int32 => {
                    let self_val: i32 = self.try_into()?; // should never fail!
                    if self_val >= 0 {
                        let self_val_as_u32 = u32::try_from(self_val).unwrap(); // checked above and a positive signed int must fit into an usigned one, if the value is positive
                        let self_val_as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                            .ok_or_else(|| {
                                VenumError::Conversion(ConversionError::NotRepresentableAs {
                                    src: self.clone(),
                                    target_type,
                                })
                            })?;
                        Ok(Value::Char(self_val_as_char))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Int64 => {
                    let self_val: i64 = self.try_into()?; // should never fail!
                    if self_val >= 0 {
                        let self_val_as_u32 = u32::try_from(self_val).unwrap(); // checked above and a positive signed int must fit into an usigned one, if the value is positive
                        let self_val_as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                            .ok_or_else(|| {
                                VenumError::Conversion(ConversionError::NotRepresentableAs {
                                    src: self.clone(),
                                    target_type,
                                })
                            })?;
                        Ok(Value::Char(self_val_as_char))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Int128 => {
                    let self_val: i128 = self.try_into()?; // should never fail!
                    if self_val >= 0 {
                        let self_val_as_u32 = u32::try_from(self_val).unwrap(); // checked above and a positive signed int must fit into an usigned one, if the value is positive
                        let self_val_as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                            .ok_or_else(|| {
                                VenumError::Conversion(ConversionError::NotRepresentableAs {
                                    src: self.clone(),
                                    target_type,
                                })
                            })?;
                        Ok(Value::Char(self_val_as_char))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Float32 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Float64 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Bool => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Decimal => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::String => match from_type {
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
            },
            ValueType::Int8 => match from_type {
                ValueType::Char => {
                    let self_val: char = self.try_into()?;
                    let self_val_as_digit_u32 =
                        self_val.to_digit(DEFAULT_RADIX_10).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    let self_val_as_target_primitive: i8 =
                        self_val_as_digit_u32.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int8(self_val_as_target_primitive))
                }
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    let self_val_as_target_primitive: i8 =
                        self_val.parse::<i8>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::Int8(self_val_as_target_primitive))
                }
                ValueType::Int8 => Ok(self.clone()),
                ValueType::Int16 => {
                    let self_val_primitive: i16 = self.try_into()?;
                    let self_val_as_target_primitive: i8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int8(self_val_as_target_primitive))
                }
                ValueType::Int32 => {
                    let self_val_primitive: i32 = self.try_into()?;
                    let self_val_as_target_primitive: i8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int8(self_val_as_target_primitive))
                }
                ValueType::Int64 => {
                    let self_val_primitive: i64 = self.try_into()?;
                    let self_val_as_target_primitive: i8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int8(self_val_as_target_primitive))
                }
                ValueType::Int128 => {
                    let self_val_primitive: i128 = self.try_into()?;
                    let self_val_as_target_primitive: i8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int8(self_val_as_target_primitive))
                }
                ValueType::UInt8 => {
                    let self_val_primitive: u8 = self.try_into()?;
                    let self_val_as_target_primitive: i8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int8(self_val_as_target_primitive))
                }
                ValueType::UInt16 => {
                    let self_val_primitive: u16 = self.try_into()?;
                    let self_val_as_target_primitive: i8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int8(self_val_as_target_primitive))
                }
                ValueType::UInt32 => {
                    let self_val_primitive: u32 = self.try_into()?;
                    let self_val_as_target_primitive: i8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int8(self_val_as_target_primitive))
                }
                ValueType::UInt64 => {
                    let self_val_primitive: u64 = self.try_into()?;
                    let self_val_as_target_primitive: i8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int8(self_val_as_target_primitive))
                }
                ValueType::UInt128 => {
                    let self_val_primitive: u128 = self.try_into()?;
                    let self_val_as_target_primitive: i8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int8(self_val_as_target_primitive))
                }
                ValueType::Float32 => {
                    let self_val_primitive: f32 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0
                        && self_val_primitive as i32 <= i8::MAX.into()
                        && self_val_primitive as i32 >= i8::MIN.into()
                    {
                        Ok(Value::Int8(self_val_primitive as i8))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Float64 => {
                    let self_val_primitive: f64 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0
                        && self_val_primitive as i64 <= i8::MAX.into()
                        && self_val_primitive as i64 >= i8::MIN.into()
                    {
                        Ok(Value::Int8(self_val_primitive as i8))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
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
                    let self_val_primitive: Decimal = self.try_into()?;
                    let self_val_as_target_primitive: i8 =
                        self_val_primitive.to_i8().ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int8(self_val_as_target_primitive))
                }
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::Int16 => match from_type {
                ValueType::Char => {
                    let self_val: char = self.try_into()?;
                    let self_val_as_digit_u32 =
                        self_val.to_digit(DEFAULT_RADIX_10).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    let self_val_as_target_primitive: i16 =
                        self_val_as_digit_u32.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int16(self_val_as_target_primitive))
                }
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    let self_val_as_target_primitive: i16 =
                        self_val.parse::<i16>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::Int16(self_val_as_target_primitive))
                }
                ValueType::Int8 => {
                    let self_val_primitive: i8 = self.try_into()?;
                    let self_val_as_target_primitive: i16 = self_val_primitive.into();
                    Ok(Value::Int16(self_val_as_target_primitive))
                }
                ValueType::Int16 => Ok(self.clone()),
                ValueType::Int32 => {
                    let self_val_primitive: i32 = self.try_into()?;
                    let self_val_as_target_primitive: i16 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int16(self_val_as_target_primitive))
                }
                ValueType::Int64 => {
                    let self_val_primitive: i64 = self.try_into()?;
                    let self_val_as_target_primitive: i16 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int16(self_val_as_target_primitive))
                }
                ValueType::Int128 => {
                    let self_val_primitive: i128 = self.try_into()?;
                    let self_val_as_target_primitive: i16 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int16(self_val_as_target_primitive))
                }
                ValueType::UInt8 => {
                    let self_val_primitive: u8 = self.try_into()?;
                    let self_val_as_target_primitive: i16 = self_val_primitive.into();
                    Ok(Value::Int16(self_val_as_target_primitive))
                }
                ValueType::UInt16 => {
                    let self_val_primitive: u16 = self.try_into()?;
                    let self_val_as_target_primitive: i16 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int16(self_val_as_target_primitive))
                }
                ValueType::UInt32 => {
                    let self_val_primitive: u32 = self.try_into()?;
                    let self_val_as_target_primitive: i16 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int16(self_val_as_target_primitive))
                }
                ValueType::UInt64 => {
                    let self_val_primitive: u64 = self.try_into()?;
                    let self_val_as_target_primitive: i16 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int16(self_val_as_target_primitive))
                }
                ValueType::UInt128 => {
                    let self_val_primitive: u128 = self.try_into()?;
                    let self_val_as_target_primitive: i16 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int16(self_val_as_target_primitive))
                }
                ValueType::Float32 => {
                    let self_val_primitive: f32 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0
                        && self_val_primitive as i32 <= i16::MAX.into()
                        && self_val_primitive as i32 >= i16::MIN.into()
                    {
                        Ok(Value::Int16(self_val_primitive as i16))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Float64 => {
                    let self_val_primitive: f64 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0
                        && self_val_primitive as i64 <= i16::MAX.into()
                        && self_val_primitive as i64 >= i16::MIN.into()
                    {
                        Ok(Value::Int16(self_val_primitive as i16))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
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
                    let self_val_primitive: Decimal = self.try_into()?;
                    let self_val_as_target_primitive: i16 =
                        self_val_primitive.to_i16().ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int16(self_val_as_target_primitive))
                }
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::Int32 => match from_type {
                ValueType::Char => {
                    let self_val: char = self.try_into()?;
                    let self_val_as_digit_u32 =
                        self_val.to_digit(DEFAULT_RADIX_10).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    let self_val_as_target_primitive: i32 =
                        self_val_as_digit_u32.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int32(self_val_as_target_primitive))
                }
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    let self_val_as_target_primitive: i32 =
                        self_val.parse::<i32>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::Int32(self_val_as_target_primitive))
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
                    let self_val_as_target_primitive: i32 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int32(self_val_as_target_primitive))
                }
                ValueType::Int128 => {
                    let self_val_primitive: i128 = self.try_into()?;
                    let self_val_as_target_primitive: i32 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
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
                    let self_val_as_target_primitive: i32 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int32(self_val_as_target_primitive))
                }
                ValueType::UInt64 => {
                    let self_val_primitive: u64 = self.try_into()?;
                    let self_val_as_target_primitive: i32 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int32(self_val_as_target_primitive))
                }
                ValueType::UInt128 => {
                    let self_val_primitive: u128 = self.try_into()?;
                    let self_val_as_target_primitive: i32 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int32(self_val_as_target_primitive))
                }
                ValueType::Float32 => {
                    let self_val_primitive: f32 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0
                        && self_val_primitive as i32 <= i32::MAX.into()
                        && self_val_primitive as i32 >= i32::MIN.into()
                    {
                        Ok(Value::Int32(self_val_primitive as i32))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Float64 => {
                    let self_val_primitive: f64 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0
                        && self_val_primitive as i64 <= i32::MAX.into()
                        && self_val_primitive as i64 >= i32::MIN.into()
                    {
                        Ok(Value::Int32(self_val_primitive as i32))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
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
                    let self_val_primitive: Decimal = self.try_into()?;
                    let self_val_as_target_primitive: i32 =
                        self_val_primitive.to_i32().ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int32(self_val_as_target_primitive))
                }
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::Int64 => match from_type {
                ValueType::Char => {
                    let self_val: char = self.try_into()?;
                    let self_val_as_digit_u32 =
                        self_val.to_digit(DEFAULT_RADIX_10).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    let self_val_as_target_primitive: i64 =
                        self_val_as_digit_u32.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int64(self_val_as_target_primitive))
                }
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    let self_val_as_target_primitive: i64 =
                        self_val.parse::<i64>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::Int64(self_val_as_target_primitive))
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
                    let self_val_as_target_primitive: i64 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
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
                    let self_val_as_target_primitive: i64 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int64(self_val_as_target_primitive))
                }
                ValueType::UInt128 => {
                    let self_val_primitive: u128 = self.try_into()?;
                    let self_val_as_target_primitive: i64 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int64(self_val_as_target_primitive))
                }
                ValueType::Float32 => {
                    let self_val_primitive: f32 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0 {
                        Ok(Value::Int64(self_val_primitive as i64))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Float64 => {
                    let self_val_primitive: f64 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0 {
                        Ok(Value::Int64(self_val_primitive as i64))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
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
                    let self_val_primitive: Decimal = self.try_into()?;
                    let self_val_as_target_primitive: i64 =
                        self_val_primitive.to_i64().ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int64(self_val_as_target_primitive))
                }
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::Int128 => match from_type {
                ValueType::Char => {
                    let self_val: char = self.try_into()?;
                    let self_val_as_digit_u32 =
                        self_val.to_digit(DEFAULT_RADIX_10).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    let self_val_as_target_primitive: i128 =
                        self_val_as_digit_u32.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int128(self_val_as_target_primitive))
                }
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    let self_val_as_target_primitive: i128 =
                        self_val.parse::<i128>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::Int128(self_val_as_target_primitive))
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
                    let self_val_as_target_primitive: i128 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int128(self_val_as_target_primitive))
                }
                ValueType::Float32 => {
                    let self_val_primitive: f32 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0 {
                        Ok(Value::Int128(self_val_primitive as i128))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Float64 => {
                    let self_val_primitive: f64 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0 {
                        Ok(Value::Int128(self_val_primitive as i128))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
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
                    let self_val_primitive: Decimal = self.try_into()?;
                    let self_val_as_target_primitive: i128 =
                        self_val_primitive.to_i128().ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Int128(self_val_as_target_primitive))
                }
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::UInt8 => match from_type {
                ValueType::Char => {
                    let self_val: char = self.try_into()?;
                    let self_val_as_digit_u32 =
                        self_val.to_digit(DEFAULT_RADIX_10).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    let self_val_as_target_primitive: u8 =
                        self_val_as_digit_u32.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt8(self_val_as_target_primitive))
                }
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    let self_val_as_target_primitive: u8 =
                        self_val.parse::<u8>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::UInt8(self_val_as_target_primitive))
                }
                ValueType::Int8 => {
                    let self_val_primitive: i8 = self.try_into()?;
                    let self_val_as_target_primitive: u8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt8(self_val_as_target_primitive))
                }
                ValueType::Int16 => {
                    let self_val_primitive: i16 = self.try_into()?;
                    let self_val_as_target_primitive: u8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt8(self_val_as_target_primitive))
                }
                ValueType::Int32 => {
                    let self_val_primitive: i32 = self.try_into()?;
                    let self_val_as_target_primitive: u8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt8(self_val_as_target_primitive))
                }
                ValueType::Int64 => {
                    let self_val_primitive: i64 = self.try_into()?;
                    let self_val_as_target_primitive: u8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt8(self_val_as_target_primitive))
                }
                ValueType::Int128 => {
                    let self_val_primitive: i128 = self.try_into()?;
                    let self_val_as_target_primitive: u8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt8(self_val_as_target_primitive))
                }
                ValueType::UInt8 => Ok(self.clone()),
                ValueType::UInt16 => {
                    let self_val_primitive: u16 = self.try_into()?;
                    let self_val_as_target_primitive: u8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt8(self_val_as_target_primitive))
                }
                ValueType::UInt32 => {
                    let self_val_primitive: u32 = self.try_into()?;
                    let self_val_as_target_primitive: u8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt8(self_val_as_target_primitive))
                }
                ValueType::UInt64 => {
                    let self_val_primitive: u64 = self.try_into()?;
                    let self_val_as_target_primitive: u8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt8(self_val_as_target_primitive))
                }
                ValueType::UInt128 => {
                    let self_val_primitive: u128 = self.try_into()?;
                    let self_val_as_target_primitive: u8 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt8(self_val_as_target_primitive))
                }
                ValueType::Float32 => {
                    let self_val_primitive: f32 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0
                        && self_val_primitive as i32 <= u8::MAX.into()
                        && self_val_primitive as i32 >= u8::MIN.into()
                    {
                        Ok(Value::UInt8(self_val_primitive as u8))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Float64 => {
                    let self_val_primitive: f64 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0
                        && self_val_primitive as i64 <= u8::MAX.into()
                        && self_val_primitive as i64 >= u8::MIN.into()
                    {
                        Ok(Value::UInt8(self_val_primitive as u8))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
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
                    let self_val_primitive: Decimal = self.try_into()?;
                    let self_val_as_target_primitive: u8 =
                        self_val_primitive.to_u8().ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt8(self_val_as_target_primitive))
                }
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::UInt16 => match from_type {
                ValueType::Char => {
                    let self_val: char = self.try_into()?;
                    let self_val_as_digit_u32 =
                        self_val.to_digit(DEFAULT_RADIX_10).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    let self_val_as_target_primitive: u16 =
                        self_val_as_digit_u32.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt16(self_val_as_target_primitive))
                }
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    let self_val_as_target_primitive: u16 =
                        self_val.parse::<u16>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::UInt16(self_val_as_target_primitive))
                }
                ValueType::Int8 => {
                    let self_val_primitive: i8 = self.try_into()?;
                    let self_val_as_target_primitive: u16 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt16(self_val_as_target_primitive))
                }
                ValueType::Int16 => {
                    let self_val_primitive: i16 = self.try_into()?;
                    let self_val_as_target_primitive: u16 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt16(self_val_as_target_primitive))
                }
                ValueType::Int32 => {
                    let self_val_primitive: i32 = self.try_into()?;
                    let self_val_as_target_primitive: u16 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt16(self_val_as_target_primitive))
                }
                ValueType::Int64 => {
                    let self_val_primitive: i64 = self.try_into()?;
                    let self_val_as_target_primitive: u16 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt16(self_val_as_target_primitive))
                }
                ValueType::Int128 => {
                    let self_val_primitive: i128 = self.try_into()?;
                    let self_val_as_target_primitive: u16 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
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
                    let self_val_as_target_primitive: u16 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt16(self_val_as_target_primitive))
                }
                ValueType::UInt64 => {
                    let self_val_primitive: u64 = self.try_into()?;
                    let self_val_as_target_primitive: u16 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt16(self_val_as_target_primitive))
                }
                ValueType::UInt128 => {
                    let self_val_primitive: u128 = self.try_into()?;
                    let self_val_as_target_primitive: u16 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt16(self_val_as_target_primitive))
                }
                ValueType::Float32 => {
                    let self_val_primitive: f32 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0
                        && self_val_primitive as i32 <= u16::MAX.into()
                        && self_val_primitive as i32 >= u16::MIN.into()
                    {
                        Ok(Value::UInt16(self_val_primitive as u16))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Float64 => {
                    let self_val_primitive: f64 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0
                        && self_val_primitive as i64 <= u16::MAX.into()
                        && self_val_primitive as i64 >= u16::MIN.into()
                    {
                        Ok(Value::UInt16(self_val_primitive as u16))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
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
                    let self_val_primitive: Decimal = self.try_into()?;
                    let self_val_as_target_primitive: u16 =
                        self_val_primitive.to_u16().ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt16(self_val_as_target_primitive))
                }
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::UInt32 => match from_type {
                ValueType::Char => {
                    let self_val: char = self.try_into()?;
                    let self_val_as_digit_u32 =
                        self_val.to_digit(DEFAULT_RADIX_10).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    let self_val_as_target_primitive: u32 =
                        self_val_as_digit_u32.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt32(self_val_as_target_primitive))
                }
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    let self_val_as_target_primitive: u16 =
                        self_val.parse::<u16>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::UInt16(self_val_as_target_primitive))
                }
                ValueType::Int8 => {
                    let self_val_primitive: i8 = self.try_into()?;
                    let self_val_as_target_primitive: u32 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt32(self_val_as_target_primitive))
                }
                ValueType::Int16 => {
                    let self_val_primitive: i16 = self.try_into()?;
                    let self_val_as_target_primitive: u32 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt32(self_val_as_target_primitive))
                }
                ValueType::Int32 => {
                    let self_val_primitive: i32 = self.try_into()?;
                    let self_val_as_target_primitive: u32 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt32(self_val_as_target_primitive))
                }
                ValueType::Int64 => {
                    let self_val_primitive: i64 = self.try_into()?;
                    let self_val_as_target_primitive: u32 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt32(self_val_as_target_primitive))
                }
                ValueType::Int128 => {
                    let self_val_primitive: i128 = self.try_into()?;
                    let self_val_as_target_primitive: u32 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt32(self_val_as_target_primitive))
                }
                ValueType::UInt8 => {
                    let self_val_primitive: u8 = self.try_into()?;
                    let self_val_as_target_primitive: u32 = self_val_primitive.into();
                    Ok(Value::UInt32(self_val_as_target_primitive))
                }
                ValueType::UInt16 => {
                    let self_val_primitive: u32 = self.try_into()?;
                    let self_val_as_target_primitive: u32 = self_val_primitive.into();
                    Ok(Value::UInt32(self_val_as_target_primitive))
                }
                ValueType::UInt32 => Ok(self.clone()),
                ValueType::UInt64 => {
                    let self_val_primitive: u64 = self.try_into()?;
                    let self_val_as_target_primitive: u32 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt32(self_val_as_target_primitive))
                }
                ValueType::UInt128 => {
                    let self_val_primitive: u128 = self.try_into()?;
                    let self_val_as_target_primitive: u32 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt32(self_val_as_target_primitive))
                }
                ValueType::Float32 => {
                    let self_val_primitive: f32 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0 && self_val_primitive > 0.0 {
                        Ok(Value::UInt32(self_val_primitive as u32))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Float64 => {
                    let self_val_primitive: f64 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0
                        && self_val_primitive as i64 <= u32::MAX.into()
                        && self_val_primitive as i64 >= u32::MIN.into()
                    {
                        Ok(Value::UInt32(self_val_primitive as u32))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
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
                    let self_val_primitive: Decimal = self.try_into()?;
                    let self_val_as_target_primitive: u32 =
                        self_val_primitive.to_u32().ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt32(self_val_as_target_primitive))
                }
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::UInt64 => match from_type {
                ValueType::Char => {
                    let self_val: char = self.try_into()?;
                    let self_val_as_digit_u32 =
                        self_val.to_digit(DEFAULT_RADIX_10).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    let self_val_as_target_primitive: u64 =
                        self_val_as_digit_u32.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt64(self_val_as_target_primitive))
                }
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    let self_val_as_target_primitive: u64 =
                        self_val.parse::<u64>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::UInt64(self_val_as_target_primitive))
                }
                ValueType::Int8 => {
                    let self_val_primitive: i8 = self.try_into()?;
                    let self_val_as_target_primitive: u64 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt64(self_val_as_target_primitive))
                }
                ValueType::Int16 => {
                    let self_val_primitive: i16 = self.try_into()?;
                    let self_val_as_target_primitive: u64 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt64(self_val_as_target_primitive))
                }
                ValueType::Int32 => {
                    let self_val_primitive: i32 = self.try_into()?;
                    let self_val_as_target_primitive: u64 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt64(self_val_as_target_primitive))
                }
                ValueType::Int64 => {
                    let self_val_primitive: i64 = self.try_into()?;
                    let self_val_as_target_primitive: u64 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt64(self_val_as_target_primitive))
                }
                ValueType::Int128 => {
                    let self_val_primitive: i128 = self.try_into()?;
                    let self_val_as_target_primitive: u64 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt64(self_val_as_target_primitive))
                }
                ValueType::UInt8 => {
                    let self_val_primitive: u8 = self.try_into()?;
                    let self_val_as_target_primitive: u64 = self_val_primitive.into();
                    Ok(Value::UInt64(self_val_as_target_primitive))
                }
                ValueType::UInt16 => {
                    let self_val_primitive: u64 = self.try_into()?;
                    let self_val_as_target_primitive: u64 = self_val_primitive.into();
                    Ok(Value::UInt64(self_val_as_target_primitive))
                }
                ValueType::UInt32 => {
                    let self_val_primitive: u64 = self.try_into()?;
                    let self_val_as_target_primitive: u64 = self_val_primitive.into();
                    Ok(Value::UInt64(self_val_as_target_primitive))
                }
                ValueType::UInt64 => Ok(self.clone()),
                ValueType::UInt128 => {
                    let self_val_primitive: u128 = self.try_into()?;
                    let self_val_as_target_primitive: u64 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt64(self_val_as_target_primitive))
                }
                ValueType::Float32 => {
                    let self_val_primitive: f32 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0 && self_val_primitive > 0.0 {
                        Ok(Value::UInt64(self_val_primitive as u64))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Float64 => {
                    let self_val_primitive: f64 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0 && self_val_primitive > 0.0 {
                        Ok(Value::UInt64(self_val_primitive as u64))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
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
                    let self_val_primitive: Decimal = self.try_into()?;
                    let self_val_as_target_primitive: u64 =
                        self_val_primitive.to_u64().ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt64(self_val_as_target_primitive))
                }
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::UInt128 => match from_type {
                ValueType::Char => {
                    let self_val: char = self.try_into()?;
                    let self_val_as_digit_u32 =
                        self_val.to_digit(DEFAULT_RADIX_10).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    let self_val_as_target_primitive: u128 =
                        self_val_as_digit_u32.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt128(self_val_as_target_primitive))
                }
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    let self_val_as_target_primitive: u128 =
                        self_val.parse::<u128>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::UInt128(self_val_as_target_primitive))
                }
                ValueType::Int8 => {
                    let self_val_primitive: i8 = self.try_into()?;
                    let self_val_as_target_primitive: u128 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt128(self_val_as_target_primitive))
                }
                ValueType::Int16 => {
                    let self_val_primitive: i16 = self.try_into()?;
                    let self_val_as_target_primitive: u128 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt128(self_val_as_target_primitive))
                }
                ValueType::Int32 => {
                    let self_val_primitive: i32 = self.try_into()?;
                    let self_val_as_target_primitive: u128 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt128(self_val_as_target_primitive))
                }
                ValueType::Int64 => {
                    let self_val_primitive: i64 = self.try_into()?;
                    let self_val_as_target_primitive: u128 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt128(self_val_as_target_primitive))
                }
                ValueType::Int128 => {
                    let self_val_primitive: i128 = self.try_into()?;
                    let self_val_as_target_primitive: u128 =
                        self_val_primitive.try_into().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt128(self_val_as_target_primitive))
                }
                ValueType::UInt8 => {
                    let self_val_primitive: u8 = self.try_into()?;
                    let self_val_as_target_primitive: u128 = self_val_primitive.into();
                    Ok(Value::UInt128(self_val_as_target_primitive))
                }
                ValueType::UInt16 => {
                    let self_val_primitive: u128 = self.try_into()?;
                    let self_val_as_target_primitive: u128 = self_val_primitive.into();
                    Ok(Value::UInt128(self_val_as_target_primitive))
                }
                ValueType::UInt32 => {
                    let self_val_primitive: u128 = self.try_into()?;
                    let self_val_as_target_primitive: u128 = self_val_primitive.into();
                    Ok(Value::UInt128(self_val_as_target_primitive))
                }
                ValueType::UInt64 => {
                    let self_val_primitive: u128 = self.try_into()?;
                    let self_val_as_target_primitive: u128 = self_val_primitive.into();
                    Ok(Value::UInt128(self_val_as_target_primitive))
                }
                ValueType::UInt128 => Ok(self.clone()),
                ValueType::Float32 => {
                    let self_val_primitive: f32 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0 && self_val_primitive > 0.0 {
                        Ok(Value::UInt128(self_val_primitive as u128))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Float64 => {
                    let self_val_primitive: f64 = self.try_into()?;
                    if self_val_primitive.fract() == 0.0 && self_val_primitive > 0.0 {
                        Ok(Value::UInt128(self_val_primitive as u128))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
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
                    let self_val_primitive: Decimal = self.try_into()?;
                    let self_val_as_target_primitive: u128 =
                        self_val_primitive.to_u128().ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::UInt128(self_val_as_target_primitive))
                }
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::Float32 => match from_type {
                ValueType::Char => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    let self_val_as_target_primitive: f32 =
                        self_val.parse::<f32>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::Float32(self_val_as_target_primitive))
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
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Int64 => {
                    let self_val_primitive: i64 = self.try_into()?;
                    let self_val_as_target_primitive: f32 = self_val_primitive as f32;
                    let self_val_as_src_type_check: i64 = self_val_as_target_primitive as i64;
                    if self_val_primitive == self_val_as_src_type_check {
                        Ok(Value::Float32(self_val_as_target_primitive))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Int128 => {
                    let self_val_primitive: i128 = self.try_into()?;
                    let self_val_as_target_primitive: f32 = self_val_primitive as f32;
                    let self_val_as_src_type_check: i128 = self_val_as_target_primitive as i128;
                    if self_val_primitive == self_val_as_src_type_check {
                        Ok(Value::Float32(self_val_as_target_primitive))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
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
                    let self_val_as_src_type_check: u32 = self_val_as_target_primitive as u32;
                    if self_val_primitive == self_val_as_src_type_check {
                        Ok(Value::Float32(self_val_as_target_primitive))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::UInt64 => {
                    let self_val_primitive: u64 = self.try_into()?;
                    let self_val_as_target_primitive: f32 = self_val_primitive as f32;
                    let self_val_as_src_type_check: u64 = self_val_as_target_primitive as u64;
                    if self_val_primitive == self_val_as_src_type_check {
                        Ok(Value::Float32(self_val_as_target_primitive))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::UInt128 => {
                    let self_val_primitive: u128 = self.try_into()?;
                    let self_val_as_target_primitive: f32 = self_val_primitive as f32;
                    let self_val_as_src_type_check: u128 = self_val_as_target_primitive as u128;
                    if self_val_primitive == self_val_as_src_type_check {
                        Ok(Value::Float32(self_val_as_target_primitive))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
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
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Bool => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Decimal => {
                    let self_val_primitive: Decimal = self.try_into()?;
                    let self_val_as_target_primitive: f32 =
                        self_val_primitive.to_f32().ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Float32(self_val_as_target_primitive))
                }
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::Float64 => match from_type {
                ValueType::Char => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    let self_val_as_target_primitive: f64 =
                        self_val.parse::<f64>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::Float64(self_val_as_target_primitive))
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
                    let self_val_as_src_type_check: i64 = self_val_as_target_primitive as i64;
                    if self_val_primitive == self_val_as_src_type_check {
                        Ok(Value::Float64(self_val_as_target_primitive))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Int128 => {
                    let self_val_primitive: i128 = self.try_into()?;
                    let self_val_as_target_primitive: f64 = self_val_primitive as f64;
                    let self_val_as_src_type_check: i128 = self_val_as_target_primitive as i128;
                    if self_val_primitive == self_val_as_src_type_check {
                        Ok(Value::Float64(self_val_as_target_primitive))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
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
                    let self_val_as_src_type_check: u64 = self_val_as_target_primitive as u64;
                    if self_val_primitive == self_val_as_src_type_check {
                        Ok(Value::Float64(self_val_as_target_primitive))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::UInt128 => {
                    let self_val_primitive: u128 = self.try_into()?;
                    let self_val_as_target_primitive: f64 = self_val_primitive as f64;
                    let self_val_as_src_type_check: u128 = self_val_as_target_primitive as u128;
                    if self_val_primitive == self_val_as_src_type_check {
                        Ok(Value::Float64(self_val_as_target_primitive))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Float32 => {
                    let self_val_primitive: f32 = self.try_into()?;
                    let self_val_as_target_primitive: f64 = self_val_primitive.into();
                    Ok(Value::Float64(self_val_as_target_primitive))
                }
                ValueType::Float64 => Ok(self.clone()),
                ValueType::Bool => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Decimal => {
                    let self_val_primitive: Decimal = self.try_into()?;
                    let self_val_as_target_primitive: f64 =
                        self_val_primitive.to_f64().ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Float64(self_val_as_target_primitive))
                }
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::Bool => match from_type {
                ValueType::Char => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    let self_val_as_target_primitive: bool =
                        self_val.parse::<bool>().map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    Ok(Value::Bool(self_val_as_target_primitive))
                }
                ValueType::Int8 => {
                    let self_val: i8 = self.try_into()?;
                    if self_val == 1 {
                        Ok(Value::Bool(true))
                    } else if self_val == 0 {
                        Ok(Value::Bool(false))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Int16 => {
                    let self_val: i16 = self.try_into()?;
                    if self_val == 1 {
                        Ok(Value::Bool(true))
                    } else if self_val == 0 {
                        Ok(Value::Bool(false))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Int32 => {
                    let self_val: i32 = self.try_into()?;
                    if self_val == 1 {
                        Ok(Value::Bool(true))
                    } else if self_val == 0 {
                        Ok(Value::Bool(false))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Int64 => {
                    let self_val: i64 = self.try_into()?;
                    if self_val == 1 {
                        Ok(Value::Bool(true))
                    } else if self_val == 0 {
                        Ok(Value::Bool(false))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Int128 => {
                    let self_val: i128 = self.try_into()?;
                    if self_val == 1 {
                        Ok(Value::Bool(true))
                    } else if self_val == 0 {
                        Ok(Value::Bool(false))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::UInt8 => {
                    let self_val: u8 = self.try_into()?;
                    if self_val == 1 {
                        Ok(Value::Bool(true))
                    } else if self_val == 0 {
                        Ok(Value::Bool(false))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::UInt16 => {
                    let self_val: u16 = self.try_into()?;
                    if self_val == 1 {
                        Ok(Value::Bool(true))
                    } else if self_val == 0 {
                        Ok(Value::Bool(false))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::UInt32 => {
                    let self_val: u32 = self.try_into()?;
                    if self_val == 1 {
                        Ok(Value::Bool(true))
                    } else if self_val == 0 {
                        Ok(Value::Bool(false))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::UInt64 => {
                    let self_val: u64 = self.try_into()?;
                    if self_val == 1 {
                        Ok(Value::Bool(true))
                    } else if self_val == 0 {
                        Ok(Value::Bool(false))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::UInt128 => {
                    let self_val: u128 = self.try_into()?;
                    if self_val == 1 {
                        Ok(Value::Bool(true))
                    } else if self_val == 0 {
                        Ok(Value::Bool(false))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ))
                    }
                }
                ValueType::Float32 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Float64 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Bool => Ok(self.clone()),
                ValueType::Decimal => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::Decimal => match from_type {
                // TODO: debatable if we should convert, e.g. '1' to 1.0
                ValueType::Char => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    let self_val_as_target_type =
                        Decimal::from_str_exact(&self_val).map_err(|_err| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;

                    Ok(Value::Decimal(self_val_as_target_type))
                }
                ValueType::Int8 => {
                    let self_val_primitive: i8 = self.try_into()?;
                    let self_val_as_target_type =
                        Decimal::from_i8(self_val_primitive).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Decimal(self_val_as_target_type))
                }
                ValueType::Int16 => {
                    let self_val_primitive: i16 = self.try_into()?;
                    let self_val_as_target_type = Decimal::from_i16(self_val_primitive)
                        .ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Decimal(self_val_as_target_type))
                }
                ValueType::Int32 => {
                    let self_val_primitive: i32 = self.try_into()?;
                    let self_val_as_target_type = Decimal::from_i32(self_val_primitive)
                        .ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Decimal(self_val_as_target_type))
                }
                ValueType::Int64 => {
                    let self_val_primitive: i64 = self.try_into()?;
                    let self_val_as_target_type = Decimal::from_i64(self_val_primitive)
                        .ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Decimal(self_val_as_target_type))
                }
                ValueType::Int128 => {
                    let self_val_primitive: i128 = self.try_into()?;
                    let self_val_as_target_type = Decimal::from_i128(self_val_primitive)
                        .ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Decimal(self_val_as_target_type))
                }
                ValueType::UInt8 => {
                    let self_val_primitive: u8 = self.try_into()?;
                    let self_val_as_target_type =
                        Decimal::from_u8(self_val_primitive).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Decimal(self_val_as_target_type))
                }
                ValueType::UInt16 => {
                    let self_val_primitive: u16 = self.try_into()?;
                    let self_val_as_target_type = Decimal::from_u16(self_val_primitive)
                        .ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Decimal(self_val_as_target_type))
                }
                ValueType::UInt32 => {
                    let self_val_primitive: u32 = self.try_into()?;
                    let self_val_as_target_type = Decimal::from_u32(self_val_primitive)
                        .ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Decimal(self_val_as_target_type))
                }
                ValueType::UInt64 => {
                    let self_val_primitive: u64 = self.try_into()?;
                    let self_val_as_target_type = Decimal::from_u64(self_val_primitive)
                        .ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Decimal(self_val_as_target_type))
                }
                ValueType::UInt128 => {
                    let self_val_primitive: u128 = self.try_into()?;
                    let self_val_as_target_type = Decimal::from_u128(self_val_primitive)
                        .ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Decimal(self_val_as_target_type))
                }
                ValueType::Float32 => {
                    let self_val_primitive: f32 = self.try_into()?;
                    let self_val_as_target_type = Decimal::from_f32(self_val_primitive)
                        .ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Decimal(self_val_as_target_type))
                }
                ValueType::Float64 => {
                    let self_val_primitive: f64 = self.try_into()?;
                    let self_val_as_target_type = Decimal::from_f64(self_val_primitive)
                        .ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            })
                        })?;
                    Ok(Value::Decimal(self_val_as_target_type))
                }
                ValueType::Bool => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Decimal => Ok(self.clone()),
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::NaiveDate => match from_type {
                ValueType::Char => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    Value::from_str_and_type(&self_val, &ValueType::NaiveDate).map_err(|_err| {
                        VenumError::Conversion(ConversionError::NotRepresentableAs {
                            src: self.clone(),
                            target_type,
                        })
                    })
                }
                ValueType::Int8 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Int16 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Int32 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Int64 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Int128 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::UInt8 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::UInt16 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::UInt32 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::UInt64 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::UInt128 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Float32 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Float64 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Bool => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Decimal => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDate => Ok(self.clone()),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::NaiveDateTime => match from_type {
                ValueType::Char => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    Value::from_str_and_type(&self_val, &ValueType::NaiveDate).map_err(|_err| {
                        VenumError::Conversion(ConversionError::NotRepresentableAs {
                            src: self.clone(),
                            target_type,
                        })
                    })
                }
                ValueType::Int8 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Int16 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Int32 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Int64 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Int128 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::UInt8 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::UInt16 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::UInt32 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::UInt64 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::UInt128 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Float32 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Float64 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Bool => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Decimal => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Ok(self.clone()),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
            },
            ValueType::DateTime => match from_type {
                ValueType::Char => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::String => {
                    let self_val: String = self.try_into()?;
                    if self_val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type,
                            },
                        ));
                    }
                    Value::from_str_and_type(&self_val, &ValueType::NaiveDate).map_err(|_err| {
                        VenumError::Conversion(ConversionError::NotRepresentableAs {
                            src: self.clone(),
                            target_type,
                        })
                    })
                }
                ValueType::Int8 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Int16 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Int32 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Int64 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Int128 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::UInt8 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::UInt16 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::UInt32 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::UInt64 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::UInt128 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Float32 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Float64 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Bool => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::Decimal => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type,
                    },
                )),
                ValueType::DateTime => Ok(self.clone()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod char_as_target {
        use super::*;

        #[test]
        fn from_char() {
            assert_eq!(
                Value::Char('a'),
                Value::Char('a').try_convert_to(ValueType::Char).unwrap()
            );
        }

        #[test]
        fn from_string() {
            assert_eq!(
                Value::Char('a'),
                Value::String(String::from("a"))
                    .try_convert_to(ValueType::Char)
                    .unwrap()
            );
        }

        #[test]
        fn from_uint8() {
            assert_eq!(
                Value::Char('1'),
                Value::UInt8(1).try_convert_to(ValueType::Char).unwrap()
            );
        }
    }

    mod string_as_target {
        use chrono::TimeZone;

        use super::*;

        #[test]
        fn from_char() {
            assert_eq!(
                Value::String(String::from("a")),
                Value::Char('a').try_convert_to(ValueType::String).unwrap()
            );
        }

        #[test]
        fn from_string() {
            assert_eq!(
                Value::String(String::from("abc")),
                Value::String(String::from("abc"))
                    .try_convert_to(ValueType::String)
                    .unwrap()
            );
        }

        #[test]
        fn from_uint8() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::UInt8(1).try_convert_to(ValueType::String).unwrap()
            );
        }

        #[test]
        fn from_uint16() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::UInt16(1).try_convert_to(ValueType::String).unwrap()
            );
        }

        #[test]
        fn from_uint32() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::UInt32(1).try_convert_to(ValueType::String).unwrap()
            );
        }

        #[test]
        fn from_uint64() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::UInt64(1).try_convert_to(ValueType::String).unwrap()
            );
        }

        #[test]
        fn from_uint128() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::UInt128(1).try_convert_to(ValueType::String).unwrap()
            );
        }

        #[test]
        fn from_int8() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::Int8(1).try_convert_to(ValueType::String).unwrap()
            );
        }

        #[test]
        fn from_int16() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::Int16(1).try_convert_to(ValueType::String).unwrap()
            );
        }

        #[test]
        fn from_int32() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::Int32(1).try_convert_to(ValueType::String).unwrap()
            );
        }

        #[test]
        fn from_int64() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::Int64(1).try_convert_to(ValueType::String).unwrap()
            );
        }

        #[test]
        fn from_int128() {
            assert_eq!(
                Value::String(String::from("1")),
                Value::Int128(1).try_convert_to(ValueType::String).unwrap()
            );
        }

        #[test]
        fn from_float32() {
            assert_eq!(
                Value::String(String::from("123")),
                Value::Float32(123.0)
                    .try_convert_to(ValueType::String)
                    .unwrap()
            );
            assert_eq!(
                Value::String(String::from("123.456")),
                Value::Float32(123.456)
                    .try_convert_to(ValueType::String)
                    .unwrap()
            );
        }

        #[test]
        fn from_float64() {
            assert_eq!(
                Value::String(String::from("123")),
                Value::Float64(123.0)
                    .try_convert_to(ValueType::String)
                    .unwrap()
            );
            assert_eq!(
                Value::String(String::from("123.456")),
                Value::Float64(123.456)
                    .try_convert_to(ValueType::String)
                    .unwrap()
            );
        }

        #[test]
        fn from_bool() {
            assert_eq!(
                Value::String(String::from("true")),
                Value::Bool(true).try_convert_to(ValueType::String).unwrap()
            );
            assert_eq!(
                Value::String(String::from("false")),
                Value::Bool(false)
                    .try_convert_to(ValueType::String)
                    .unwrap()
            );
        }

        #[test]
        fn from_decimal() {
            assert_eq!(
                Value::String(String::from("123.456")),
                Value::Decimal(Decimal::new(123456, 3))
                    .try_convert_to(ValueType::String)
                    .unwrap()
            );
        }

        #[test]
        fn from_naive_date() {
            assert_eq!(
                Value::String(String::from("2022-12-31")),
                Value::NaiveDate(NaiveDate::from_ymd(2022, 12, 31))
                    .try_convert_to(ValueType::String)
                    .unwrap()
            );
        }

        #[test]
        fn from_naive_date_time() {
            assert_eq!(
                Value::String(String::from("2022-12-31 10:00:00")),
                Value::NaiveDateTime(NaiveDate::from_ymd(2022, 12, 31).and_hms(10, 0, 0))
                    .try_convert_to(ValueType::String)
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
                .try_convert_to(ValueType::String)
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
                .try_convert_to(ValueType::String)
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
                .try_convert_to(ValueType::String)
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
                .try_convert_to(ValueType::String)
                .unwrap()
            );
        }
    }

    mod uint8_as_target {
        use chrono::TimeZone;

        use super::*;

        #[test]
        fn from_char() {
            assert_eq!(
                Value::UInt8(8),
                Value::Char('8').try_convert_to(ValueType::UInt8).unwrap()
            );
        }

        #[test]
        fn from_string() {
            assert_eq!(
                Value::UInt8(8),
                Value::String(String::from("8"))
                    .try_convert_to(ValueType::UInt8)
                    .unwrap()
            );
        }

        #[test]
        fn from_uint8() {
            assert_eq!(
                Value::UInt8(8),
                Value::UInt8(8).try_convert_to(ValueType::UInt8).unwrap()
            );
        }

        #[test]
        fn from_uint16() {
            assert_eq!(
                Value::UInt8(8),
                Value::UInt16(8).try_convert_to(ValueType::UInt8).unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_uint16_err_val_too_big() {
            Value::UInt16(u16::MAX)
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        fn from_uint32() {
            assert_eq!(
                Value::UInt8(8),
                Value::UInt32(8).try_convert_to(ValueType::UInt8).unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_uint32_err_val_too_big() {
            Value::UInt32(u32::MAX)
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        fn from_uint64() {
            assert_eq!(
                Value::UInt8(8),
                Value::UInt64(8).try_convert_to(ValueType::UInt8).unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_uint64_err_val_too_big() {
            Value::UInt64(u64::MAX)
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        fn from_uint128() {
            assert_eq!(
                Value::UInt8(8),
                Value::UInt128(8).try_convert_to(ValueType::UInt8).unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_uint128_err_val_too_big() {
            Value::UInt128(u128::MAX)
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        fn from_int8() {
            assert_eq!(
                Value::UInt8(8),
                Value::Int8(8).try_convert_to(ValueType::UInt8).unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int8_err_val_too_small() {
            Value::Int8(-1).try_convert_to(ValueType::UInt8).unwrap();
        }

        #[test]
        fn from_int16() {
            assert_eq!(
                Value::UInt8(8),
                Value::Int16(8).try_convert_to(ValueType::UInt8).unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int16_err_val_too_big() {
            Value::Int16(i16::MAX)
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int16_err_val_too_small() {
            Value::Int16(-1).try_convert_to(ValueType::UInt8).unwrap();
        }

        #[test]
        fn from_int32() {
            assert_eq!(
                Value::UInt8(8),
                Value::Int32(8).try_convert_to(ValueType::UInt8).unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int32_err_val_too_big() {
            Value::Int32(i32::MAX)
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int32_err_val_too_small() {
            Value::Int32(-1).try_convert_to(ValueType::UInt8).unwrap();
        }

        #[test]
        fn from_int64() {
            assert_eq!(
                Value::UInt8(8),
                Value::Int64(8).try_convert_to(ValueType::UInt8).unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int64_err_val_too_big() {
            Value::Int64(i64::MAX)
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int64_err_val_too_small() {
            Value::Int64(-1).try_convert_to(ValueType::UInt8).unwrap();
        }

        #[test]
        fn from_int128() {
            assert_eq!(
                Value::UInt8(8),
                Value::Int128(8).try_convert_to(ValueType::UInt8).unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int128_err_val_too_big() {
            Value::Int128(i128::MAX)
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_int128_err_val_too_small() {
            Value::Int128(-1).try_convert_to(ValueType::UInt8).unwrap();
        }

        #[test]
        fn from_float32() {
            assert_eq!(
                Value::UInt8(8),
                Value::Float32(8.0)
                    .try_convert_to(ValueType::UInt8)
                    .unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float32_err_val_too_big() {
            Value::Float32(12345678.0)
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float32_err_val_too_small() {
            Value::Float32(-1.0)
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float32_err_val_uneven() {
            Value::Float32(1.5)
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        fn from_float64() {
            assert_eq!(
                Value::UInt8(8),
                Value::Float64(8.0)
                    .try_convert_to(ValueType::UInt8)
                    .unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_too_big() {
            Value::Float64(12345678.0)
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_too_small() {
            Value::Float64(-1.0)
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_float64_err_val_uneven() {
            Value::Float64(1.5)
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        fn from_bool() {
            assert_eq!(
                Value::UInt8(1),
                Value::Bool(true).try_convert_to(ValueType::UInt8).unwrap()
            );
            assert_eq!(
                Value::UInt8(0),
                Value::Bool(false).try_convert_to(ValueType::UInt8).unwrap()
            );
        }

        #[test]
        fn from_decimal() {
            assert_eq!(
                Value::UInt8(123),
                Value::Decimal(Decimal::new(123, 0))
                    .try_convert_to(ValueType::UInt8)
                    .unwrap()
            );
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_decimal_err_val_too_big() {
            Value::Decimal(Decimal::new(123456, 0))
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_decimal_err_val_too_small() {
            Value::Decimal(Decimal::new(-1, 0))
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_naive_date() {
            Value::NaiveDate(NaiveDate::from_ymd(2022, 12, 31))
                .try_convert_to(ValueType::UInt8)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Conversion(NotRepresentableAs")]
        fn from_naive_date_time() {
            Value::NaiveDateTime(NaiveDate::from_ymd(2022, 12, 31).and_hms(10, 0, 0))
                .try_convert_to(ValueType::UInt8)
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
            .try_convert_to(ValueType::UInt8)
            .unwrap();
        }
    }
}