use self::node::Node;
use crate::{
	bytes::ToBytes,
	instruction::Instruction,
	token::{symbol::Symbol, tokenkind::TokenKind, Token},
};

pub mod literal;
pub mod node;

macro_rules! exit {
	($code:expr) => {
		std::process::exit($code)
	};
	($code:expr, $msg:expr) => {{
		eprintln!("{}", $msg);
		std::process::exit($code)
	}};
}

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

	pub fn toInstructions(&self) -> Vec<Instruction> {
		let mut instrs = Vec::new();

		instrs.extend(self.toInstruction(self.program()));

		instrs
	}

	fn toInstruction(&self, node: &Node) -> Vec<Instruction> {
		let mut instrs = Vec::new();

		match node {
			Node::BinaryExpression(op, a, b) => {
				instrs.extend(self.toInstruction(a));
				instrs.extend(self.toInstruction(b));
				let i = match op.kind() {
					TokenKind::Symbol(sym) => match sym {
						Symbol::Plus => Instruction::Add,
						Symbol::Minus => Instruction::Sub,
						Symbol::Asterisk => Instruction::Mul,
						Symbol::Slash => Instruction::Div,
						other if [Symbol::Percent, Symbol::Ampersand, Symbol::Pipe].contains(&other) => {
							exit!(1, format!("binary operator not yet implemented: {other:?}"))
						}
						other => exit!(1, format!("inavalid binary operator {other:?}")),
					},
					TokenKind::Compound(symA, symB)
						if [
							(Symbol::Ampersand, Symbol::Ampersand),
							(Symbol::Pipe, Symbol::Pipe),
							(Symbol::Equal, Symbol::Equal),
							(Symbol::Bang, Symbol::Equal),
						]
						.contains(&(symA.clone(), symB.clone())) =>
					{
						exit!(1, format!("binary operator not yet implemented: {:?}", (symA, symB)))
					}
					other => exit!(1, format!("invalid binary operator {other:?}")),
				};
				instrs.push(i);
			}
			Node::Block(_) => todo!(),
			Node::CallExpression(_, _) => todo!(),
			Node::Class(_, _, _) => todo!(),
			Node::DoWhileStatement(_, _) => todo!(),
			Node::EmptyStatement => {}
			Node::ExpressionStatement(_) => todo!(),
			Node::ForStatement(_, _, _, _) => todo!(),
			Node::Function(_, _, _) => todo!(),
			Node::Identifier(_) => todo!(),
			Node::IfStatement(_, _, _) => todo!(),
			Node::Literal(lit) => {
				instrs.push(Instruction::Push(lit.value().clone()));
			}
			Node::MemberExpression(_, _, _) => todo!(),
			Node::PrintExpression(value) => {
				instrs.extend(self.toInstruction(value));
				instrs.push(Instruction::Print);
			}
			Node::Program(body) => {
				for n in body.iter() {
					instrs.extend(self.toInstruction(n));
				}
				instrs.push(Instruction::Halt);
			}
			Node::ReturnStatement(_) => todo!(),
			Node::SuperExpression => todo!(),
			Node::ThisExpression => todo!(),
			Node::UnaryExpression(_, _) => todo!(),
			Node::VariableDeclaration(_, _) => todo!(),
			Node::VariableStatement(_) => todo!(),
			Node::WhileStatement(_, _) => todo!(),
		}

		instrs
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
