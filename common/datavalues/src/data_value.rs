// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

// Borrow from apache/arrow/rust/datafusion/src/functions.rs
// See notice.md

use std::convert::TryFrom;
use std::fmt;

use common_arrow::arrow::datatypes::IntervalUnit;
use common_arrow::arrow::datatypes::TimeUnit;
use common_exception::ErrorCode;
use common_exception::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::DataArrayRef;
use crate::DataField;
use crate::DataType;


/// A specific value of a data type.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum DataValue {
    /// Base type.
    Null,
    Boolean(Option<bool>),
    Int8(Option<i8>),
    Int16(Option<i16>),
    Int32(Option<i32>),
    Int64(Option<i64>),
    UInt8(Option<u8>),
    UInt16(Option<u16>),
    UInt32(Option<u32>),
    UInt64(Option<u64>),
    Float32(Option<f32>),
    Float64(Option<f64>),
    Binary(Option<Vec<u8>>),
    Utf8(Option<String>),

    /// Datetime.
    /// Date stored as a signed 32bit int
    Date32(Option<i32>),
    /// Date stored as a signed 64bit int
    Date64(Option<i64>),
    /// Timestamp Second
    TimestampSecond(Option<i64>),
    /// Timestamp Milliseconds
    TimestampMillisecond(Option<i64>),
    /// Timestamp Microseconds
    TimestampMicrosecond(Option<i64>),
    /// Timestamp Nanoseconds
    TimestampNanosecond(Option<i64>),
    /// Interval with YearMonth unit
    IntervalYearMonth(Option<i32>),
    /// Interval with DayTime unit
    IntervalDayTime(Option<i64>),

    // Container struct.
    List(Option<Vec<DataValue>>, DataType),
    Struct(Vec<DataValue>),
}

pub type DataValueRef = Box<DataValue>;

impl DataValue {
    pub fn is_null(&self) -> bool {
        matches!(
            self,
            DataValue::Boolean(None)
                | DataValue::Int8(None)
                | DataValue::Int16(None)
                | DataValue::Int32(None)
                | DataValue::Int64(None)
                | DataValue::UInt8(None)
                | DataValue::UInt16(None)
                | DataValue::UInt32(None)
                | DataValue::UInt64(None)
                | DataValue::Float32(None)
                | DataValue::Float64(None)
                | DataValue::Binary(None)
                | DataValue::Utf8(None)
                | DataValue::Date32(None)
                | DataValue::Date64(None)
                | DataValue::TimestampMillisecond(None)
                | DataValue::TimestampMicrosecond(None)
                | DataValue::TimestampNanosecond(None)
                | DataValue::List(None, _)
        )
    }

    pub fn data_type(&self) -> DataType {
        match self {
            DataValue::Null => DataType::Null,
            DataValue::Boolean(_) => DataType::Boolean,
            DataValue::Int8(_) => DataType::Int8,
            DataValue::Int16(_) => DataType::Int16,
            DataValue::Int32(_) => DataType::Int32,
            DataValue::Int64(_) => DataType::Int64,
            DataValue::UInt8(_) => DataType::UInt8,
            DataValue::UInt16(_) => DataType::UInt16,
            DataValue::UInt32(_) => DataType::UInt32,
            DataValue::UInt64(_) => DataType::UInt64,
            DataValue::Float32(_) => DataType::Float32,
            DataValue::Float64(_) => DataType::Float64,
            DataValue::Utf8(_) => DataType::Utf8,
            DataValue::Date32(_) => DataType::Date32,
            DataValue::Date64(_) => DataType::Date64,
            DataValue::TimestampSecond(_) => DataType::Timestamp(TimeUnit::Second, None),
            DataValue::TimestampMillisecond(_) => DataType::Timestamp(TimeUnit::Millisecond, None),
            DataValue::TimestampMicrosecond(_) => DataType::Timestamp(TimeUnit::Microsecond, None),
            DataValue::TimestampNanosecond(_) => DataType::Timestamp(TimeUnit::Nanosecond, None),
            DataValue::IntervalYearMonth(_) => DataType::Interval(IntervalUnit::YearMonth),
            DataValue::IntervalDayTime(_) => DataType::Interval(IntervalUnit::DayTime),
            DataValue::List(_, data_type) => {
                DataType::List(Box::new(DataField::new("item", data_type.clone(), true)))
            }
            DataValue::Struct(v) => {
                let fields = v
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        let typ = x.data_type();
                        DataField::new(format!("item_{}", i).as_str(), typ, true)
                    })
                    .collect::<Vec<_>>();
                DataType::Struct(fields)
            }
            DataValue::Binary(_) => DataType::Binary,
        }
    }

    pub fn to_array(&self) -> Result<DataArrayRef> {
        self.to_array_with_size(1)
    }

    pub fn to_array_with_size(&self, size: usize) -> Result<DataArrayRef> {
        todo!()
    }
}

typed_cast_from_data_value_to_std!(Int8, i8);
typed_cast_from_data_value_to_std!(Int16, i16);
typed_cast_from_data_value_to_std!(Int32, i32);
typed_cast_from_data_value_to_std!(Int64, i64);
typed_cast_from_data_value_to_std!(UInt8, u8);
typed_cast_from_data_value_to_std!(UInt16, u16);
typed_cast_from_data_value_to_std!(UInt32, u32);
typed_cast_from_data_value_to_std!(UInt64, u64);
typed_cast_from_data_value_to_std!(Float32, f32);
typed_cast_from_data_value_to_std!(Float64, f64);
typed_cast_from_data_value_to_std!(Boolean, bool);

impl TryFrom<&DataType> for DataValue {
    type Error = ErrorCode;

    fn try_from(data_type: &DataType) -> Result<Self> {
        match data_type {
            DataType::Null => Ok(DataValue::Null),
            DataType::Boolean => Ok(DataValue::Boolean(None)),
            DataType::Int8 => Ok(DataValue::Int8(None)),
            DataType::Int16 => Ok(DataValue::Int16(None)),
            DataType::Int32 => Ok(DataValue::Int32(None)),
            DataType::Int64 => Ok(DataValue::Int64(None)),
            DataType::UInt8 => Ok(DataValue::UInt8(None)),
            DataType::UInt16 => Ok(DataValue::UInt16(None)),
            DataType::UInt32 => Ok(DataValue::UInt32(None)),
            DataType::UInt64 => Ok(DataValue::UInt64(None)),
            DataType::Float32 => Ok(DataValue::Float32(None)),
            DataType::Float64 => Ok(DataValue::Float64(None)),
            DataType::Utf8 => Ok(DataValue::Utf8(None)),
            DataType::Date32 => Ok(DataValue::UInt32(None)),
            DataType::Date64 => Ok(DataValue::UInt64(None)),
            DataType::Timestamp(_, _) => Ok(DataValue::UInt64(None)),
            DataType::Interval(IntervalUnit::YearMonth) => Ok(DataValue::UInt32(None)),
            DataType::Interval(IntervalUnit::DayTime) => Ok(DataValue::UInt64(None)),
            DataType::List(f) => Ok(DataValue::List(None, f.data_type().clone())),
            DataType::Struct(_) => Ok(DataValue::Struct(vec![])),
            DataType::Binary => Ok(DataValue::Binary(None)),
        }
    }
}

impl fmt::Display for DataValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataValue::Null => write!(f, "Null"),
            DataValue::Boolean(v) => format_data_value_with_option!(f, v),
            DataValue::Float32(v) => format_data_value_with_option!(f, v),
            DataValue::Float64(v) => format_data_value_with_option!(f, v),
            DataValue::Int8(v) => format_data_value_with_option!(f, v),
            DataValue::Int16(v) => format_data_value_with_option!(f, v),
            DataValue::Int32(v) => format_data_value_with_option!(f, v),
            DataValue::Int64(v) => format_data_value_with_option!(f, v),
            DataValue::UInt8(v) => format_data_value_with_option!(f, v),
            DataValue::UInt16(v) => format_data_value_with_option!(f, v),
            DataValue::UInt32(v) => format_data_value_with_option!(f, v),
            DataValue::UInt64(v) => format_data_value_with_option!(f, v),
            DataValue::Utf8(v) => format_data_value_with_option!(f, v),
            DataValue::Binary(None) => write!(f, "NULL"),
            DataValue::Binary(Some(v)) => write!(
                f,
                "{}",
                v.iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            DataValue::Date32(v) => format_data_value_with_option!(f, v),
            DataValue::Date64(v) => format_data_value_with_option!(f, v),
            DataValue::TimestampSecond(v) => format_data_value_with_option!(f, v),
            DataValue::TimestampMillisecond(v) => format_data_value_with_option!(f, v),
            DataValue::TimestampMicrosecond(v) => format_data_value_with_option!(f, v),
            DataValue::TimestampNanosecond(v) => format_data_value_with_option!(f, v),
            DataValue::IntervalDayTime(v) => format_data_value_with_option!(f, v),
            DataValue::IntervalYearMonth(v) => format_data_value_with_option!(f, v),
            DataValue::List(None, ..) => write!(f, "NULL"),
            DataValue::List(Some(v), ..) => {
                write!(
                    f,
                    "{}",
                    v.iter()
                        .map(|v| format!("{}", v))
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }
            DataValue::Struct(v) => write!(f, "{:?}", v),
        }
    }
}

impl fmt::Debug for DataValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataValue::Null => write!(f, "Null"),
            DataValue::Boolean(v) => format_data_value_with_option!(f, v),
            DataValue::Int8(v) => format_data_value_with_option!(f, v),
            DataValue::Int16(v) => format_data_value_with_option!(f, v),
            DataValue::Int32(v) => format_data_value_with_option!(f, v),
            DataValue::Int64(v) => format_data_value_with_option!(f, v),
            DataValue::UInt8(v) => format_data_value_with_option!(f, v),
            DataValue::UInt16(v) => format_data_value_with_option!(f, v),
            DataValue::UInt32(v) => format_data_value_with_option!(f, v),
            DataValue::UInt64(v) => format_data_value_with_option!(f, v),
            DataValue::Float32(v) => format_data_value_with_option!(f, v),
            DataValue::Float64(v) => format_data_value_with_option!(f, v),
            DataValue::Utf8(v) => format_data_value_with_option!(f, v),
            DataValue::Binary(None) => write!(f, "{}", self),
            DataValue::Binary(Some(_)) => write!(f, "\"{}\"", self),
            DataValue::Date32(_) => write!(f, "Date32(\"{}\")", self),
            DataValue::Date64(_) => write!(f, "Date64(\"{}\")", self),
            DataValue::IntervalDayTime(_) => {
                write!(f, "IntervalDayTime(\"{}\")", self)
            }
            DataValue::IntervalYearMonth(_) => {
                write!(f, "IntervalYearMonth(\"{}\")", self)
            }
            DataValue::TimestampSecond(_) => write!(f, "TimestampSecond({})", self),
            DataValue::TimestampMillisecond(_) => {
                write!(f, "TimestampMillisecond({})", self)
            }
            DataValue::TimestampMicrosecond(_) => {
                write!(f, "TimestampMicrosecond({})", self)
            }
            DataValue::TimestampNanosecond(_) => {
                write!(f, "TimestampNanosecond({})", self)
            }
            DataValue::List(_, _) => write!(f, "[{}]", self),
            DataValue::Struct(v) => write!(f, "{:?}", v),
        }
    }
}
