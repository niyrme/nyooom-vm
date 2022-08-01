use crate::{bytes::ToBytes, value::Value2};

pub type LiteralValue = Value2;

#[derive(Debug)]
pub struct Literal {
	value: LiteralValue,
}

impl Literal {
	pub fn new(value: LiteralValue) -> Self {
		Self { value }
	}

	pub fn value(&self) -> &LiteralValue {
		&self.value
	}
}

impl From<LiteralValue> for Literal {
	fn from(value: LiteralValue) -> Self {
		Self { value }
	}
}

impl ToBytes for Literal {
	fn bytes(&self) -> Vec<u8> {
		self.value.bytes()
	}
}
