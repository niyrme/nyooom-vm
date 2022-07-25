use std::io;

use crate::Err;

#[derive(Debug, Clone)]
pub enum Keyword {
	Let,
	If,
	Else,
	While,
	Do,
	For,
	Def,
	Return,
	Class,
	This,
	Super,
	Print,
}

impl TryFrom<&str> for Keyword {
	type Error = io::Error;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		match s {
			"let" => Ok(Self::Let),
			"if" => Ok(Self::If),
			"else" => Ok(Self::Else),
			"while" => Ok(Self::While),
			"do" => Ok(Self::Do),
			"for" => Ok(Self::For),
			"def" => Ok(Self::Def),
			"return" => Ok(Self::Return),
			"class" => Ok(Self::Class),
			"this" => Ok(Self::This),
			"super" => Ok(Self::Super),
			"print" => Ok(Self::Print),
			other => Err!(format!("failed get Keyword from '{other}'")),
		}
	}
}

impl TryFrom<String> for Keyword {
	type Error = io::Error;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::try_from(value.as_str())
	}
}
