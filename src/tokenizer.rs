use std::{
	io::{ErrorKind, Result},
	iter::Peekable,
	slice::Iter,
};

use crate::{
	err,
	token::{keyword::Keyword, symbol::Symbol, tokenkind::TokenKind, Token, TokenLine, TokenValue},
	Err,
};

pub type Stream<'a> = Peekable<Iter<'a, u8>>;

pub struct Tokenizer {
	line: TokenLine,
}

impl Tokenizer {
	pub fn tokenize(stream: Stream) -> Result<Vec<Token>> {
		Self::new()._tokenize(stream)
	}

	fn new() -> Self {
		Self { line: 1 }
	}

	fn _tokenize(&mut self, mut stream: Stream) -> Result<Vec<Token>> {
		let mut error = false;
		let mut tokens = Vec::new();

		loop {
			let token = self.makeToken(&mut stream);

			let token = match token {
				Ok(tk) => tk,
				Err(e) => {
					eprintln!("{e}");
					error = true;
					Token::new(TokenKind::Err(e.to_string()), TokenValue::Error, self.line)
				}
			};

			if token.kind().eq(&TokenKind::EOF) {
				tokens.push(token);
				break;
			}
			tokens.push(token);
		}

		if error { Err!("failed to tokenize") } else { Ok(tokens) }
	}

	fn makeToken(&mut self, stream: &mut Stream) -> Result<Token> {
		let byte = if let Some(b) = stream.peek() {
			*b
		} else {
			return Ok(Token::new(TokenKind::EOF, TokenValue::None, self.line));
		};

		let punctuation = vec![
			b'=', b'!', b'+', b'-', b'*', b'/', b'&', b'|', b'^', b'%', b'<', b'>', b'.', b':', b',', b';', b'(', b')', b'{',
			b'}', b'[', b']',
		];
		let whitespace = vec![b' ', b'\t', b'\r'];

		match byte {
			b'0'..=b'9' => self.makeNumber(stream),
			b'\'' => self.makeChar(stream),
			b'"' => self.makeString(stream),
			b'_' | b'a'..=b'z' | b'A'..=b'Z' => self.makeKeyword(stream),
			other if punctuation.contains(other) => self.makePunctuation(stream),
			b'\n' => {
				stream.next().unwrap();
				self.line += 1;
				self.makeToken(stream)
			}
			other if whitespace.contains(other) => {
				stream.next().unwrap();
				self.makeToken(stream)
			}
			other => Err!(format!("invalid character {}", *other as char)),
		}
	}

	fn makeNumber(&mut self, stream: &mut Stream) -> Result<Token> {
		let mut numS = String::new();

		let mut isFloat = false;

		while let Some(c) = stream.peek() {
			if !c.is_ascii_digit() {
				break;
			}
			numS.push(*stream.next().unwrap() as char);
		}

		if stream.peek() == Some(&&b'.') {
			isFloat = true;
			numS.push(*stream.next().unwrap() as char);
			while let Some(c) = stream.peek() {
				if !c.is_ascii_digit() {
					break;
				}
				numS.push(*stream.next().unwrap() as char);
			}
		}

		let token = if isFloat {
			Token::new(
				TokenKind::Float,
				TokenValue::Float(numS.parse().expect("failed to parse float")),
				self.line,
			)
		} else {
			Token::new(
				TokenKind::Int,
				TokenValue::Int(numS.parse().expect("failed to parse int")),
				self.line,
			)
		};

		return Ok(token);
	}

	fn makeKeyword(&mut self, stream: &mut Stream) -> Result<Token> {
		let mut kw = String::new();

		while let Some(c) = stream.peek() {
			if !(c.is_ascii_alphabetic() || c.eq(&&b'_')) {
				break;
			} else {
				kw.push(*stream.next().unwrap() as char);
			}
		}

		match Keyword::try_from(kw.to_string()) {
			Ok(v) => Ok(Token::new(TokenKind::Keyword, TokenValue::Keyword(v), self.line)),
			Err(_) => {
				let tokenKV = match kw.as_str() {
					"null" => Some((TokenKind::Null, TokenValue::Null)),
					"true" => Some((TokenKind::Bool, TokenValue::True)),
					"false" => Some((TokenKind::Bool, TokenValue::False)),
					_ => None,
				};

				let token = match tokenKV {
					Some((kind, value)) => Token::new(kind, value, self.line),
					None => Token::new(TokenKind::Identifier, TokenValue::Identifier(kw), self.line),
				};

				Ok(token)
			}
		}
	}

	fn makeChar(&mut self, stream: &mut Stream) -> Result<Token> {
		// consume '
		stream.next();

		let chr = stream
			.next()
			.ok_or(err!("expected char, found EOF", ErrorKind::UnexpectedEof))?;

		let value = if chr.eq(&b'\\') {
			let escaped = stream
				.next()
				.ok_or(err!("expected char, found EOF", ErrorKind::UnexpectedEof))?;

			match escaped {
				b'0' => '\0',
				b'n' => '\n',
				b'r' => '\r',
				b't' => '\t',
				b'\'' => '\'',
				b'\\' => '\\',
				other => {
					if stream
						.next()
						.ok_or(err!("expected closing quote, found EOF", ErrorKind::UnexpectedEof))?
						.eq(&b'\'')
					{
						stream.next();
					}

					return Err!(format!("invalid escaped char '\\{other}'"));
				}
			}
		} else {
			*chr as char
		};

		match stream.next() {
			Some(v) if v.eq(&b'\'') => Ok(Token::new(TokenKind::Char, TokenValue::Char(value), self.line)),
			Some(other) => Err!(format!("expected closing quote, got '{}'", *other as char)),
			None => Err!("expected closing quote, found EOF", ErrorKind::UnexpectedEof),
		}
	}

	fn makeString(&mut self, stream: &mut Stream) -> Result<Token> {
		// consume "
		stream.next();

		let mut str = String::new();

		while let Some(chr) = stream.next() {
			match chr {
				b'"' => return Ok(Token::new(TokenKind::Str, TokenValue::Str(str), self.line)),
				b'\n' => {
					self.line += 1;
					str.push('\n');
				}
				b'\\' => {
					let c = stream.next().ok_or(err!("unexpected EOF"))?;

					str += match c {
						b'0' => Ok("\0"),
						b'n' => Ok("\n"),
						b'r' => Ok("\r"),
						b't' => Ok("\t"),
						b'"' => Ok("\""),
						b'a' => Ok("\\a"),
						b'b' => Ok("\\b"),
						b'f' => Ok("\\f"),
						b'v' => Ok("\\v"),
						b'\\' => Ok("\\"),
						b'x' => todo!("ascii escape not supporter"),
						other => Err!(format!("invalid escape sequence in string: '\\{}'", *other as char)),
					}?;
				}
				other => str.push(*other as char),
			}
		}

		Err!("unterminated string", ErrorKind::UnexpectedEof)
	}

	fn makePunctuation(&mut self, stream: &mut Stream) -> Result<Token> {
		let p = stream.next().unwrap();

		match (p, stream.peek()) {
			(b'/', Some(b'/')) => {
				self.skipSingleComment(stream)?;
				self.makeToken(stream)
			}
			(b'/', Some(b'*')) => {
				stream.next();
				self.skipMultiComment(stream)?;
				self.makeToken(stream)
			}
			(b'&', Some(b'&')) => Ok(Token::new(
				TokenKind::Compound(Symbol::Ampersand, Symbol::Ampersand),
				TokenValue::None,
				self.line,
			)),
			(b'|', Some(b'|')) => Ok(Token::new(
				TokenKind::Compound(Symbol::Pipe, Symbol::Pipe),
				TokenValue::None,
				self.line,
			)),
			(other, _) => {
				let kind = TokenKind::try_from(*other)?;
				Ok(Token::new(kind, TokenValue::None, self.line))
			}
		}
	}

	fn skipSingleComment(&mut self, stream: &mut Stream) -> Result<()> {
		while let Some(c) = stream.next() {
			match c {
				b'\n' => {
					self.line += 1;
					break;
				}
				b'\0' => break,
				_ => {}
			}
		}

		Ok(())
	}

	fn skipMultiComment(&mut self, stream: &mut Stream) -> Result<()> {
		loop {
			let current = stream
				.next()
				.ok_or(err!("unterminated multi-line comment", ErrorKind::UnexpectedEof))?;

			match (current, stream.next()) {
				(b'*', Some(b'/')) => return Ok(()),
				(b'/', Some(b'*')) => self.skipMultiComment(stream)?,
				(_, None) => break,
				_ => {}
			}
		}

		Err!("unterminated multi-line comment", ErrorKind::UnexpectedEof)
	}
}
