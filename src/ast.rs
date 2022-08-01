use self::node::Node;
use crate::{bytes::ToBytes, token::Token};

pub mod literal;
pub mod node;

#[derive(Debug)]
pub struct AST {
	program: Node,
}

impl AST {
	pub fn new(program: Node) -> Self {
		Self { program }
	}

	pub fn program(&self) -> &Node {
		&self.program
	}
}

impl From<Node> for AST {
	fn from(program: Node) -> Self {
		Self { program }
	}
}

impl ToBytes for AST {
	fn bytes(&self) -> Vec<u8> {
		self.program.bytes()
	}
}

pub type Tokens = Vec<Token>;
pub trait GenerateAST {
	type Error;

	fn generate(tokens: Tokens) -> Result<AST, Self::Error>;
}
