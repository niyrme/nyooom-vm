use std::{
	io::Result,
	ops::{Add, Div, Mul, Sub},
};

use crate::{
	ast::AST,
	bytes::{FromBytes, ToBytes},
	instruction::{Instruction, Instructions},
	ValueType,
	MAGIC_NUMBER,
};

macro_rules! exit {
	($code:expr) => {
		std::process::exit($code)
	};
	($code:expr, $msg:expr) => {{
		eprintln!($msg);
		std::process::exit($code)
	}};
}

type Stack = Vec<ValueType>;

pub struct VM {
	code:  Instructions,
	stack: Stack,
}

impl VM {
	pub fn new(code: Instructions) -> Self {
		Self {
			code,
			..Default::default()
		}
	}

	pub fn run(&mut self) -> Result<i32> {
		if self.code[..3] == MAGIC_NUMBER {
			self.code.drain(..3);
		}

		loop {
			match Instruction::fromBytes(&mut self.code) {
				Instruction::Halt => match self.stack.pop() {
					None => return Ok(0),
					Some(val) => match val {
						ValueType::Int32(v) => return Ok(v),
						ValueType::Int64(v) => return Ok(v as i32),
						other => exit!(1, "cannot use {other:?} as exit code"),
					},
				},
				Instruction::Push(v) => self.push(v),
				Instruction::Pop => drop(self.pop()),
				Instruction::Add => self.binaryOp(Add::add),
				Instruction::Sub => self.binaryOp(Sub::sub),
				Instruction::Mul => self.binaryOp(Mul::mul),
				Instruction::Div => self.binaryOp(Div::div),
				Instruction::Print => {
					let val = self.pop();
					print!("{}", val.to_string());
				}
				#[allow(unreachable_patterns)]
				other => exit!(1, "instruction not implemented: {other:?}"),
			}
		}
	}

	fn binaryOp(&mut self, op: fn(ValueType, ValueType) -> Result<ValueType>) {
		let b = self.pop();
		let a = self.pop();

		match op(a, b) {
			Ok(result) => self.push(result),
			Err(e) => exit!(1, "{e}"),
		}
	}

	fn push(&mut self, value: ValueType) {
		self.stack.push(value);
	}

	fn pop(&mut self) -> ValueType {
		self.stack.pop().expect("stack is empty")
	}
}

impl From<Instructions> for VM {
	fn from(code: Instructions) -> Self {
		Self::new(code)
	}
}

impl From<AST> for VM {
	fn from(ast: AST) -> Self {
		Self::new(ast.bytes())
	}
}

impl Default for VM {
	fn default() -> Self {
		Self {
			code:  Instructions::new(),
			stack: Stack::new(),
		}
	}
}
