use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_keyword, is_type_symbol};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum Type {
    Unknown(Option<String>),
    Builtin,
    Empty,
    UInt,
    Int,
    Float,
    Char,
    String,
    Path,
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
            Type::Unknown(_) => false,
            Type::Sum(inner_types) => inner_types.iter().all(|t| t.is_complete()),
            Type::Prod(inner_types) => inner_types.iter().all(|t| t.is_complete()),
            Type::Fun(inner_types) => inner_types.iter().all(|t| t.is_complete()),
            Type::App(inner_types) => inner_types.iter().all(|t| t.is_complete()),
            _ => true,
        }
    }

    pub fn from_str(s: &str, loc: Loc) -> Result<Self> {
        let t = match s {
            "Empty" => Type::Empty,
            "UInt" => Type::UInt,
            "Int" => Type::Int,
            "Float" => Type::Float,
            "Char" => Type::Char,
            "String" => Type::String,
            "Path" => Type::Path,
            "Type" => Type::Type,
            st if is_type_symbol(st) => {
                if is_keyword(st) {
                    Type::Builtin
                } else {
                    Type::Unknown(Some(st.into()))
                }
            }
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: Some(loc),
                    desc: "expected a type".into(),
                }));
            }
        };

        Ok(t)
    }

    pub fn from_string(s: String, loc: Loc) -> Result<Self> {
        Type::from_str(&s, loc)
    }

    pub fn to_string(&self) -> String {
        match self {
            Type::Unknown(opt_t) => match opt_t {
                Some(s) => format!("Unknown({})", s),
                None => "Unknown".into(),
            },
            Type::Builtin => "Builtin".into(),
            Type::Empty => "Empty".into(),
            Type::UInt => "UInt".into(),
            Type::Int => "Int".into(),
            Type::Float => "Float".into(),
            Type::Char => "Char".into(),
            Type::String => "String".into(),
            Type::Path => "Path".into(),
            Type::Sum(types) => format!(
                "(Sum {})",
                types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Type::Prod(types) => format!(
                "(Prod {})",
                types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Type::Fun(types) => format!(
                "(Fun {})",
                types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Type::App(types) => format!(
                "(App {})",
                types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Type::Type => "Type".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn type_from_str() {
        use super::Type;
        use crate::loc::Loc;

        let mut s = "Char";

        let res = Type::from_str(s, Loc::default());

        assert!(res.is_ok());

        let mut t = res.unwrap();

        assert_eq!(t, Type::Char);

        s = "Square";

        t = Type::from_str(s, Loc::default()).unwrap();

        assert_eq!(t, Type::Unknown(Some(s.into())));
    }

    #[test]
    fn type_to_string() {
        use super::Type;

        let s: String = "(Sum Int Char (Prod String Unknown(Circle)))".into();
        let t = Type::Sum(vec![
            Type::Int,
            Type::Char,
            Type::Prod(vec![Type::String, Type::Unknown(Some("Circle".into()))]),
        ]);

        let st = t.to_string();

        assert_eq!(s, st);
    }
}
