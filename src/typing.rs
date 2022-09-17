use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Unknown,
    Builtin,
    Empty,
    UInt,
    Int,
    Float,
    Char,
    String,
    Sum(Vec<Type>),
    Prod(Vec<Type>),
    Fun(Vec<Type>),
    App(Vec<Type>),
    Type,
}

impl Type {
    pub fn push_inner_type(mut self, loc: Loc, t: Type) -> Result<Self> {
        self = match self {
            Type::Sum(mut v) => {
                v.push(t);
                Type::Sum(v)
            }
            Type::Prod(mut v) => {
                v.push(t);
                Type::Prod(v)
            }
            Type::App(mut v) => {
                v.push(t);
                Type::App(v)
            }
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: Some(loc),
                    desc: "expected a type with inner types".into(),
                }));
            }
        };

        Ok(self)
    }

    pub fn is_complete(&self) -> bool {
        match self {
            Type::Unknown => false,
            Type::Sum(inner_types) => inner_types.iter().all(|t| t.is_complete()),
            Type::Prod(inner_types) => inner_types.iter().all(|t| t.is_complete()),
            Type::Fun(inner_types) => inner_types.iter().all(|t| t.is_complete()),
            Type::App(inner_types) => inner_types.iter().all(|t| t.is_complete()),
            _ => true,
        }
    }
}
