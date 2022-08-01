use std::{ops::Add, string::String as StdString};

use super::VALUE_STR;
use crate::bytes::ToBytes;

#[derive(Debug, Clone)]
pub struct String {
	value: StdString,
}

impl String {
	fn new(value: StdString) -> Self {
		Self { value }
	}

	pub fn value(&self) -> StdString {
		self.value.clone()
	}

	pub fn len(&self) -> usize {
		self.value.len()
	}
}

impl ToString for String {
	fn to_string(&self) -> StdString {
		self.value.to_string()
	}
}

impl ToBytes for String {
	fn bytes(&self) -> Vec<u8> {
		let mut bytes = Vec::with_capacity(3 + self.len());

		bytes.push(VALUE_STR);
		bytes.extend((self.value.len() as u16).to_le_bytes());
		bytes.extend(self.value.chars().map(|c| c as u8));

		bytes
	}
}

impl<S: Into<StdString>> From<S> for String {
	fn from(s: S) -> Self {
		Self::new(s.into())
	}
}

impl<S: ToString> Add<S> for String {
	type Output = Self;

	fn add(self, rhs: S) -> Self::Output {
		Self::Output::new(self.value + &rhs.to_string())
	}
}
