#![allow(non_snake_case)]

pub mod ast;
pub mod bytes;
mod instruction;
// mod parser;
pub mod token;
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

#[macro_export]
macro_rules! yeet {
	($err:expr) => {
		return Err(e)
	};
}

pub const MAGIC_NUMBER: [u8; 3] = [0x6e, 0x79, 0x62];
