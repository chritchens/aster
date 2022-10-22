use crate::error::{Error, ParsingError};
use crate::result::Result;
use crate::syntax::SYMBOL_PATH_SEPARATOR;
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

pub fn path_prefix(s: &str) -> String {
    let mut v: Vec<&str> = s.split(SYMBOL_PATH_SEPARATOR).collect();
    let len = v.len();

    if len > 1 {
        v.remove(len - 1);
        v.join(".")
    } else {
        "".into()
    }
}

pub fn add_prefix(s: &str, prefix: &str) -> String {
    if prefix.is_empty() {
        s.into()
    } else {
        vec![prefix, s].join(&SYMBOL_PATH_SEPARATOR.to_string())
    }
}

pub fn path_suffix(s: &str) -> String {
    let mut v: Vec<&str> = s.split(SYMBOL_PATH_SEPARATOR).collect();
    let len = v.len();

    if len > 1 {
        v.remove(len - 1).into()
    } else {
        s.into()
    }
}

pub fn add_suffix(s: &str, suffix: &str) -> String {
    if suffix.is_empty() {
        s.into()
    } else {
        vec![s, suffix].join(&SYMBOL_PATH_SEPARATOR.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ValueScope {
    pub file: String,
    pub tpl_name: Option<String>,
    pub path: Vec<usize>,
}

impl ValueScope {
    pub fn new() -> ValueScope {
        ValueScope::default()
    }

    pub fn is_tpl(&self) -> bool {
        self.tpl_name.is_some() && self.path.len() == 1
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Value {
    pub token: Token,
    pub scope: ValueScope,
    pub qualification: Option<String>,
    pub name: Option<String>,
    pub typing: Option<Type>,
    pub prim: Option<PrimValue>,
    pub children: Vec<Value>,
}

impl Value {
    pub fn new() -> Self {
        Value::default()
    }

    pub fn file(&self) -> String {
        self.token.file()
    }

    pub fn name(&self) -> String {
        self.name.clone().unwrap_or_else(|| "".into())
    }

    pub fn qualified_name(&self) -> String {
        let mut qualified_name = Vec::new();

        if let Some(path) = self.qualification.clone() {
            qualified_name.push(path);
        }

        qualified_name.push(self.name.clone().unwrap());

        qualified_name.join(".")
    }

    pub fn is_tpl(&self) -> bool {
        self.scope.is_tpl()
    }

    pub fn new_empty(scope: &ValueScope, token: Token) -> Result<Self> {
        if token.kind != TokenKind::EmptyLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: token.loc(),
                desc: "expected an empty literal".into(),
            }));
        }

        let mut value = Value::default();

        value.token = token;
        value.scope = scope.clone();
        value.prim = Some(PrimValue::Empty);
        value.typing = Some(Type::Empty);

        Ok(value)
    }

    pub fn new_keyword(scope: &ValueScope, token: Token) -> Result<Self> {
        if token.kind != TokenKind::Keyword {
            return Err(Error::Parsing(ParsingError {
                loc: token.loc(),
                desc: "expected a keyword".into(),
            }));
        }

        let name = token.chunks[0].to_string();

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
        value.scope = scope.clone();
        value.name = Some(name);
        value.typing = Some(typing);

        Ok(value)
    }

    pub fn new_char(scope: &ValueScope, token: Token) -> Result<Self> {
        if token.kind != TokenKind::CharLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: token.loc(),
                desc: "expected a char literal".into(),
            }));
        }

        let mut content = token.chunks[0].content.clone();
        content.remove(0);
        content.remove(content.len() - 1);

        let mut value = Value::default();

        value.token = token;
        value.scope = scope.clone();
        value.prim = Some(PrimValue::new_char(&content));
        value.typing = Some(Type::Char);

        Ok(value)
    }

    pub fn new_uint(scope: &ValueScope, token: Token) -> Result<Self> {
        if token.kind != TokenKind::UIntLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: token.loc(),
                desc: "expected a uint literal".into(),
            }));
        }

        let content = token.chunks[0].content.clone();

        let mut value = Value::default();

        value.token = token;
        value.scope = scope.clone();
        value.prim = Some(PrimValue::new_uint(&content));
        value.typing = Some(Type::UInt);

        Ok(value)
    }

    pub fn new_int(scope: &ValueScope, token: Token) -> Result<Self> {
        if token.kind != TokenKind::IntLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: token.loc(),
                desc: "expected an int literal".into(),
            }));
        }

        let content = token.chunks[0].content.clone();

        let mut value = Value::default();

        value.token = token;
        value.scope = scope.clone();
        value.prim = Some(PrimValue::new_int(&content));
        value.typing = Some(Type::Int);

        Ok(value)
    }

    pub fn new_float(scope: &ValueScope, token: Token) -> Result<Self> {
        if token.kind != TokenKind::FloatLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: token.loc(),
                desc: "expected a float literal".into(),
            }));
        }

        let content = token.chunks[0].content.clone();

        let mut value = Value::default();

        value.token = token;
        value.scope = scope.clone();
        value.prim = Some(PrimValue::new_float(&content));
        value.typing = Some(Type::Float);

        Ok(value)
    }

    pub fn new_string(scope: &ValueScope, token: Token) -> Result<Self> {
        if token.kind != TokenKind::StringLiteral {
            return Err(Error::Parsing(ParsingError {
                loc: token.loc(),
                desc: "expected a string literal".into(),
            }));
        }

        let mut content = token.chunks[0].content.clone();
        content.remove(0);
        content.remove(content.len() - 1);

        let mut value = Value::default();

        value.token = token;
        value.scope = scope.clone();
        value.prim = Some(PrimValue::new_string(&content));
        value.typing = Some(Type::String);

        Ok(value)
    }

    pub fn new_symbol(scope: &ValueScope, token: Token) -> Result<Self> {
        if token.kind != TokenKind::ValueSymbol
            && token.kind != TokenKind::TypeSymbol
            && token.kind != TokenKind::PathSymbol
        {
            return Err(Error::Parsing(ParsingError {
                loc: token.loc(),
                desc: "expected a symbol".into(),
            }));
        }

        let mut name = token.chunks[0].to_string();
        if name.is_empty() {
            return Err(Error::Parsing(ParsingError {
                loc: token.loc(),
                desc: "expected a non-empty function name".into(),
            }));
        }

        let qualification = match token.kind {
            TokenKind::Keyword | TokenKind::ValueSymbol | TokenKind::TypeSymbol => None,
            TokenKind::PathSymbol => Some(path_prefix(&name)),
            _ => unreachable!(),
        };

        if qualification.is_some() {
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
        value.scope = scope.clone();
        value.qualification = qualification;
        value.name = Some(name);

        value.typing = Some(typing);

        Ok(value)
    }

    pub fn set_scope_path(&mut self, i: usize) {
        self.scope.path.push(i);

        for j in 0..self.children.len() {
            if self.children.len() > 1 {
                self.children[j].scope = self.scope.clone();
                self.children[j].set_scope_path(j);
            }
        }
    }

    pub fn new_app(scope: &mut ValueScope, tokens: Vec<Token>) -> Result<Self> {
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

        let mut name = head_token.chunks[0].to_string();
        if name.is_empty() {
            return Err(Error::Parsing(ParsingError {
                loc: head_token.loc(),
                desc: "expected a non-empty function name".into(),
            }));
        }

        let old_scope = scope.clone();

        scope.file = head_token.file();

        if scope.tpl_name.is_none() {
            scope.tpl_name = Some(name.clone());
        }

        let qualification = match head_token.kind {
            TokenKind::Keyword | TokenKind::ValueSymbol | TokenKind::TypeSymbol => None,
            TokenKind::PathSymbol => Some(path_prefix(&name)),
            _ => unreachable!(),
        };

        if qualification.is_some() {
            name = path_suffix(&name);
        }

        let mut value = Value::default();
        value.token = head_token;
        value.scope = scope.clone();
        value.qualification = qualification;
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
                            scope.path.push(form_count - 1);
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

                    let child_value = Value::new_app(scope, child_tokens)?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);
                }
                TokenKind::FormEnd => {
                    scope.path.remove(scope.path.len() - 1);
                    break;
                }
                TokenKind::EmptyLiteral => {
                    let child_value = Value::new_empty(scope, token.clone())?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::Keyword => {
                    let child_value = Value::new_keyword(scope, token.clone())?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::UIntLiteral => {
                    let child_value = Value::new_uint(scope, token.clone())?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::IntLiteral => {
                    let child_value = Value::new_int(scope, token.clone())?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::FloatLiteral => {
                    let child_value = Value::new_float(scope, token.clone())?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::CharLiteral => {
                    let child_value = Value::new_char(scope, token.clone())?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::StringLiteral => {
                    let child_value = Value::new_string(scope, token.clone())?;
                    value.children.push(child_value.clone());

                    let mut typing = value.typing.unwrap_or_else(|| Type::App(vec![]));
                    typing = typing.push_inner_type(loc, child_value.typing.unwrap())?;
                    value.typing = Some(typing);

                    idx += 1;
                }
                TokenKind::ValueSymbol | TokenKind::TypeSymbol | TokenKind::PathSymbol => {
                    let child_value = Value::new_symbol(scope, token.clone())?;
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

        scope.tpl_name = old_scope.tpl_name;
        scope.path = old_scope.path;

        Ok(value)
    }

    pub fn is_fully_typed(&self) -> bool {
        self.typing.is_none() || self.typing.as_ref().map(|t| t.is_complete()).unwrap()
    }
}
