use std::{
	io::Result,
	ops::{Add, Div, Mul, Sub},
};

use crate::{
	bytes::FromBytes,
	instruction::{Instruction, Instructions},
	value::Value,
	MAGIC_NUMBER,
};

macro_rules! exit {
	($code:expr) => {
		std::process::exit($code)
	};
	($msg:expr, $code:expr) => {{
		eprintln!("{}", $msg);
		std::process::exit($code)
	}};
}

type Stack = Vec<Value>;

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
						Value::Int32(v) => return Ok(v),
						Value::Int64(v) => return Ok(v as i32),
						other => exit!(format!("cannot use {other:?} as exit code"), 1),
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
				other => exit!(format!("instruction not implemented: {other:?}"), 1),
			}
		}
	}

	fn binaryOp(&mut self, op: fn(Value, Value) -> Result<Value>) {
		let b = self.pop();
		let a = self.pop();

		match op(a, b) {
			Ok(result) => self.push(result),
			Err(e) => exit!(e.to_string(), 1),
		}
	}

	fn push(&mut self, value: Value) {
		self.stack.push(value);
	}

	fn pop(&mut self) -> Value {
		self.stack.pop().expect("stack is empty")
	}
}

impl From<Instructions> for VM {
	fn from(code: Instructions) -> Self {
		Self {
			code,
			..Default::default()
		}
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
