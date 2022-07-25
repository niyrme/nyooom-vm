use std::{
	io::Result,
	ops::{Add, Div, Mul, Sub},
};

use crate::{
	bytes::{FromBytes, ToBytes},
	Err,
};

// Value: 0x1_
const VALUE_NULL: u8 = 0x10;
const VALUE_TRUE: u8 = 0x11;
const VALUE_FALSE: u8 = 0x12;
const VALUE_INT32: u8 = 0x13;
const VALUE_INT64: u8 = 0x14;
const VALUE_FLOAT: u8 = 0x15;
const VALUE_CHAR: u8 = 0x16;
const VALUE_STR: u8 = 0x17;

#[derive(Debug)]
pub enum Value {
	Null,
	True,
	False,
	Int32(i32),
	Int64(i64),
	Float(f64),

	Char(char),
	Str(String),
}

impl ToString for Value {
	fn to_string(&self) -> String {
		match self {
			Value::Null => String::from("null"),
			Value::True => String::from("true"),
			Value::False => String::from("false"),
			Value::Int32(v) => v.to_string(),
			Value::Int64(v) => v.to_string(),
			Value::Float(v) => v.to_string(),
			Value::Char(v) => v.to_string(),
			Value::Str(v) => v.to_string(),

			#[allow(unreachable_patterns)]
			other => todo!("{other:?}.to_string()"),
		}
	}
}

impl ToBytes for Value {
	fn bytes(&self) -> Vec<u8> {
		let mut bytes = Vec::new();

		match &self {
			Self::Null => bytes.push(VALUE_NULL),
			Self::True => bytes.push(VALUE_TRUE),
			Self::False => bytes.push(VALUE_FALSE),
			Self::Int32(v) => {
				bytes.push(VALUE_INT32);
				bytes.extend(v.to_le_bytes());
			}
			Self::Int64(v) => {
				bytes.push(VALUE_INT64);
				bytes.extend(v.to_le_bytes());
			}
			Self::Float(v) => {
				bytes.push(VALUE_FLOAT);
				bytes.extend(v.to_le_bytes());
			}
			Self::Char(c) => {
				bytes.push(VALUE_CHAR);
				bytes.push(*c as u8);
			}
			Self::Str(s) => {
				bytes.push(VALUE_STR);
				bytes.extend((s.len() as u16).to_le_bytes());
				bytes.extend(s.chars().map(|c| c as u8));
			}
			#[allow(unreachable_patterns)]
			other => todo!("{other:?}"),
		}

		bytes
	}
}

impl FromBytes for Value {
	fn fromBytes(bytes: &mut Vec<u8>) -> Self {
		match bytes.remove(0) {
			VALUE_NULL => Self::Null,
			VALUE_TRUE => Self::True,
			VALUE_FALSE => Self::False,
			VALUE_INT32 => {
				let mut valBytes = [0; 4];
				(0..4).for_each(|idx| valBytes[idx] = bytes.remove(0));

				Self::Int32(i32::from_le_bytes(valBytes))
			}
			VALUE_INT64 => {
				let mut valBytes = [0; 8];
				(0..8).for_each(|idx| valBytes[idx] = bytes.remove(0));

				Self::Int64(i64::from_le_bytes(valBytes))
			}
			VALUE_FLOAT => {
				let mut valBytes = [0; 8];
				(0..8).for_each(|idx| valBytes[idx] = bytes.remove(0));

				Self::Float(f64::from_le_bytes(valBytes))
			}
			VALUE_CHAR => Self::Char(bytes.remove(0) as char),
			VALUE_STR => {
				let strSizeBytes = [bytes.remove(0), bytes.remove(0)];
				let strSize = u16::from_le_bytes(strSizeBytes) as usize;

				let mut strBytes = Vec::new();

				(0..strSize).for_each(|_| strBytes.push(bytes.remove(0)));

				let s = String::from_utf8(strBytes).expect("failed to read string");

				Self::Str(s)
			}
			#[allow(unreachable_patterns)]
			other => todo!("{other:?}"),
		}
	}
}

impl Add for Value {
	type Output = Result<Self>;

	fn add(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Self::Int32(a), Self::Int32(b)) => Ok(Self::Int32(a + b)),
			(Self::Int32(a), Self::Int64(b)) => Ok(Self::Int64(a as i64 + b)),
			(Self::Int32(a), Self::Float(b)) => Ok(Self::Float(a as f64 + b)),
			(Self::Int32(a), Self::True) => Ok(Self::Int32(a + 1)),
			(Self::Int32(a), Self::False) => Ok(Self::Int32(a)),

			(Self::Int64(a), Self::Int32(b)) => Ok(Self::Int64(a + b as i64)),
			(Self::Int64(a), Self::Int64(b)) => Ok(Self::Int64(a + b)),
			(Self::Int64(a), Self::Float(b)) => Ok(Self::Float(a as f64 + b)),
			(Self::Int64(a), Self::True) => Ok(Self::Int64(a + 1)),
			(Self::Int64(a), Self::False) => Ok(Self::Int64(a)),

			(Self::Float(a), Self::Float(b)) => Ok(Self::Float(a + b)),
			(Self::Float(a), Self::Int32(b)) => Ok(Self::Float(a + b as f64)),
			(Self::Float(a), Self::Int64(b)) => Ok(Self::Float(a + b as f64)),
			(Self::Float(a), Self::True) => Ok(Self::Float(a + 1.0)),
			(Self::Float(a), Self::False) => Ok(Self::Float(a)),

			(Self::Str(a), Self::Int32(b)) => Ok(Self::Str(a + b.to_string().as_str())),
			(Self::Str(a), Self::Int64(b)) => Ok(Self::Str(a + b.to_string().as_str())),
			(Self::Str(a), Self::Float(b)) => Ok(Self::Str(a + b.to_string().as_str())),
			(Self::Str(a), Self::True) => Ok(Self::Str(a + "true")),
			(Self::Str(a), Self::False) => Ok(Self::Str(a + "false")),
			(Self::Str(a), Self::Char(b)) => Ok(Self::Str(a + b.to_string().as_str())),
			(Self::Str(a), Self::Str(b)) => Ok(Self::Str(a + b.as_str())),

			(otherA, otherB) => Err!(format!("invalid operation {otherA:?} + {otherB:?}")),
		}
	}
}

impl Sub for Value {
	type Output = Result<Self>;

	fn sub(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Self::Int32(a), Self::Int32(b)) => Ok(Self::Int32(a - b)),
			(Self::Int32(a), Self::Int64(b)) => Ok(Self::Int64(a as i64 - b)),
			(Self::Int32(a), Self::Float(b)) => Ok(Self::Float(a as f64 - b)),
			(Self::Int32(a), Self::True) => Ok(Self::Int32(a - 1)),
			(Self::Int32(a), Self::False) => Ok(Self::Int32(a)),

			(Self::Int64(a), Self::Int32(b)) => Ok(Self::Int64(a - b as i64)),
			(Self::Int64(a), Self::Int64(b)) => Ok(Self::Int64(a - b)),
			(Self::Int64(a), Self::Float(b)) => Ok(Self::Float(a as f64 - b)),
			(Self::Int64(a), Self::True) => Ok(Self::Int64(a - 1)),
			(Self::Int64(a), Self::False) => Ok(Self::Int64(a)),

			(Self::Float(a), Self::Float(b)) => Ok(Self::Float(a - b)),
			(Self::Float(a), Self::Int32(b)) => Ok(Self::Float(a - b as f64)),
			(Self::Float(a), Self::Int64(b)) => Ok(Self::Float(a - b as f64)),
			(Self::Float(a), Self::True) => Ok(Self::Float(a - 1.0)),
			(Self::Float(a), Self::False) => Ok(Self::Float(a)),

			(otherA, otherB) => Err!(format!("invalid operation {otherA:?} - {otherB:?}")),
		}
	}
}

impl Mul for Value {
	type Output = Result<Self>;

	fn mul(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Self::Int32(a), Self::Int32(b)) => Ok(Self::Int32(a * b)),
			(Self::Int32(a), Self::Int64(b)) => Ok(Self::Int64(a as i64 * b)),
			(Self::Int32(a), Self::Float(b)) => Ok(Self::Float(a as f64 * b)),
			(Self::Int32(a), Self::True) => Ok(Self::Int32(a)),
			(Self::Int32(_), Self::False) => Ok(Self::Int32(0)),

			(Self::Int64(a), Self::Int32(b)) => Ok(Self::Int64(a * b as i64)),
			(Self::Int64(a), Self::Int64(b)) => Ok(Self::Int64(a * b)),
			(Self::Int64(a), Self::Float(b)) => Ok(Self::Float(a as f64 * b)),
			(Self::Int64(a), Self::True) => Ok(Self::Int64(a)),
			(Self::Int64(_), Self::False) => Ok(Self::Int64(0)),

			(Self::Float(a), Self::Float(b)) => Ok(Self::Float(a * b)),
			(Self::Float(a), Self::Int32(b)) => Ok(Self::Float(a * b as f64)),
			(Self::Float(a), Self::Int64(b)) => Ok(Self::Float(a * b as f64)),
			(Self::Float(a), Self::True) => Ok(Self::Float(a)),
			(Self::Float(_), Self::False) => Ok(Self::Float(0.0)),

			(otherA, otherB) => Err!(format!("invalid operation {otherA:?} * {otherB:?}")),
		}
	}
}

impl Div for Value {
	type Output = Result<Self>;

	fn div(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Self::Int32(_) | Self::Int64(_) | Self::Float(_), Self::Int32(0) | Self::Int64(0) | Self::False) => {
				Err!("cannot divide by 0")
			}
			(Self::Int32(_) | Self::Int64(_) | Self::Float(_), Self::Float(v)) if v == 0.0 => Err!("cannot divide by 0"),
			(Self::Int32(a), Self::Int32(b)) => Ok(Self::Int32(a / b)),
			(Self::Int32(a), Self::Int64(b)) => Ok(Self::Int64(a as i64 / b)),
			(Self::Int32(a), Self::Float(b)) => Ok(Self::Float(a as f64 / b)),
			(Self::Int32(a), Self::True) => Ok(Self::Int32(a)),

			(Self::Int64(a), Self::Int32(b)) => Ok(Self::Int64(a / b as i64)),
			(Self::Int64(a), Self::Int64(b)) => Ok(Self::Int64(a / b)),
			(Self::Int64(a), Self::Float(b)) => Ok(Self::Float(a as f64 / b)),
			(Self::Int64(a), Self::True) => Ok(Self::Int64(a)),

			(Self::Float(a), Self::Float(b)) => Ok(Self::Float(a / b)),
			(Self::Float(a), Self::Int32(b)) => Ok(Self::Float(a / b as f64)),
			(Self::Float(a), Self::Int64(b)) => Ok(Self::Float(a / b as f64)),
			(Self::Float(a), Self::True) => Ok(Self::Float(a)),

			(otherA, otherB) => Err!(format!("invalid operation {otherA:?} / {otherB:?}")),
		}
	}
}

impl From<bool> for Value {
	fn from(b: bool) -> Self {
		if b { Self::True } else { Self::False }
	}
}

impl From<i32> for Value {
	fn from(v: i32) -> Self {
		Self::Int32(v)
	}
}

impl From<i64> for Value {
	fn from(v: i64) -> Self {
		Self::Int64(v)
	}
}

impl From<f64> for Value {
	fn from(v: f64) -> Self {
		Self::Float(v)
	}
}

impl From<char> for Value {
	fn from(c: char) -> Self {
		Self::Char(c)
	}
}

impl From<String> for Value {
	fn from(s: String) -> Self {
		Self::Str(s)
	}
}
