use self::{tokenkind::TokenKind, keyword::Keyword};

pub mod keyword;
pub mod tokenkind;
pub mod symbol;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
	Null,
	Yep,
	Nop,

	Int(isize),

	Float(f64),

	Char(char),
	Str(String),

	Keyword(Keyword),
	Identifier(String),

	None,

	Error,
}

pub type TokenLine = u16;

#[derive(Debug, Clone)]
pub struct Token {
	kind:  TokenKind,
	value: TokenValue,
	line:  TokenLine,
}

impl Token {
	pub fn new(kind: TokenKind, value: TokenValue, line: TokenLine) -> Self {
		Self { kind, value, line }
	}

	pub fn kind(&self) -> TokenKind {
		self.kind.clone()
	}

	pub fn value(&self) -> TokenValue {
		self.value.clone()
	}

	pub fn line(&self) -> TokenLine {
		self.line
	}
}
