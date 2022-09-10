use crate::error::{Error, ParsingError};
use crate::result::Result;
use crate::token::{Token, TokenKind};
use crate::typing::Type;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PrimValue {
    Empty,
    UInt(String),
    Int(String),
    Float(String),
    Char(String),
    String(String),
}

impl Default for PrimValue {
    fn default() -> Self {
        PrimValue::Empty
    }
}

impl PrimValue {
    pub fn new_empty() -> Self {
        PrimValue::Empty
    }

    pub fn new_char(c: char) -> Self {
        PrimValue::Char(c.to_string())
    }

    pub fn new_uint(s: &str) -> Self {
        PrimValue::UInt(s.to_string())
    }

    pub fn new_int(s: &str) -> Self {
        PrimValue::Int(s.to_string())
    }

    pub fn new_float(s: &str) -> Self {
        PrimValue::Float(s.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Value {
    pub tokens: Vec<Token>,
    pub name: Option<String>,
    pub typing: Option<Type>,
    pub content: Option<PrimValue>,
    pub children: Vec<Value>,
}

impl Value {
    pub fn new() -> Self {
        Value::default()
    }

    pub fn new_empty(tokens: Vec<Token>) -> Result<Self> {
        if tokens.len() != 1 || tokens[0].kind != TokenKind::EmptyLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: Some(tokens[0].chunks.as_ref().unwrap()[0].loc.clone()),
                desc: "expected an empty literal".into(),
            }));
        }

        let mut value = Value::default();

        value.tokens = tokens;
        value.typing = Some(Type::Empty);
        value.content = Some(PrimValue::Empty);

        Ok(value)
    }

    pub fn new_char(tokens: Vec<Token>) -> Result<Self> {
        if tokens.len() != 1 || tokens[0].kind != TokenKind::CharLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: Some(tokens[0].chunks.as_ref().unwrap()[0].loc.clone()),
                desc: "expected a char literal".into(),
            }));
        }

        let c = tokens[0].chunks.as_ref().unwrap()[0].content;

        let mut value = Value::default();

        value.tokens = tokens;
        value.typing = Some(Type::Char);
        value.content = Some(PrimValue::new_char(c));

        Ok(value)
    }

    pub fn new_uint(tokens: Vec<Token>) -> Result<Self> {
        if tokens.len() != 1 || tokens[0].kind != TokenKind::UIntLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: Some(tokens[0].chunks.as_ref().unwrap()[0].loc.clone()),
                desc: "expected a uint literal".into(),
            }));
        }

        let s: String = tokens[0].chunks.as_ref().unwrap().clone().into_iter().fold(
            "".into(),
            |mut acc, chunk| {
                acc.push(chunk.content);

                acc
            },
        );

        let mut value = Value::default();

        value.tokens = tokens;
        value.typing = Some(Type::UInt);
        value.content = Some(PrimValue::new_uint(&s));

        Ok(value)
    }

    pub fn new_int(tokens: Vec<Token>) -> Result<Self> {
        if tokens.len() != 1 || tokens[0].kind != TokenKind::IntLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: Some(tokens[0].chunks.as_ref().unwrap()[0].loc.clone()),
                desc: "expected an int literal".into(),
            }));
        }

        let s: String = tokens[0].chunks.as_ref().unwrap().clone().into_iter().fold(
            "".into(),
            |mut acc, chunk| {
                acc.push(chunk.content);

                acc
            },
        );

        let mut value = Value::default();

        value.tokens = tokens;
        value.typing = Some(Type::Int);
        value.content = Some(PrimValue::new_int(&s));

        Ok(value)
    }

    pub fn new_float(tokens: Vec<Token>) -> Result<Self> {
        if tokens.len() != 1 || tokens[0].kind != TokenKind::FloatLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: Some(tokens[0].chunks.as_ref().unwrap()[0].loc.clone()),
                desc: "expected a float literal".into(),
            }));
        }

        let s: String = tokens[0].chunks.as_ref().unwrap().clone().into_iter().fold(
            "".into(),
            |mut acc, chunk| {
                acc.push(chunk.content);

                acc
            },
        );

        let mut value = Value::default();

        value.tokens = tokens;
        value.typing = Some(Type::Float);
        value.content = Some(PrimValue::new_float(&s));

        Ok(value)
    }
}
