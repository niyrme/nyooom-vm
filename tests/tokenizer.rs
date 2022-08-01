use std::io;

use nyooom_vm::{
	token::{tokenkind::TokenKind, Token, TokenValue},
	tokenizer::Tokenizer,
};

type R = io::Result<()>;

fn t<'a>(s: &'a str) -> io::Result<Vec<Token>> {
	Tokenizer::tokenize(s.as_bytes().iter().peekable())
}

#[test]
fn test_empty() -> R {
	assert_eq!(t("")?, vec![Token::new(TokenKind::EOF, TokenValue::None, 1)]);

	Ok(())
}

#[test]
fn test_null() -> R {
	assert_eq!(t("null")?, vec![
		Token::new(TokenKind::Null, TokenValue::Null, 1),
		Token::new(TokenKind::EOF, TokenValue::None, 1)
	]);

	Ok(())
}

#[test]
fn test_bool() -> R {
	assert_eq!(t("true")?, vec![
		Token::new(TokenKind::Bool, TokenValue::True, 1),
		Token::new(TokenKind::EOF, TokenValue::None, 1)
	]);
	assert_eq!(t("false")?, vec![
		Token::new(TokenKind::Bool, TokenValue::False, 1),
		Token::new(TokenKind::EOF, TokenValue::None, 1)
	]);

	Ok(())
}

#[test]
fn test_int() -> R {
	assert_eq!(t("42")?, vec![
		Token::new(TokenKind::Int, TokenValue::Int(42), 1),
		Token::new(TokenKind::EOF, TokenValue::None, 1)
	]);

	Ok(())
}

#[test]
fn test_float() -> R {
	assert_eq!(t("3.14159")?, vec![
		Token::new(TokenKind::Float, TokenValue::Float(3.14159), 1),
		Token::new(TokenKind::EOF, TokenValue::None, 1)
	]);

	Ok(())
}

#[test]
fn test_char() -> R {
	for c in ['a', '\n', '\0'] {
		assert_eq!(t(format!("'{}'", &c).as_str())?, vec![
			Token::new(TokenKind::Char, TokenValue::Char(c), 1),
			Token::new(TokenKind::EOF, TokenValue::None, 1)
		])
	}

	Ok(())
}

#[test]
fn test_string() -> R {
	for s in ["", "Hello World!", "true"] {
		assert_eq!(t(format!("{:?}", &s).as_str())?, vec![
			Token::new(TokenKind::Str, TokenValue::Str(s.to_string()), 1),
			Token::new(TokenKind::EOF, TokenValue::None, 1)
		])
	}

	Ok(())
}
