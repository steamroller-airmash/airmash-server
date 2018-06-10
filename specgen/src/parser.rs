
use std::str::{self, Utf8Error};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenData<'a> {
	Ident(&'a str),
	LBracket, // {
	RBracket, // }
	Colon,    // :
	LBrace,   // [
	RBrace,   // ]
	Comma,    // ,
	Equals,   // =
	Integer(u32),
	DocComment(&'a str),
}

#[derive(Copy, Clone, Debug)]
struct Token<'a> {
	pub ty: TokenData<'a>,
	pub line: u32
}

#[derive(Clone, Debug)]
pub enum SpecFieldType<'a> {
	Simple(&'a str),
	Compound(&'a str, Vec<SpecFieldType<'a>>)
}

impl<'a> SpecFieldType<'a> {
	pub fn to_str(&self) -> String {
		match self {
			SpecFieldType::Simple(s) => s.to_string(),
			SpecFieldType::Compound(n, v) => {
				let v = v.iter().map(|x| x.to_str()).collect::<Vec<String>>();
				n.to_string() + "<" + &v.join(",") + ">"
			}
		}
	}
}

#[derive(Clone, Debug)]
pub struct SpecField<'a> {
	pub name: &'a str,
	pub ty:   SpecFieldType<'a>,
	pub docs: Vec<&'a str>
}

#[derive(Clone, Default, Debug)]
pub struct SpecDefinition<'a> {
	pub name: &'a str,
	pub docs: Vec<&'a str>,
	pub fields: Vec<SpecField<'a>>
}

#[derive(Clone, Debug, Default)]
pub struct Spec<'a> {
	pub name: &'a str,
	pub docs: Vec<&'a str>,
	pub defs: Vec<SpecDefinition<'a>>,
}

#[derive(Clone, Debug)]
pub struct EnumBranch<'a> {
	pub name:  &'a str,
	pub docs:  Vec<&'a str>,
	pub value: u32
}

#[derive(Clone, Debug)]
pub enum TypeClass<'a> {
	//Extern,
	//TypeDef(Box<SpecFieldType<'a>>),
	Enum(Vec<EnumBranch<'a>>, SpecFieldType<'a>)
}

#[derive(Clone, Debug)]
pub struct Type<'a> {
	pub name: &'a str,
	pub docs: Vec<&'a str>,
	pub class: TypeClass<'a>
}

#[derive(Clone, Debug, Default)]
pub struct Protocol<'a> {
	pub specs: Vec<Spec<'a>>,
	pub types: Vec<Type<'a>>
}

#[derive(Copy, Clone, Debug)]
pub enum ErrorType<'a> {
	UnexpectedEOF,
	NonAsciiCodepoint(u8),
	UnexpectedChar(char),
	Utf8Error(Utf8Error),
	ExpectingIdent(TokenData<'a>),
	ExpectingKeyword(&'static str, TokenData<'a>),
	ExpectingRBracket(TokenData<'a>),
	ExpectingColon(TokenData<'a>),
	ExpectingComma(TokenData<'a>),
	ExpectingEquals(TokenData<'a>),
	InvalidIntegerLiteral(&'a str)
}

#[derive(Copy, Clone, Debug)]
pub struct Error<'a> {
	pub ty: ErrorType<'a>,
	pub line: u32
}

impl<'a> Error<'a> {
	pub fn new(ty: ErrorType<'a>, line: u32) -> Self {
		Self { ty, line }
	}
}

struct Tokens<'a> {
	tokens: Vec<Token<'a>>,
	index: usize
}

impl<'a> Tokens<'a> {
	fn tokenize(b: &'a [u8]) -> Result<Vec<Token<'a>>, Error<'a>> {
		let mut i = 0;
		let mut line = 1;
		let mut result = vec![];

		while i < b.len() {
			if b[i] > 127 {
				return Err(Error::new(ErrorType::NonAsciiCodepoint(b[i]), line))
			}

			let data = match b[i] as char {
				'\n' => {
					line += 1;
					i += 1;
					continue;
				},
				':' => Token{ line, ty: TokenData::Colon },
				',' => Token{ line, ty: TokenData::Comma },
				'{' => Token{ line, ty: TokenData::LBracket },
				'}' => Token{ line, ty: TokenData::RBracket },
				'[' => Token{ line, ty: TokenData::LBrace },
				']' => Token{ line, ty: TokenData::RBrace },
				'=' => Token{ line, ty: TokenData::Equals },
				'a'...'z' | 'A'...'Z' => {
					let mut len = 1;

					loop {
						if i + len == b.len() {
							break;
						}
						if b[i + len] > 127 {
							return Err(Error::new(
								ErrorType::NonAsciiCodepoint(b[i+len]), line
							));
						}

						match b[i + len] as char {
							'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => (),
							_ => break
						}

						len += 1
					}

					let s = match str::from_utf8(&b[i..i+len]) {
						Ok(s) => s,
						Err(e) => return Err(Error::new(
							ErrorType::Utf8Error(e), 
							line
						))
					};

					i += len - 1;

					Token {	ty: TokenData::Ident(s), line }
				},
				'#' => {
					let mut len = 0;
					
					loop {						
						if b[i + len] > 127 {
							return Err(Error::new(
								ErrorType::NonAsciiCodepoint(b[i + len]), line
							));
						}

						match b[i + len] as char {
							'\n' => break,
							'\r' => break,
							_ => len += 1,
						};
					}

					let s = match str::from_utf8(&b[i+1..i+len]) {
						Ok(s) => s,
						Err(e) => return Err(Error::new(
							ErrorType::Utf8Error(e), line
						))
					};

					i += len;

					Token {
						ty: TokenData::DocComment(s),
						line
					}
				},
				'\r' | ' ' | '\t' => {
					i += 1;
					continue;
				},
				'0'...'9' => {
					let mut len = 1;

					loop {
						if i + len == b.len() {
							break;
						}
						if b[i + len] > 127 {
							return Err(Error::new(ErrorType::NonAsciiCodepoint(b[i+1]), line));
						}

						match b[i + len] as char {
							'0'...'9' => len += 1,
							'A'...'Z' | 'a'...'z' | '_' => { 
								let s = match str::from_utf8(&b[i..i+len]) {
									Ok(s) => s,
									Err(e) => return Err(Error::new(
										ErrorType::Utf8Error(e), 
										line
									))
								};
								
								return Err(Error::new(
									ErrorType::InvalidIntegerLiteral(s),
									line
								));
							},
							_ => break
						}
					}
					
					let s = match str::from_utf8(&b[i..i+len]) {
						Ok(s) => s,
						Err(e) => return Err(Error::new(
							ErrorType::Utf8Error(e), 
							line
						))
					};

					let num = match s.parse() {
						Ok(n) => n,
						Err(_) => return Err(Error::new(
							ErrorType::InvalidIntegerLiteral(s),
							line
						))
					};

					i += len-1;

					Token{ ty: TokenData::Integer(num), line }
				}
				_ => return Err(Error::new(ErrorType::UnexpectedChar(b[i] as char), line))
			};

			result.push(data);

			i += 1;
		}

		Ok(result)
	}

	pub fn new(b: &'a [u8]) -> Result<Self, Error<'a>> {
		Ok(Self { 
			tokens: Self::tokenize(b)?,
			index: 0
		})
	}

	pub fn current(&self) -> Result<Token<'a>, Error<'a>> {
		if self.tokens.len() <= self.index {
			return Err(Error{ 
				ty: ErrorType::UnexpectedEOF, 
				line: self.tokens.last().map(|x| x.line).unwrap_or(0)
			});
		}
		Ok(self.tokens[self.index])
	}
	pub fn step(&mut self) {
		self.index += 1;
	}
	pub fn rollback(&mut self) {
		assert!(self.index > 0);
		self.index -= 1;
	}

	pub fn empty(&self) -> bool {
		self.index >= self.tokens.len()
	}
}

fn parse_kw<'a>(tokens: &mut Tokens<'a>, kw: &'static str) -> Result<(), Error<'a>> {
	let Token { ty, line } = tokens.current()?;

	if let TokenData::Ident(s) = ty {
		if s == kw {
			tokens.step();
			return Ok(());
		}
	}

	Err(Error::new(
		ErrorType::ExpectingKeyword(kw, ty),
		line
	))
}

fn parse_spec_kw<'a>(tokens: &mut Tokens<'a>) -> Result<(), Error<'a>> {
	parse_kw(tokens, "spec")
}

fn parse_lbracket<'a>(tokens: &mut Tokens<'a>) -> Result<(), Error<'a>> {
	let Token { ty, line } = tokens.current()?;

	if ty != TokenData::LBracket {
		return Err(Error::new(
			ErrorType::ExpectingRBracket(ty),
			line
		))
	}

	tokens.step();
	Ok(())
}
fn parse_comma<'a>(tokens: &mut Tokens<'a>) -> Result<(), Error<'a>> {
		let Token { ty, line } = tokens.current()?;

	if ty != TokenData::Comma {
		return Err(Error::new(
			ErrorType::ExpectingComma(ty),
			line
		))
	}

	tokens.step();
	Ok(())
}
fn parse_colon<'a>(tokens: &mut Tokens<'a>) -> Result<(), Error<'a>> {
		let Token { ty, line } = tokens.current()?;

	if ty != TokenData::Colon {
		return Err(Error::new(
			ErrorType::ExpectingColon(ty),
			line
		))
	}

	tokens.step();
	Ok(())
}
fn parse_equals<'a>(tokens: &mut Tokens<'a>) -> Result<(), Error<'a>> {
		let Token { ty, line } = tokens.current()?;

	if ty != TokenData::Equals {
		return Err(Error::new(
			ErrorType::ExpectingEquals(ty),
			line
		))
	}

	tokens.step();
	Ok(())
}

fn parse_field_type<'a>(tokens: &mut Tokens<'a>) -> Result<SpecFieldType<'a>, Error<'a>> {
	let name = parse_ident(tokens)?;

	if tokens.current()?.ty == TokenData::LBrace {
		tokens.step();

		let mut subtypes = vec![];

		while tokens.current()?.ty != TokenData::RBrace {
			subtypes.push(parse_field_type(tokens)?);

			// Comma not required for last element
			if tokens.current()?.ty == TokenData::RBrace {}
			else { parse_comma(tokens)?; }
		}

		tokens.step();

		Ok(SpecFieldType::Compound(name, subtypes))
	}
	else {
		Ok(SpecFieldType::Simple(name))
	}
}

fn parse_ident<'a>(tokens: &mut Tokens<'a>) -> Result<&'a str, Error<'a>> {
	let Token{ ty, line } = tokens.current()?;

	if let TokenData::Ident(s) = ty {
		tokens.step();
		Ok(s)
	}
	else {
		Err(Error::new(ErrorType::ExpectingIdent(ty), line))
	}
}
fn parse_num<'a>(tokens: &mut Tokens<'a>) -> Result<u32, Error<'a>> {
	let Token{ ty, line } = tokens.current()?;

	if let TokenData::Integer(n) = ty {
		tokens.step();
		Ok(n)
	}
	else {
		Err(Error::new(ErrorType::ExpectingIdent(ty), line))
	}
}

fn parse_docs<'a>(tokens: &mut Tokens<'a>) -> Result<Vec<&'a str>, Error<'a>> {
	let mut docs = vec![];
	
	while let Token{ ty: TokenData::DocComment(doc), .. } = tokens.current()? {
		docs.push(doc);
		tokens.step();
	}

	Ok(docs)
}

fn parse_field<'a>(tokens: &mut Tokens<'a>) -> Result<SpecField<'a>, Error<'a>> {
	let docs = parse_docs(tokens)?;
	let name = parse_ident(tokens)?;
	parse_colon(tokens)?;
	let ty = parse_field_type(tokens)?;

	if tokens.current()?.ty == TokenData::RBracket {}
	else { parse_comma(tokens)?; }

	Ok(SpecField { name, ty, docs })
}

fn parse_def<'a>(tokens: &mut Tokens<'a>) -> Result<SpecDefinition<'a>, Error<'a>> {
	let mut def = SpecDefinition::default();

	def.docs = parse_docs(tokens)?;
	def.name = parse_ident(tokens)?;
	parse_lbracket(tokens)?;

	while tokens.current()?.ty != TokenData::RBracket {
		def.fields.push(parse_field(tokens)?);
	}

	tokens.step();

	Ok(def)
} 

fn parse_spec<'a>(tokens: &mut Tokens<'a>, docs: Vec<&'a str>) -> Result<Spec<'a>, Error<'a>> {
	let mut defs = vec![];

	parse_spec_kw(tokens)?;
	let name = parse_ident(tokens)?;
	parse_lbracket(tokens)?;

	while tokens.current()?.ty != TokenData::RBracket {
		defs.push(parse_def(tokens)?);
	}

	tokens.step();
	Ok(Spec {
		defs,
		name,
		docs
	})
}

fn parse_enum_branch<'a>(tokens: &mut Tokens<'a>) -> Result<EnumBranch<'a>, Error<'a>> {
	let docs = parse_docs(tokens)?;
	let name = parse_ident(tokens)?;
	parse_equals(tokens)?;
	let value = parse_num(tokens)?;

	Ok(EnumBranch {
		docs,
		name,
		value
	})
}

fn parse_enum<'a>(tokens: &mut Tokens<'a>, docs: Vec<&'a str>) -> Result<Type<'a>, Error<'a>> {
	parse_kw(tokens, "enum")?;

	let name = parse_ident(tokens)?;
	parse_colon(tokens)?;
	let ty = parse_field_type(tokens)?;
	let mut branches = vec![];

	parse_lbracket(tokens)?;

	while tokens.current()?.ty != TokenData::RBracket {
		branches.push(parse_enum_branch(tokens)?);

		if tokens.current()?.ty != TokenData::RBracket {
			parse_comma(tokens)?;
		}
	}

	tokens.step();

	Ok(Type {
		name,
		docs,
		class: TypeClass::Enum(branches, ty)
	})
}

pub fn parse<'a>(input: &'a [u8]) -> Result<Protocol<'a>, Error<'a>> {
	let mut tokens = Tokens::new(input)?;
	let mut result = Protocol::default();

	while !tokens.empty() {
		let docs = parse_docs(&mut tokens)?;

		if let Ok(_) = parse_kw(&mut tokens, "spec") {
			tokens.rollback();
			result.specs.push(parse_spec(&mut tokens, docs)?);
		}
		else if let Ok(_) = parse_kw(&mut tokens, "enum") {
			tokens.rollback();
			result.types.push(parse_enum(&mut tokens, docs)?);
		}
		else {
			let Token{ line, ty } = tokens.current()?;

			return Err(Error::new(
				ErrorType::ExpectingKeyword("spec or enum", ty),
				line
			));
		}
	}


	Ok(result)
}
