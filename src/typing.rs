use std::fmt;
use std::iter;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Builtin,
    Empty,
    Prim,
    UInt,
    Int,
    Float,
    Size,
    Char,
    String,
    Mem,
    Path,
    IO,
    Ctx,
    Sum(Vec<Type>),
    Prod(Vec<Type>),
    Sig(Vec<Type>),
    Fun(Vec<Type>),
    App(Vec<Type>),
    Attrs(Vec<Type>),
    Type,
    Unknown(String),
}

impl Default for Type {
    fn default() -> Self {
        Type::Builtin
    }
}

impl Type {
    pub fn new() -> Type {
        Type::default()
    }

    pub fn len(&self) -> usize {
        match self {
            Type::Sum(types) => types.len(),
            Type::Prod(types) => types.len(),
            Type::Sig(types) => types.len(),
            Type::Fun(types) => types.len(),
            Type::App(types) => types.len(),
            Type::Attrs(types) => types.len(),
            _ => 1,
        }
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn push(&mut self, typing: &Type) {
        match self {
            Type::Sum(types) => types.push(typing.clone()),
            Type::Prod(types) => types.push(typing.clone()),
            Type::Sig(types) => types.push(typing.clone()),
            Type::Fun(types) => types.push(typing.clone()),
            Type::App(types) => types.push(typing.clone()),
            Type::Attrs(types) => types.push(typing.clone()),
            _ => {}
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            Type::Builtin => "Builtin".into(),
            Type::Empty => "Empty".into(),
            Type::Prim => "Prim".into(),
            Type::UInt => "UInt".into(),
            Type::Int => "Int".into(),
            Type::Float => "Float".into(),
            Type::Size => "Size".into(),
            Type::Char => "Char".into(),
            Type::String => "String".into(),
            Type::Mem => "Mem".into(),
            Type::Path => "Path".into(),
            Type::IO => "IO".into(),
            Type::Ctx => "Ctx".into(),
            Type::Sum(types) => format!(
                "(Sum {})",
                types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Type::Prod(types) => format!(
                "(Prod {})",
                types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Type::Sig(types) => format!(
                "(Sig {})",
                types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Type::Fun(types) => format!(
                "(Fun {})",
                types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Type::App(types) => format!(
                "(App {})",
                types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Type::Attrs(types) => format!(
                "(Attrs {})",
                types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Type::Type => "Type".into(),
            Type::Unknown(typing) => format!("Unknown({})", typing),
        }
    }

    pub fn is_complete(&self) -> bool {
        match self {
            Type::Sum(types) => types.iter().all(|t| t.is_complete()),
            Type::Prod(types) => types.iter().all(|t| t.is_complete()),
            Type::Sig(types) => types.iter().all(|t| t.is_complete()),
            Type::Fun(types) => types.iter().all(|t| t.is_complete()),
            Type::App(types) => types.iter().all(|t| t.is_complete()),
            Type::Attrs(types) => types.iter().all(|t| t.is_complete()),
            Type::Unknown(_) => false,
            _ => true,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl iter::IntoIterator for Type {
    type Item = Type;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Type::Sum(types) => types.into_iter(),
            Type::Prod(types) => types.into_iter(),
            Type::Sig(types) => types.into_iter(),
            Type::Fun(types) => types.into_iter(),
            Type::App(types) => types.into_iter(),
            Type::Attrs(types) => types.into_iter(),
            x => vec![x].into_iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn type_is_complete() {
        use super::Type;

        assert!(Type::Ctx.is_complete());
        assert!(!Type::Unknown("unknown".into()).is_complete());
        assert!(!Type::App(vec![Type::Type, Type::Unknown("AType".into())]).is_complete());
        assert!(Type::Attrs(vec![Type::Char, Type::Float]).is_complete());
    }
}
