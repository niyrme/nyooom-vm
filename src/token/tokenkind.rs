use std::io;

use crate::Err;

use super::symbol::Symbol;

type TryFromError = io::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
	Null,
	Bool,
	Int,
	Float,
	Char,
	Str,

	Keyword,
	Identifier,

	Symbol(Symbol),
	Compound(Symbol, Symbol),

	EOF,
	Err(String),
}

impl TryFrom<char> for TokenKind {
	type Error = TryFromError;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'=' => Ok(Self::Symbol(Symbol::Equal)),
			'!' => Ok(Self::Symbol(Symbol::Bang)),
			'+' => Ok(Self::Symbol(Symbol::Plus)),
			'-' => Ok(Self::Symbol(Symbol::Minus)),
			'*' => Ok(Self::Symbol(Symbol::Asterisk)),
			'/' => Ok(Self::Symbol(Symbol::Slash)),
			'^' => Ok(Self::Symbol(Symbol::Caret)),
			'%' => Ok(Self::Symbol(Symbol::Percent)),
			'<' => Ok(Self::Symbol(Symbol::Lesser)),
			'>' => Ok(Self::Symbol(Symbol::Greater)),
			'.' => Ok(Self::Symbol(Symbol::Dot)),
			':' => Ok(Self::Symbol(Symbol::Colon)),
			',' => Ok(Self::Symbol(Symbol::Comma)),
			';' => Ok(Self::Symbol(Symbol::Semicolon)),
			'(' => Ok(Self::Symbol(Symbol::LParen)),
			')' => Ok(Self::Symbol(Symbol::RParen)),
			'{' => Ok(Self::Symbol(Symbol::LBrace)),
			'}' => Ok(Self::Symbol(Symbol::RBrace)),
			'[' => Ok(Self::Symbol(Symbol::LBracket)),
			']' => Ok(Self::Symbol(Symbol::RBracket)),
			other => Err!(format!("failed to get TokenKind from '{other}'")),
		}
	}
}

impl TryFrom<u8> for TokenKind {
	type Error = TryFromError;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		Self::try_from(value as char)
	}
}

impl TryFrom<&str> for TokenKind {
	type Error = TryFromError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"&&" => Ok(Self::Compound(Symbol::Ampersand, Symbol::Ampersand)),
			"||" => Ok(Self::Compound(Symbol::Pipe, Symbol::Pipe)),
			other if other.len() == 1 => Self::try_from(other.chars().nth(0).unwrap()),
			other => Err!(format!("failed to get TokenKind from '{other}'")),
		}
	}
}

impl TryFrom<String> for TokenKind {
	type Error = TryFromError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::try_from(value.as_str())
	}
}
