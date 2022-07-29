use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;

use crate::errors::{ConversionError, Result, VenumError};
use crate::venum::{Value, ValueType};

const DEFAULT_RADIX_10: u32 = 10;

impl Value {
    // TODO: docu
    pub fn try_convert_to(&self, target_type: ValueType) -> Result<Value> {
        let from_type = ValueType::try_from(self)?; // TODO: wrap error
        match target_type {
            ValueType::Char => match from_type {
                ValueType::Char => Ok(self.clone()),
                ValueType::String => {
                    let s: String = self.try_into()?; // TODO: wrap error
                    if s.len() == 1 {
                        let c = s.chars().next().unwrap();
                        Ok(Value::Char(c))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            },
                        ))
                    }
                }
                ValueType::UInt8 => {
                    let self_val: u8 = self.try_into()?; // should never fail!
                    let self_val_as_u32: u32 = self_val.into();
                    let as_char =
                        char::from_digit(self_val_as_u32, DEFAULT_RADIX_10).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type,
                            })
                        })?;
                    Ok(Value::Char(as_char))
                }
                ValueType::UInt16 => {
                    let self_val: u16 = self.try_into()?; // should never fail!
                    let self_val_as_u32: u32 = self_val.into();
                    let as_char =
                        char::from_digit(self_val_as_u32, DEFAULT_RADIX_10).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type,
                            })
                        })?;
                    Ok(Value::Char(as_char))
                }
                ValueType::UInt32 => {
                    let self_val: u32 = self.try_into()?; // should never fail!
                    let as_char =
                        char::from_digit(self_val, DEFAULT_RADIX_10).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type,
                            })
                        })?;
                    Ok(Value::Char(as_char))
                }
                ValueType::UInt64 => {
                    let self_val: u64 = self.try_into()?; // should never fail!
                    let self_val_as_u32: u32 = self_val.try_into().map_err(|_err| {
                        VenumError::Conversion(ConversionError::NotRepresentableAs {
                            src: self.clone(),
                            target_type: target_type.clone(),
                        })
                    })?;
                    let as_char =
                        char::from_digit(self_val_as_u32, DEFAULT_RADIX_10).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type,
                            })
                        })?;
                    Ok(Value::Char(as_char))
                }
                ValueType::UInt128 => {
                    let self_val: u128 = self.try_into()?; // should never fail!
                    let self_val_as_u32: u32 = self_val.try_into().map_err(|_err| {
                        VenumError::Conversion(ConversionError::NotRepresentableAs {
                            src: self.clone(),
                            target_type: target_type.clone(),
                        })
                    })?;
                    let as_char =
                        char::from_digit(self_val_as_u32, DEFAULT_RADIX_10).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type,
                            })
                        })?;
                    Ok(Value::Char(as_char))
                }
                ValueType::Int8 => {
                    let self_val: i8 = self.try_into()?; // should never fail!
                    if self_val >= 0 {
                        let self_val_as_u32 = u32::try_from(self_val).unwrap(); // checked above and a positive signed int must fit into an usigned one, if the value is positive
                        let as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                            .ok_or_else(|| {
                                VenumError::Conversion(ConversionError::NotRepresentableAs {
                                    src: self.clone(),
                                    target_type: target_type,
                                })
                            })?;
                        Ok(Value::Char(as_char))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type,
                            },
                        ))
                    }
                }
                ValueType::Int16 => {
                    let self_val: i16 = self.try_into()?; // should never fail!
                    if self_val >= 0 {
                        let self_val_as_u32 = u32::try_from(self_val).unwrap(); // checked above and a positive signed int must fit into an usigned one, if the value is positive
                        let as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                            .ok_or_else(|| {
                                VenumError::Conversion(ConversionError::NotRepresentableAs {
                                    src: self.clone(),
                                    target_type: target_type,
                                })
                            })?;
                        Ok(Value::Char(as_char))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type,
                            },
                        ))
                    }
                }
                ValueType::Int32 => {
                    let self_val: i32 = self.try_into()?; // should never fail!
                    if self_val >= 0 {
                        let self_val_as_u32 = u32::try_from(self_val).unwrap(); // checked above and a positive signed int must fit into an usigned one, if the value is positive
                        let as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                            .ok_or_else(|| {
                                VenumError::Conversion(ConversionError::NotRepresentableAs {
                                    src: self.clone(),
                                    target_type: target_type,
                                })
                            })?;
                        Ok(Value::Char(as_char))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type,
                            },
                        ))
                    }
                }
                ValueType::Int64 => {
                    let self_val: i64 = self.try_into()?; // should never fail!
                    if self_val >= 0 {
                        let self_val_as_u32 = u32::try_from(self_val).unwrap(); // checked above and a positive signed int must fit into an usigned one, if the value is positive
                        let as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                            .ok_or_else(|| {
                                VenumError::Conversion(ConversionError::NotRepresentableAs {
                                    src: self.clone(),
                                    target_type: target_type,
                                })
                            })?;
                        Ok(Value::Char(as_char))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type,
                            },
                        ))
                    }
                }
                ValueType::Int128 => {
                    let self_val: i128 = self.try_into()?; // should never fail!
                    if self_val >= 0 {
                        let self_val_as_u32 = u32::try_from(self_val).unwrap(); // checked above and a positive signed int must fit into an usigned one, if the value is positive
                        let as_char = char::from_digit(self_val_as_u32, DEFAULT_RADIX_10)
                            .ok_or_else(|| {
                                VenumError::Conversion(ConversionError::NotRepresentableAs {
                                    src: self.clone(),
                                    target_type: target_type,
                                })
                            })?;
                        Ok(Value::Char(as_char))
                    } else {
                        Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type,
                            },
                        ))
                    }
                }
                ValueType::Float32 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type: target_type,
                    },
                )),
                ValueType::Float64 => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type: target_type,
                    },
                )),
                ValueType::Bool => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type: target_type,
                    },
                )),
                ValueType::Decimal => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type: target_type,
                    },
                )),
                ValueType::NaiveDate => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type: target_type,
                    },
                )),
                ValueType::NaiveDateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type: target_type,
                    },
                )),
                ValueType::DateTime => Err(VenumError::Conversion(
                    ConversionError::NotRepresentableAs {
                        src: self.clone(),
                        target_type: target_type,
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
                    let val: i8 = self.try_into()?;
                    Ok(Value::String(val.to_string()))
                }
                ValueType::Int16 => {
                    let val: i16 = self.try_into()?;
                    Ok(Value::String(val.to_string()))
                }
                ValueType::Int32 => {
                    let val: i32 = self.try_into()?;
                    Ok(Value::String(val.to_string()))
                }
                ValueType::Int64 => {
                    let val: i64 = self.try_into()?;
                    Ok(Value::String(val.to_string()))
                }
                ValueType::Int128 => {
                    let val: i128 = self.try_into()?;
                    Ok(Value::String(val.to_string()))
                }
                ValueType::UInt8 => {
                    let val: u8 = self.try_into()?;
                    Ok(Value::String(val.to_string()))
                }
                ValueType::UInt16 => {
                    let val: u16 = self.try_into()?;
                    Ok(Value::String(val.to_string()))
                }
                ValueType::UInt32 => {
                    let val: u32 = self.try_into()?;
                    Ok(Value::String(val.to_string()))
                }
                ValueType::UInt64 => {
                    let val: u64 = self.try_into()?;
                    Ok(Value::String(val.to_string()))
                }
                ValueType::UInt128 => {
                    let val: u128 = self.try_into()?;
                    Ok(Value::String(val.to_string()))
                }
                ValueType::Float32 => {
                    let val: f32 = self.try_into()?;
                    Ok(Value::String(val.to_string()))
                }
                ValueType::Float64 => {
                    let val: f64 = self.try_into()?;
                    Ok(Value::String(val.to_string()))
                }
                ValueType::Bool => {
                    let val: bool = self.try_into()?;
                    Ok(Value::String(val.to_string()))
                }
                ValueType::Decimal => {
                    let val: Decimal = self.try_into()?;
                    Ok(Value::String(val.to_string()))
                }
                ValueType::NaiveDate => {
                    let val: NaiveDate = self.try_into()?; // TODO format
                    Ok(Value::String(val.to_string()))
                }
                ValueType::NaiveDateTime => {
                    let val: NaiveDateTime = self.try_into()?; // TODO format
                    Ok(Value::String(val.to_string()))
                }
                ValueType::DateTime => {
                    let val: DateTime<FixedOffset> = self.try_into()?; // TODO format
                    Ok(Value::String(val.to_string()))
                }
            },
            ValueType::Int8 => match from_type {
                ValueType::Char => {
                    let val: char = self.try_into()?;
                    let val_as_digit = val.to_digit(DEFAULT_RADIX_10).ok_or_else(|| {
                        VenumError::Conversion(ConversionError::NotRepresentableAs {
                            src: self.clone(),
                            target_type: target_type.clone(),
                        })
                    })?;
                    let val_as_target_type_digit: i8 = val_as_digit.try_into().map_err(|_err| {
                        VenumError::Conversion(ConversionError::NotRepresentableAs {
                            src: self.clone(),
                            target_type: target_type,
                        })
                    })?;
                    Ok(Value::Int8(val_as_target_type_digit))
                }
                ValueType::String => {
                    let val: String = self.try_into()?;
                    if val.is_empty() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type,
                            },
                        ));
                    }

                    let mut chars = val.chars();
                    let val_as_char: char = chars.next().unwrap(); // we checked the length above, there must be something here!
                    if chars.next().is_some() {
                        return Err(VenumError::Conversion(
                            ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type,
                            },
                        ));
                    }

                    let val_as_u32: u32 =
                        val_as_char.to_digit(DEFAULT_RADIX_10).ok_or_else(|| {
                            VenumError::Conversion(ConversionError::NotRepresentableAs {
                                src: self.clone(),
                                target_type: target_type.clone(),
                            })
                        })?;
                    let val_as_i8: i8 = val_as_u32.try_into().map_err(|_err| {
                        VenumError::Conversion(ConversionError::NotRepresentableAs {
                            src: self.clone(),
                            target_type: target_type,
                        })
                    })?;
                    Ok(Value::Int8(val_as_i8))
                }
                ValueType::Int8 => todo!(),
                ValueType::Int16 => todo!(),
                ValueType::Int32 => todo!(),
                ValueType::Int64 => todo!(),
                ValueType::Int128 => todo!(),
                ValueType::UInt8 => todo!(),
                ValueType::UInt16 => todo!(),
                ValueType::UInt32 => todo!(),
                ValueType::UInt64 => todo!(),
                ValueType::UInt128 => todo!(),
                ValueType::Float32 => todo!(),
                ValueType::Float64 => todo!(),
                ValueType::Bool => todo!(),
                ValueType::Decimal => todo!(),
                ValueType::NaiveDate => todo!(),
                ValueType::NaiveDateTime => todo!(),
                ValueType::DateTime => todo!(),
            },
            ValueType::Int16 => todo!(),
            ValueType::Int32 => todo!(),
            ValueType::Int64 => todo!(),
            ValueType::Int128 => todo!(),
            ValueType::UInt8 => todo!(),
            ValueType::UInt16 => todo!(),
            ValueType::UInt32 => todo!(),
            ValueType::UInt64 => todo!(),
            ValueType::UInt128 => todo!(),
            ValueType::Float32 => todo!(),
            ValueType::Float64 => todo!(),
            ValueType::Bool => todo!(),
            ValueType::Decimal => todo!(),
            ValueType::NaiveDate => todo!(),
            ValueType::NaiveDateTime => todo!(),
            ValueType::DateTime => todo!(),
        }
    }
}
