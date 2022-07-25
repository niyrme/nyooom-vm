use crate::{bytes::ToBytes, token::Token};

pub struct AST {}

impl ToBytes for AST {
	fn bytes(&self) -> Vec<u8> {
		todo!()
	}
}

pub type Tokens = Vec<Token>;
pub trait GenerateAST {
	type Error;

	fn generate(tokens: Tokens) -> Result<AST, Self::Error>;
}
