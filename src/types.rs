use crate::chunk::StringChunks;
use crate::error::{Error, SyntacticError};
use crate::form::Form;
use crate::form::{TypesForm, TypesFormTailElement};
use crate::result::Result;
use crate::token::{Token, TokenKind, Tokens};
use crate::value::SimpleValue;
use std::fmt;
use std::iter;
use std::ops;

fn parse_types_form_tail_element(elem: &TypesFormTailElement) -> Result<Type> {
    let elem_type = match elem.clone() {
        TypesFormTailElement::Ignore(value) => {
            let simple_type = SimpleType::from_simple_value(&value)?;
            Type::Simple(simple_type)
        }
        TypesFormTailElement::Empty(value) => {
            let simple_type = SimpleType::from_simple_value(&value)?;
            Type::Simple(simple_type)
        }
        TypesFormTailElement::Atomic(value) => {
            let simple_type = SimpleType::from_simple_value(&value)?;
            Type::Simple(simple_type)
        }
        TypesFormTailElement::Keyword(value) => {
            let simple_type = SimpleType::from_simple_value(&value)?;
            Type::Simple(simple_type)
        }
        TypesFormTailElement::Symbol(value) => {
            let simple_type = SimpleType::from_simple_value(&value)?;
            Type::Simple(simple_type)
        }
        TypesFormTailElement::PathSymbol(value) => {
            let simple_type = SimpleType::from_simple_value(&value)?;
            Type::Simple(simple_type)
        }
        TypesFormTailElement::Form(form) => Type::from_types_form(&form)?,
    };

    Ok(elem_type)
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum SimpleType {
    Builtin(SimpleValue),
    Ignore(SimpleValue),
    Empty(SimpleValue),
    Atomic(SimpleValue),
    UInt(SimpleValue),
    Int(SimpleValue),
    Float(SimpleValue),
    Size(SimpleValue),
    Pointer(SimpleValue),
    Ref(SimpleValue),
    Char(SimpleValue),
    String(SimpleValue),
    Mem(SimpleValue),
    Path(SimpleValue),
    IO(SimpleValue),
    Ctx(SimpleValue),
    Type(SimpleValue),
    Symbol(SimpleValue),
    PathSymbol(SimpleValue),
}

impl Default for SimpleType {
    fn default() -> SimpleType {
        let chunks = StringChunks::from_str("Empty");
        let mut token = Token::new();
        token.kind = TokenKind::Keyword;
        token.chunks = chunks;

        SimpleType::Empty(SimpleValue::TypeKeyword(token))
    }
}

impl SimpleType {
    pub fn new() -> SimpleType {
        SimpleType::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<SimpleType> {
        let value = SimpleValue::from_str(s)?;

        SimpleType::from_simple_value(&value)
    }

    pub fn from_simple_value(value: &SimpleValue) -> Result<SimpleType> {
        let simple_type = match value {
            SimpleValue::Ignore(_) => SimpleType::Ignore(value.to_owned()),
            SimpleValue::TypeKeyword(_) => match value.to_string().as_str() {
                "Builtin" => SimpleType::Builtin(value.to_owned()),
                "Empty" => SimpleType::Empty(value.to_owned()),
                "Atomic" => SimpleType::Atomic(value.to_owned()),
                "UInt" => SimpleType::UInt(value.to_owned()),
                "Int" => SimpleType::Int(value.to_owned()),
                "Float" => SimpleType::Float(value.to_owned()),
                "Size" => SimpleType::Size(value.to_owned()),
                "Pointer" => SimpleType::Pointer(value.to_owned()),
                "Ref" => SimpleType::Ref(value.to_owned()),
                "Char" => SimpleType::Char(value.to_owned()),
                "String" => SimpleType::String(value.to_owned()),
                "Mem" => SimpleType::Mem(value.to_owned()),
                "Path" => SimpleType::Path(value.to_owned()),
                "IO" => SimpleType::IO(value.to_owned()),
                "Ctx" => SimpleType::Ctx(value.to_owned()),
                "Type" => SimpleType::Type(value.to_owned()),
                _ => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: value.loc(),
                        desc: "unexpected value".into(),
                    }));
                }
            },
            SimpleValue::TypeSymbol(_) => SimpleType::Symbol(value.to_owned()),
            SimpleValue::TypePathSymbol(_) => SimpleType::PathSymbol(value.to_owned()),
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: value.loc(),
                    desc: "expected a simple type".into(),
                }));
            }
        };

        Ok(simple_type)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            SimpleType::Builtin(_) => "Builtin".into(),
            SimpleType::Ignore(_) => "Ignore".into(),
            SimpleType::Empty(_) => "Empty".into(),
            SimpleType::Atomic(_) => "Atomic".into(),
            SimpleType::UInt(_) => "UInt".into(),
            SimpleType::Int(_) => "Int".into(),
            SimpleType::Float(_) => "Float".into(),
            SimpleType::Size(_) => "Size".into(),
            SimpleType::Pointer(_) => "Pointer".into(),
            SimpleType::Ref(_) => "Ref".into(),
            SimpleType::Char(_) => "Char".into(),
            SimpleType::String(_) => "String".into(),
            SimpleType::Mem(_) => "Mem".into(),
            SimpleType::Path(_) => "Path".into(),
            SimpleType::IO(_) => "IO".into(),
            SimpleType::Ctx(_) => "Ctx".into(),
            SimpleType::Type(_) => "Type".into(),
            SimpleType::Symbol(symbol_type) => symbol_type.to_string(),
            SimpleType::PathSymbol(symbol_type) => symbol_type.to_string(),
        }
    }
}

impl std::str::FromStr for SimpleType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl fmt::Display for SimpleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct EnumType {
    pub tokens: Box<Tokens>,
    pub elements: Vec<Type>,
}

impl EnumType {
    pub fn new() -> EnumType {
        EnumType::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<EnumType> {
        let form = Form::from_str(s)?;

        EnumType::from_form(&form)
    }

    pub fn from_form(form: &Form) -> Result<EnumType> {
        let form = TypesForm::from_form(form)?;

        EnumType::from_types_form(&form)
    }

    pub fn from_types_form(form: &TypesForm) -> Result<EnumType> {
        if form.head.to_string() != "Enum" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected an Enum keyword".into(),
            }));
        }

        let mut enum_type = EnumType::new();
        enum_type.tokens = form.tokens.clone();

        for elem in form.tail.iter() {
            let elem_type = parse_types_form_tail_element(elem)?;
            enum_type.elements.push(elem_type);
        }

        Ok(enum_type)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(Enum {})",
            self.elements
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl std::str::FromStr for EnumType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl fmt::Display for EnumType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ops::Index<usize> for EnumType {
    type Output = Type;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.elements[idx]
    }
}

impl ops::IndexMut<usize> for EnumType {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.elements[idx]
    }
}

impl iter::IntoIterator for EnumType {
    type Item = Type;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct PairType {
    pub tokens: Box<Tokens>,
    pub first: Box<Type>,
    pub second: Box<Type>,
}

impl PairType {
    pub fn new() -> PairType {
        PairType::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<PairType> {
        let form = Form::from_str(s)?;

        PairType::from_form(&form)
    }

    pub fn from_form(form: &Form) -> Result<PairType> {
        let form = TypesForm::from_form(form)?;

        PairType::from_types_form(&form)
    }

    pub fn from_types_form(form: &TypesForm) -> Result<PairType> {
        if form.head.to_string() != "Pair" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected an Pair keyword".into(),
            }));
        }

        if form.tail.len() != 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected two types".into(),
            }));
        }

        let mut pair_type = PairType::new();
        pair_type.tokens = form.tokens.clone();

        let first = form.tail[0].clone();
        let second = form.tail[1].clone();

        pair_type.first = Box::new(parse_types_form_tail_element(&first)?);
        pair_type.second = Box::new(parse_types_form_tail_element(&second)?);

        Ok(pair_type)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(Pair {} {})",
            self.first.to_string(),
            self.second.to_string()
        )
    }
}

impl std::str::FromStr for PairType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl fmt::Display for PairType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct ListType {
    pub tokens: Box<Tokens>,
    pub elements: Vec<Type>,
}

impl ListType {
    pub fn new() -> ListType {
        ListType::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<ListType> {
        let form = Form::from_str(s)?;

        ListType::from_form(&form)
    }

    pub fn from_form(form: &Form) -> Result<ListType> {
        let form = TypesForm::from_form(form)?;

        ListType::from_types_form(&form)
    }

    pub fn from_types_form(form: &TypesForm) -> Result<ListType> {
        if form.head.to_string() != "List" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected an List keyword".into(),
            }));
        }

        let mut list_type = ListType::new();
        list_type.tokens = form.tokens.clone();

        for elem in form.tail.iter() {
            let elem_type = parse_types_form_tail_element(elem)?;
            list_type.elements.push(elem_type);
        }

        Ok(list_type)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(List {})",
            self.elements
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl std::str::FromStr for ListType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl fmt::Display for ListType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ops::Index<usize> for ListType {
    type Output = Type;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.elements[idx]
    }
}

impl ops::IndexMut<usize> for ListType {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.elements[idx]
    }
}

impl iter::IntoIterator for ListType {
    type Item = Type;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct ArrType {
    pub tokens: Box<Tokens>,
    pub elements: Vec<Type>,
}

impl ArrType {
    pub fn new() -> ArrType {
        ArrType::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<ArrType> {
        let form = Form::from_str(s)?;

        ArrType::from_form(&form)
    }

    pub fn from_form(form: &Form) -> Result<ArrType> {
        let form = TypesForm::from_form(form)?;

        ArrType::from_types_form(&form)
    }

    pub fn from_types_form(form: &TypesForm) -> Result<ArrType> {
        if form.head.to_string() != "Arr" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected an Arr keyword".into(),
            }));
        }

        let mut arr_type = ArrType::new();
        arr_type.tokens = form.tokens.clone();

        for elem in form.tail.iter() {
            let elem_type = parse_types_form_tail_element(elem)?;
            arr_type.elements.push(elem_type);
        }

        Ok(arr_type)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(Arr {})",
            self.elements
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl std::str::FromStr for ArrType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl fmt::Display for ArrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ops::Index<usize> for ArrType {
    type Output = Type;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.elements[idx]
    }
}

impl ops::IndexMut<usize> for ArrType {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.elements[idx]
    }
}

impl iter::IntoIterator for ArrType {
    type Item = Type;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct VecType {
    pub tokens: Box<Tokens>,
    pub elements: Vec<Type>,
}

impl VecType {
    pub fn new() -> VecType {
        VecType::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<VecType> {
        let form = Form::from_str(s)?;

        VecType::from_form(&form)
    }

    pub fn from_form(form: &Form) -> Result<VecType> {
        let form = TypesForm::from_form(form)?;

        VecType::from_types_form(&form)
    }

    pub fn from_types_form(form: &TypesForm) -> Result<VecType> {
        if form.head.to_string() != "Vec" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected an Vec keyword".into(),
            }));
        }

        let mut vec_type = VecType::new();
        vec_type.tokens = form.tokens.clone();

        for elem in form.tail.iter() {
            let elem_type = parse_types_form_tail_element(elem)?;
            vec_type.elements.push(elem_type);
        }

        Ok(vec_type)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(Vec {})",
            self.elements
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl std::str::FromStr for VecType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl fmt::Display for VecType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ops::Index<usize> for VecType {
    type Output = Type;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.elements[idx]
    }
}

impl ops::IndexMut<usize> for VecType {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.elements[idx]
    }
}

impl iter::IntoIterator for VecType {
    type Item = Type;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct MapType {
    pub tokens: Box<Tokens>,
    pub entries: Vec<PairType>,
}

impl MapType {
    pub fn new() -> MapType {
        MapType::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<MapType> {
        let form = Form::from_str(s)?;

        MapType::from_form(&form)
    }

    pub fn from_form(form: &Form) -> Result<MapType> {
        let form = TypesForm::from_form(form)?;

        MapType::from_types_form(&form)
    }

    pub fn from_types_form(form: &TypesForm) -> Result<MapType> {
        if form.head.to_string() != "Map" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected an Map keyword".into(),
            }));
        }

        let mut map_type = MapType::new();
        map_type.tokens = form.tokens.clone();

        for entry in form.tail.iter() {
            let entry_pair = PairType::from_str(&entry.to_string())?;
            map_type.entries.push(entry_pair);
        }

        Ok(map_type)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(Map {})",
            self.entries
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl std::str::FromStr for MapType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl fmt::Display for MapType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ops::Index<usize> for MapType {
    type Output = PairType;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.entries[idx]
    }
}

impl ops::IndexMut<usize> for MapType {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.entries[idx]
    }
}

impl iter::IntoIterator for MapType {
    type Item = PairType;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct FunType {
    pub tokens: Box<Tokens>,
    pub parameters: Vec<Type>,
    pub body: Box<Type>,
}

impl FunType {
    pub fn new() -> FunType {
        FunType::default()
    }

    pub fn parameters_to_string(&self) -> String {
        self.parameters
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<FunType> {
        let form = Form::from_str(s)?;

        FunType::from_form(&form)
    }

    pub fn from_form(form: &Form) -> Result<FunType> {
        let form = TypesForm::from_form(form)?;

        FunType::from_types_form(&form)
    }

    pub fn from_types_form(form: &TypesForm) -> Result<FunType> {
        if form.head.to_string() != "Fun" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected an Fun keyword".into(),
            }));
        }

        if form.tail.len() < 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected at least one parameter and a body".into(),
            }));
        }

        let mut fun_type = FunType::new();
        fun_type.tokens = form.tokens.clone();

        let len = form.tail.len();

        for param in form.tail[0..(len - 1)].iter() {
            let param_type = parse_types_form_tail_element(param)?;
            fun_type.parameters.push(param_type);
        }

        let body_type = parse_types_form_tail_element(&form.tail[len - 1])?;
        fun_type.body = Box::new(body_type);

        Ok(fun_type)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(Fun {} {})",
            self.parameters_to_string(),
            self.body.to_string()
        )
    }
}

impl std::str::FromStr for FunType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl fmt::Display for FunType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum Type {
    Simple(SimpleType),
    Enum(Box<EnumType>),
    Pair(Box<PairType>),
    List(Box<ListType>),
    Arr(Box<ArrType>),
    Vec(Box<VecType>),
    Map(Box<MapType>),
    Fun(Box<FunType>),
}

impl Default for Type {
    fn default() -> Type {
        Type::Simple(SimpleType::new())
    }
}

impl Type {
    pub fn new() -> Type {
        Type::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Type> {
        let form = Form::from_str(s)?;

        Type::from_form(&form)
    }

    pub fn from_form(form: &Form) -> Result<Type> {
        let form = TypesForm::from_form(form)?;

        Type::from_types_form(&form)
    }

    pub fn from_simple_value(value: &SimpleValue) -> Result<Type> {
        let simple_type = SimpleType::from_simple_value(value)?;

        Ok(Type::Simple(simple_type))
    }

    pub fn from_types_form(form: &TypesForm) -> Result<Type> {
        let t = if let Ok(enum_type) = EnumType::from_types_form(form) {
            Type::Enum(Box::new(enum_type))
        } else if let Ok(pair_type) = PairType::from_types_form(form) {
            Type::Pair(Box::new(pair_type))
        } else if let Ok(list_type) = ListType::from_types_form(form) {
            Type::List(Box::new(list_type))
        } else if let Ok(arr_type) = ArrType::from_types_form(form) {
            Type::Arr(Box::new(arr_type))
        } else if let Ok(vec_type) = VecType::from_types_form(form) {
            Type::Vec(Box::new(vec_type))
        } else if let Ok(map_type) = MapType::from_types_form(form) {
            Type::Map(Box::new(map_type))
        } else if let Ok(fun_type) = FunType::from_types_form(form) {
            Type::Fun(Box::new(fun_type))
        } else {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "unexpected form".into(),
            }));
        };

        Ok(t)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            Type::Simple(simple_type) => simple_type.to_string(),
            Type::Enum(enum_type) => enum_type.to_string(),
            Type::Pair(pair_type) => pair_type.to_string(),
            Type::List(list_type) => list_type.to_string(),
            Type::Arr(arr_type) => arr_type.to_string(),
            Type::Vec(vec_type) => vec_type.to_string(),
            Type::Map(map_type) => map_type.to_string(),
            Type::Fun(fun_type) => fun_type.to_string(),
        }
    }
}

impl std::str::FromStr for Type {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
