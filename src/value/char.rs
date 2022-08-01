use super::VALUE_CHAR;
use crate::bytes::ToBytes;

#[derive(Debug)]
pub struct Char {
	value: char,
}

impl Char {
	fn new(value: char) -> Self {
		Self { value }
	}

	pub fn value(&self) -> char {
		self.value
	}
}

impl ToString for Char {
	fn to_string(&self) -> String {
		self.value.to_string()
	}
}

impl ToBytes for Char {
	fn bytes(&self) -> Vec<u8> {
		let mut bytes = Vec::with_capacity(2);
		bytes.push(VALUE_CHAR);
		bytes.push(self.value as u8);

		bytes
	}
}

impl<C: std::convert::Into<char>> From<C> for Char {
	fn from(c: C) -> Self {
		Self::new(c.into())
	}
}
