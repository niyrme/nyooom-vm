#![allow(non_snake_case)]

use value::Value2;

pub mod ast;
pub mod bytes;
mod instruction;
pub mod token;
pub mod tokenizer;
mod value;
pub mod vm;

#[macro_export]
macro_rules! err {
	($msg:expr) => {
		std::io::Error::new(std::io::ErrorKind::Other, $msg)
	};
	($msg:expr, $kind:expr) => {
		std::io::Error::new($kind, $msg)
	};
}

#[macro_export]
macro_rules! Err {
	($msg:expr) => {
		Err(std::io::Error::new(std::io::ErrorKind::Other, $msg))
	};
	($msg:expr, $kind:expr) => {
		Err(std::io::Error::new($kind, $msg))
	};
}

pub(crate) type ValueType = Value2;

pub const MAGIC_NUMBER: [u8; 3] = [0x6e, 0x79, 0x62];
