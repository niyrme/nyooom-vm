use super::literal::Literal;
use crate::{
	bytes::ToBytes,
	instruction::{INSTR_HALT, INSTR_PRINT, INSTR_PUSH},
	token::Token,
};

#[derive(Debug)]
pub enum Node {
	/// BinaryExpression(OPERATOR, LEFT, RIGHT)
	BinaryExpression(Token, Box<Node>, Box<Node>),
	Block(Vec<Node>),
	CallExpression(Box<Node>, Vec<Node>),
	Class(Box<Node>, Option<Box<Node>>, Box<Node>),
	/// DoWhileStatement(BODY, TEST)
	DoWhileStatement(Box<Node>, Box<Node>),
	EmptyStatement,
	ExpressionStatement(Box<Node>),
	ForStatement(Option<Box<Node>>, Option<Box<Node>>, Option<Box<Node>>, Box<Node>),
	Function(Box<Node>, Vec<Node>, Box<Node>),
	Identifier(Token),
	IfStatement(Box<Node>, Box<Node>, Option<Box<Node>>),
	Literal(Literal),
	/// MemberExpression(COMPUTED?, OBJECT, PROPERTY)
	MemberExpression(bool, Box<Node>, Box<Node>),
	PrintExpression(Box<Node>),
	Program(Vec<Node>),
	ReturnStatement(Option<Box<Node>>),
	SuperExpression,
	ThisExpression,
	/// UnaryExpression(OPERATOR, LEFT)
	UnaryExpression(Token, Box<Node>),
	VariableDeclaration(Box<Node>, Box<Node>),
	VariableStatement(Vec<Node>),
	/// WhileStatement(TEST, BODY)
	WhileStatement(Box<Node>, Box<Node>),
}

impl ToBytes for Node {
	fn bytes(&self) -> Vec<u8> {
		let mut bytes = Vec::new();

		match self {
			Node::BinaryExpression(_, _, _) => todo!(),
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
			Node::Literal(v) => {
				bytes.push(INSTR_PUSH);
				bytes.extend(v.bytes());
			}
			Node::MemberExpression(_, _, _) => todo!(),
			Node::PrintExpression(v) => {
				bytes.extend(v.bytes());
				bytes.push(INSTR_PRINT);
			}
			Node::Program(body) => {
				body.iter().for_each(|node| bytes.extend(node.bytes()));
				bytes.push(INSTR_HALT);
			}
			Node::ReturnStatement(_) => todo!(),
			Node::SuperExpression => todo!(),
			Node::ThisExpression => todo!(),
			Node::UnaryExpression(_, _) => todo!(),
			Node::VariableDeclaration(_, _) => todo!(),
			Node::VariableStatement(_) => todo!(),
			Node::WhileStatement(_, _) => todo!(),
		}

		bytes
	}
}
