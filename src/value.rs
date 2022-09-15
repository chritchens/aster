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

    pub fn new_uint(s: &str) -> Self {
        PrimValue::UInt(s.to_string())
    }

    pub fn new_int(s: &str) -> Self {
        PrimValue::Int(s.to_string())
    }

    pub fn new_float(s: &str) -> Self {
        PrimValue::Float(s.to_string())
    }

    pub fn new_char(s: &str) -> Self {
        PrimValue::Char(s.to_string())
    }

    pub fn new_string(s: &str) -> Self {
        PrimValue::String(s.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Value {
    pub token: Token,
    pub name: Option<String>,
    pub value: Option<PrimValue>,
    pub typing: Option<Type>,
    pub values: Vec<Value>,
}

impl Value {
    pub fn new() -> Self {
        Value::default()
    }

    pub fn new_empty(token: Token) -> Result<Self> {
        if token.kind != TokenKind::EmptyLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: Some(token.chunks.as_ref().unwrap()[0].loc.clone()),
                desc: "expected an empty literal".into(),
            }));
        }

        let mut value = Value::default();

        value.token = token;
        value.value = Some(PrimValue::Empty);
        value.typing = Some(Type::Empty);

        Ok(value)
    }

    pub fn new_char(token: Token) -> Result<Self> {
        if token.kind != TokenKind::CharLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: Some(token.chunks.as_ref().unwrap()[0].loc.clone()),
                desc: "expected a char literal".into(),
            }));
        }

        let mut content = token.chunks.as_ref().unwrap()[0].content.clone();
        content.remove(0);
        content.remove(content.len() - 1);

        let mut value = Value::default();

        value.token = token;
        value.value = Some(PrimValue::new_char(&content));
        value.typing = Some(Type::Char);

        Ok(value)
    }

    pub fn new_uint(token: Token) -> Result<Self> {
        if token.kind != TokenKind::UIntLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: Some(token.chunks.as_ref().unwrap()[0].loc.clone()),
                desc: "expected a uint literal".into(),
            }));
        }

        let content = token.chunks.as_ref().unwrap()[0].content.clone();

        let mut value = Value::default();

        value.token = token;
        value.value = Some(PrimValue::new_uint(&content));
        value.typing = Some(Type::UInt);

        Ok(value)
    }

    pub fn new_int(token: Token) -> Result<Self> {
        if token.kind != TokenKind::IntLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: Some(token.chunks.as_ref().unwrap()[0].loc.clone()),
                desc: "expected an int literal".into(),
            }));
        }

        let content = token.chunks.as_ref().unwrap()[0].content.clone();

        let mut value = Value::default();

        value.token = token;
        value.value = Some(PrimValue::new_int(&content));
        value.typing = Some(Type::Int);

        Ok(value)
    }

    pub fn new_float(token: Token) -> Result<Self> {
        if token.kind != TokenKind::FloatLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: Some(token.chunks.as_ref().unwrap()[0].loc.clone()),
                desc: "expected a float literal".into(),
            }));
        }

        let content = token.chunks.as_ref().unwrap()[0].content.clone();

        let mut value = Value::default();

        value.token = token;
        value.value = Some(PrimValue::new_float(&content));
        value.typing = Some(Type::Float);

        Ok(value)
    }

    pub fn new_string(token: Token) -> Result<Self> {
        if token.kind != TokenKind::StringLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: Some(token.chunks.as_ref().unwrap()[0].loc.clone()),
                desc: "expected a string literal".into(),
            }));
        }

        let mut content = token.chunks.as_ref().unwrap()[0].content.clone();
        content.remove(0);
        content.remove(content.len() - 1);

        let mut value = Value::default();

        value.token = token;
        value.value = Some(PrimValue::new_string(&content));
        value.typing = Some(Type::String);

        Ok(value)
    }

    pub fn new_symbol(token: Token) -> Result<Self> {
        if token.kind != TokenKind::ValueSymbol && token.kind != TokenKind::TypeSymbol {
            return Err(Error::Parsing(ParsingError {
                loc: Some(token.chunks.as_ref().unwrap()[0].loc.clone()),
                desc: "expected a symbol".into(),
            }));
        }

        let name = token.chunks.as_ref().unwrap()[0].to_string();
        if name.is_empty() {
            return Err(Error::Parsing(ParsingError {
                loc: Some(token.chunks.as_ref().unwrap()[0].loc.clone()),
                desc: "expected a non-empty function name".into(),
            }));
        }

        let typing = match token.kind {
            TokenKind::ValueSymbol => Type::Unknown,
            TokenKind::TypeSymbol => Type::Type,
            _ => unreachable!(),
        };

        let mut value = Value::default();
        value.token = token;
        value.name = Some(name);

        value.typing = Some(typing);

        Ok(value)
    }

    pub fn new_fun(tokens: Vec<Token>) -> Result<Self> {
        if tokens.len() < 3
            || tokens[0].kind != TokenKind::FormStart
            || tokens.last().unwrap().kind != TokenKind::FormEnd
        {
            return Err(Error::Parsing(ParsingError {
                loc: Some(tokens[0].chunks.as_ref().unwrap()[0].loc.clone()),
                desc: "expected a form".into(),
            }));
        }

        if tokens[1].kind != TokenKind::ValueSymbol {
            return Err(Error::Parsing(ParsingError {
                loc: Some(tokens[0].chunks.as_ref().unwrap()[0].loc.clone()),
                desc: "expected a function name".into(),
            }));
        }

        let head_token = tokens[1].clone();

        let name = head_token.chunks.as_ref().unwrap()[0].to_string();
        if name.is_empty() {
            return Err(Error::Parsing(ParsingError {
                loc: Some(head_token.chunks.as_ref().unwrap()[0].loc.clone()),
                desc: "expected a non-empty function name".into(),
            }));
        }

        let mut value = Value::default();
        value.token = head_token;
        value.name = Some(name);

        let len = tokens.len() - 1;
        let mut idx = 1;

        while idx < len {
            let token = tokens[idx].clone();
            let loc = token.chunks.as_ref().unwrap()[0].loc.clone();

            match token.kind {
                TokenKind::FormStart => {
                    let mut form_count = 0;
                    let mut child_tokens = vec![];

                    for child_token in tokens[idx..].iter() {
                        if child_token.kind == TokenKind::FormStart {
                            form_count += 1;
                            child_tokens.push(child_token.clone());
                            idx += 1;
                        } else if child_token.kind == TokenKind::FormEnd {
                            form_count -= 1;
                            child_tokens.push(child_token.clone());
                            idx += 1;

                            if form_count == 0 {
                                break;
                            }
                        } else {
                            child_tokens.push(child_token.clone());
                            idx += 1;
                        }
                    }

                    if form_count != 0 {
                        return Err(Error::Parsing(ParsingError {
                            loc: Some(token.chunks.as_ref().unwrap()[0].loc.clone()),
                            desc: "expected a well-formed form".into(),
                        }));
                    }

                    let child_value = Value::new_fun(child_tokens)?;
                    value.values.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::Fun(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);
                }
                TokenKind::FormEnd => break,
                TokenKind::EmptyLiteral => {
                    let child_value = Value::new_empty(token.clone())?;
                    value.values.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::Fun(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::UIntLiteral => {
                    let child_value = Value::new_uint(token.clone())?;
                    value.values.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::Fun(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::IntLiteral => {
                    let child_value = Value::new_int(token.clone())?;
                    value.values.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::Fun(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::FloatLiteral => {
                    let child_value = Value::new_float(token.clone())?;
                    value.values.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::Fun(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::CharLiteral => {
                    let child_value = Value::new_char(token.clone())?;
                    value.values.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::Fun(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::StringLiteral => {
                    let child_value = Value::new_string(token.clone())?;
                    value.values.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::Fun(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::ValueSymbol | TokenKind::TypeSymbol => {
                    let child_value = Value::new_symbol(token.clone())?;
                    value.values.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::Fun(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                _ => {
                    return Err(Error::Parsing(ParsingError {
                        loc: Some(token.chunks.as_ref().unwrap()[0].loc.clone()),
                        desc: "expected a well-formed form".into(),
                    }));
                }
            }
        }

        Ok(value)
    }
}
