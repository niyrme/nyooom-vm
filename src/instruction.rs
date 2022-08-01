use crate::{
	bytes::{FromBytes, ToBytes},
	ValueType,
};

// Instruction: 0x0_
pub(crate) const INSTR_HALT: u8 = 0x00;
pub(crate) const INSTR_PUSH: u8 = 0x01;
pub(crate) const INSTR_POP: u8 = 0x02;

pub(crate) const INSTR_ADD: u8 = 0x20;
pub(crate) const INSTR_SUB: u8 = 0x21;
pub(crate) const INSTR_MUL: u8 = 0x22;
pub(crate) const INSTR_DIV: u8 = 0x23;

pub(crate) const INSTR_PRINT: u8 = 0x30;

pub(crate) type Instructions = Vec<u8>;

#[derive(Debug)]
pub enum Instruction {
	Halt,
	Push(ValueType),
	Pop,
	Add,
	Sub,
	Mul,
	Div,
	Print,
}

impl ToBytes for Instruction {
	fn bytes(&self) -> Vec<u8> {
		let mut bytes = Vec::new();

		match &self {
			Self::Halt => bytes.push(INSTR_HALT),
			Self::Push(v) => {
				bytes.push(INSTR_PUSH);
				bytes.extend(v.bytes());
			}
			Self::Pop => bytes.push(INSTR_POP),
			Self::Add => bytes.push(INSTR_ADD),
			Self::Sub => bytes.push(INSTR_SUB),
			Self::Mul => bytes.push(INSTR_MUL),
			Self::Div => bytes.push(INSTR_DIV),
			Self::Print => bytes.push(INSTR_PRINT),
			#[allow(unreachable_patterns)]
			other => panic!("not implemented {other:?}.toBytes()"),
		}

		bytes
	}
}

impl FromBytes for Instruction {
	fn fromBytes(bytes: &mut Vec<u8>) -> Self {
		match bytes.remove(0) {
			INSTR_HALT => Self::Halt,
			INSTR_PUSH => {
				let value = ValueType::fromBytes(bytes);

				Self::Push(value)
			}
			INSTR_POP => Self::Pop,
			INSTR_ADD => Self::Add,
			INSTR_SUB => Self::Sub,
			INSTR_MUL => Self::Mul,
			INSTR_DIV => Self::Div,
			INSTR_PRINT => Self::Print,
			#[allow(unreachable_patterns)]
			other => panic!("invalid instruction: {other:x}"),
		}
	}
}
