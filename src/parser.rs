use std::io::Result;

use crate::parser::tokenizer::Tokenizer;

mod keyword;
pub(crate) mod token;
mod tokenizer;
mod tokenkind;

pub(crate) struct Parser {}

impl Parser {
	#[allow(unreachable_code)]
	pub(crate) fn parse(text: String) -> Result<AST> {
		let tokens = Tokenizer::new().tokenize(text.as_bytes().to_vec().iter().peekable())?;
		todo!("parse")
	}
}
