#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct EnumType(Vec<Type>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct PairType {
    pub first: Box<Type>,
    pub second: Box<Type>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct ListType(Vec<Type>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct ArrType(Vec<Type>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct VecType(Vec<Type>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct MapType(Vec<PairType>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct FunType {
    pub args: Vec<Type>,
    pub ret: Box<Type>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum Type {
    Builtin,
    Empty,
    Atomic,
    UInt,
    Int,
    Float,
    Size,
    Pointer,
    Ref,
    Char,
    String,
    Mem,
    Path,
    IO,
    Ctx,
    Enum(Box<EnumType>),
    Pair(Box<PairType>),
    List(Box<ListType>),
    Arr(Box<ArrType>),
    Vec(Box<VecType>),
    Map(Box<MapType>),
    Fun(Box<FunType>),
    Type,
}
