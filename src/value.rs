use crate::token::Token;
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

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Value {
    tokens: Vec<Token>,
    name: Option<String>,
    typing: Option<Type>,
    content: Option<PrimValue>,
    children: Vec<Value>,
}

impl Value {
    pub fn new() -> Self {
        Value::default()
    }
}
