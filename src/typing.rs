#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Empty,
    UInt,
    Int,
    Float,
    Char,
    String,
    Sum(Vec<Type>),
    Prod(Vec<Type>),
    Fun(Vec<Type>),
}
