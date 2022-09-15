use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Unknown,
    Empty,
    UInt,
    Int,
    Float,
    Char,
    String,
    Sum(Vec<Type>),
    Prod(Vec<Type>),
    Fun(Vec<Type>),
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
            Type::Fun(mut v) => {
                v.push(t);
                Type::Fun(v)
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
}
