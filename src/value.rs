use crate::error::{Error, ParsingError};
use crate::result::Result;
use crate::syntax::{path_prefix, path_suffix};
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
    pub path: Option<String>,
    pub name: Option<String>,
    pub value: Option<PrimValue>,
    pub children: Vec<Value>,
    pub typing: Option<Type>,
}

impl Value {
    pub fn new() -> Self {
        Value::default()
    }

    pub fn new_empty(token: Token) -> Result<Self> {
        if token.kind != TokenKind::EmptyLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: token.loc(),
                desc: "expected an empty literal".into(),
            }));
        }

        let mut value = Value::default();

        value.token = token;
        value.value = Some(PrimValue::Empty);
        value.typing = Some(Type::Empty);

        Ok(value)
    }

    pub fn new_keyword(token: Token) -> Result<Self> {
        if token.kind != TokenKind::Keyword {
            return Err(Error::Parsing(ParsingError {
                loc: token.loc(),
                desc: "expected a keyword".into(),
            }));
        }

        let name = token.chunks.as_ref().unwrap()[0].to_string();

        let mut value = Value::default();

        let typing = if name == "Empty"
            || name == "UInt"
            || name == "Int"
            || name == "Float"
            || name == "Char"
            || name == "String"
            || name == "Path"
            || name == "IO"
        {
            Type::Type
        } else {
            Type::Builtin
        };

        value.token = token;
        value.name = Some(name);
        value.typing = Some(typing);

        Ok(value)
    }

    pub fn new_char(token: Token) -> Result<Self> {
        if token.kind != TokenKind::CharLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: token.loc(),
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
                loc: token.loc(),
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
                loc: token.loc(),
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
                loc: token.loc(),
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
                loc: token.loc(),
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
        if token.kind != TokenKind::ValueSymbol
            && token.kind != TokenKind::TypeSymbol
            && token.kind != TokenKind::PathSymbol
        {
            return Err(Error::Parsing(ParsingError {
                loc: token.loc(),
                desc: "expected a symbol".into(),
            }));
        }

        let mut name = token.chunks.as_ref().unwrap()[0].to_string();
        if name.is_empty() {
            return Err(Error::Parsing(ParsingError {
                loc: token.loc(),
                desc: "expected a non-empty function name".into(),
            }));
        }

        let path = match token.kind {
            TokenKind::Keyword | TokenKind::ValueSymbol | TokenKind::TypeSymbol => None,
            TokenKind::PathSymbol => Some(path_prefix(&name)),
            _ => unreachable!(),
        };

        if path.is_some() {
            name = path_suffix(&name);
        }

        let typing = match token.kind {
            TokenKind::ValueSymbol => Type::Unknown(None),
            TokenKind::TypeSymbol => Type::Unknown(Some(name.clone())), // could be a type constructor
            TokenKind::PathSymbol => Type::Path,
            _ => unreachable!(),
        };

        let mut value = Value::default();
        value.token = token;
        value.path = path;
        value.name = Some(name);

        value.typing = Some(typing);

        Ok(value)
    }

    pub fn new_app(tokens: Vec<Token>) -> Result<Self> {
        if tokens.len() < 3
            || tokens[0].kind != TokenKind::FormStart
            || tokens.last().unwrap().kind != TokenKind::FormEnd
        {
            return Err(Error::Parsing(ParsingError {
                loc: tokens[0].loc(),
                desc: "expected a form".into(),
            }));
        }

        let head_token = tokens[1].clone();

        if head_token.kind != TokenKind::ValueSymbol
            && head_token.kind != TokenKind::TypeSymbol
            && head_token.kind != TokenKind::PathSymbol
            && head_token.kind != TokenKind::Keyword
        {
            return Err(Error::Parsing(ParsingError {
                loc: head_token.loc(),
                desc: "expected the function name to be a symbol or keyword".into(),
            }));
        }

        let mut name = head_token.chunks.as_ref().unwrap()[0].to_string();
        if name.is_empty() {
            return Err(Error::Parsing(ParsingError {
                loc: head_token.loc(),
                desc: "expected a non-empty function name".into(),
            }));
        }

        let path = match head_token.kind {
            TokenKind::Keyword | TokenKind::ValueSymbol | TokenKind::TypeSymbol => None,
            TokenKind::PathSymbol => Some(path_prefix(&name)),
            _ => unreachable!(),
        };

        if path.is_some() {
            name = path_suffix(&name);
        }

        let mut value = Value::default();
        value.token = head_token;
        value.path = path;
        value.name = Some(name);

        let len = tokens.len() - 1;
        let mut idx = 1;

        while idx < len {
            let token = tokens[idx].clone();
            let loc = token.loc().unwrap();

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
                            loc: Some(loc),
                            desc: "expected a well-formed form".into(),
                        }));
                    }

                    let child_value = Value::new_app(child_tokens)?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);
                }
                TokenKind::FormEnd => break,
                TokenKind::EmptyLiteral => {
                    let child_value = Value::new_empty(token.clone())?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::Keyword => {
                    let child_value = Value::new_keyword(token.clone())?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::UIntLiteral => {
                    let child_value = Value::new_uint(token.clone())?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::IntLiteral => {
                    let child_value = Value::new_int(token.clone())?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::FloatLiteral => {
                    let child_value = Value::new_float(token.clone())?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::CharLiteral => {
                    let child_value = Value::new_char(token.clone())?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::StringLiteral => {
                    let child_value = Value::new_string(token.clone())?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::ValueSymbol | TokenKind::TypeSymbol | TokenKind::PathSymbol => {
                    let child_value = Value::new_symbol(token.clone())?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                _ => {
                    return Err(Error::Parsing(ParsingError {
                        loc: Some(loc),
                        desc: "expected a well-formed form".into(),
                    }));
                }
            }
        }

        if value.token.kind == TokenKind::Keyword && value.name.as_ref().unwrap() == "deftype" {
            if let Some(Type::App(v)) = value.typing.as_ref() {
                if let Type::App(v2) = v[2].clone() {
                    let mut v3 = v2;
                    v3.remove(0);
                    v3.push(Type::Type);

                    let constructor_type = Type::Fun(v3);

                    let mut typing_v = v.clone();
                    typing_v[1] = constructor_type;

                    value.typing = Some(Type::App(typing_v));
                }
            }
        }

        Ok(value)
    }

    pub fn is_fully_typed(&self) -> bool {
        self.typing.is_none() || self.typing.as_ref().map(|t| t.is_complete()).unwrap()
    }
}
