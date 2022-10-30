use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::is_type_symbol;
use crate::syntax::Keyword;
use crate::token::{Token, TokenKind};
use crate::typing::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct PrimValue {
    pub token: Token,
    pub typing: Type,
    pub value: String,
}

impl PrimValue {
    pub fn new() -> PrimValue {
        PrimValue::default()
    }

    pub fn file(&self) -> String {
        self.token.file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.token.loc()
    }

    pub fn from_token(token: Token) -> Result<PrimValue> {
        match token.kind {
            TokenKind::EmptyLiteral
            | TokenKind::UIntLiteral
            | TokenKind::IntLiteral
            | TokenKind::FloatLiteral
            | TokenKind::CharLiteral
            | TokenKind::StringLiteral => {
                let mut prim = PrimValue::new();

                prim.typing = match token.kind {
                    TokenKind::EmptyLiteral => Type::Empty,
                    TokenKind::UIntLiteral => Type::UInt,
                    TokenKind::IntLiteral => Type::Int,
                    TokenKind::FloatLiteral => Type::Float,
                    TokenKind::CharLiteral => Type::Char,
                    TokenKind::StringLiteral => Type::String,
                    _ => unreachable!(),
                };

                prim.value = token.chunks[0].to_string();
                prim.token = token;

                Ok(prim)
            }
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: token.loc(),
                    desc: "expected a primitive value".into(),
                }));
            }
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl fmt::Display for PrimValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SymbolKind {
    Type,
    Value,
}

impl Default for SymbolKind {
    fn default() -> SymbolKind {
        SymbolKind::Type
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct SymbolValue {
    pub token: Token,
    pub kind: SymbolKind,
    pub typing: Type,
    pub value: String,
}

impl SymbolValue {
    pub fn new() -> SymbolValue {
        SymbolValue::default()
    }

    pub fn file(&self) -> String {
        self.token.file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.token.loc()
    }

    pub fn is_keyword(&self) -> bool {
        Keyword::is(&self.value)
    }

    pub fn from_token(token: Token) -> Result<SymbolValue> {
        match token.kind {
            TokenKind::Keyword
            | TokenKind::ValueSymbol
            | TokenKind::TypeSymbol
            | TokenKind::PathSymbol => {
                let string_value = token.chunks[0].to_string();
                let mut symbol = SymbolValue::new();

                if is_type_symbol(&string_value) {
                    symbol.kind = SymbolKind::Type;
                    symbol.typing = Type::Type;
                } else {
                    symbol.kind = SymbolKind::Value;
                    symbol.typing = if Keyword::is(&string_value) {
                        Type::Builtin
                    } else {
                        Type::Unknown(string_value.clone())
                    };
                }

                symbol.value = string_value;
                symbol.token = token;

                Ok(symbol)
            }
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: token.loc(),
                    desc: "expected a symbol".into(),
                }));
            }
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl fmt::Display for SymbolValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FormKind {
    Empty,
    ImportDefs,
    ExportDefs,
    DefType,
    DefSig,
    DefAttrs,
    DefPrim,
    DefSum,
    DefProd,
    DefFun,
    DefApp,
    AnonType,
    AnonSig,
    AnonAttrs,
    AnonPrim,
    AnonSum,
    AnonProd,
    AnonFun,
    TypeApp,
    FunApp,
}

impl Default for FormKind {
    fn default() -> FormKind {
        FormKind::Empty
    }
}

impl FormKind {
    pub fn from_form(form: &FormValue) -> Result<FormKind> {
        let head = form.head();
        let head_value = head.to_string();

        let kind = match head_value.as_str() {
            "()" => FormKind::Empty,
            "import" => FormKind::ImportDefs,
            "export" => FormKind::ExportDefs,
            "type" => FormKind::AnonType,
            "sig" => FormKind::AnonSig,
            "prim" => FormKind::AnonPrim,
            "sum" => FormKind::AnonSum,
            "prod" => FormKind::AnonProd,
            "fun" => FormKind::AnonFun,
            "attrs" => FormKind::AnonAttrs,
            "def" => {
                let tail_head = form.values[1].clone();
                let value = tail_head.to_string();

                match value.as_str() {
                    "type" => FormKind::DefType,
                    "sig" => FormKind::DefSig,
                    "prim" => FormKind::DefPrim,
                    "sum" => FormKind::DefSum,
                    "prod" => FormKind::DefProd,
                    "fun" => FormKind::DefFun,
                    "attrs" => FormKind::DefAttrs,
                    "app" => FormKind::DefApp,
                    _ => match tail_head {
                        Value::Symbol(_) => match form.values[2].clone() {
                            Value::Form(form) => {
                                let head = form.head();
                                let head_value = head.to_string();

                                match head_value.as_str() {
                                    "type" => FormKind::DefType,
                                    "sig" => FormKind::DefSig,
                                    "prim" => FormKind::DefPrim,
                                    "sum" => FormKind::DefSum,
                                    "prod" => FormKind::DefProd,
                                    "fun" => FormKind::DefFun,
                                    "attrs" => FormKind::DefAttrs,
                                    "app" | "let" => FormKind::DefApp,
                                    _ => {
                                        return Err(Error::Semantic(SemanticError {
                                            loc: head.loc(),
                                            desc: "expected a different head value".into(),
                                        }));
                                    }
                                }
                            }
                            _ => {
                                return Err(Error::Semantic(SemanticError {
                                    loc: form.values[2].loc(),
                                    desc: "expected a form".into(),
                                }));
                            }
                        },
                        _ => {
                            return Err(Error::Semantic(SemanticError {
                                loc: tail_head.loc(),
                                desc: format!("expected {} to be a symbol", value),
                            }));
                        }
                    },
                }
            }
            x if is_type_symbol(x) => FormKind::TypeApp,
            _ => FormKind::FunApp,
        };

        Ok(kind)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FormParam {
    Prim(PrimValue),
    Symbol(SymbolValue),
}

impl FormParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            FormParam::Prim(prim) => prim.to_string(),
            FormParam::Symbol(symbol) => symbol.to_string(),
        }
    }
}

impl fmt::Display for FormParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct FormValue {
    pub kind: FormKind,
    pub typing: Vec<Type>,
    pub values: Vec<Value>,
}

impl FormValue {
    pub fn new() -> FormValue {
        FormValue::default()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn head(&self) -> Value {
        self.values[0].clone()
    }

    pub fn file(&self) -> String {
        self.head().file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.head().loc()
    }

    pub fn tail(&self) -> Vec<Value> {
        self.values[1..].into()
    }

    pub fn params(&self) -> Vec<FormParam> {
        let mut params = Vec::new();

        for value in self.values.iter() {
            match value {
                Value::Prim(prim) => {
                    params.push(FormParam::Prim(prim.clone()));
                }
                Value::Symbol(symbol) => {
                    if !symbol.is_keyword() {
                        params.push(FormParam::Symbol(symbol.clone()));
                    }
                }
                _ => {}
            }
        }

        params
    }

    pub fn push_value(&mut self, value: &Value) {
        self.typing.extend(value.typing());
        self.values.push(value.clone());
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "({})",
            self.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl fmt::Display for FormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
    Prim(PrimValue),
    Symbol(SymbolValue),
    Form(FormValue),
}

impl Default for Value {
    fn default() -> Self {
        Value::Prim(PrimValue::default())
    }
}

impl Value {
    pub fn new() -> Value {
        Value::default()
    }

    pub fn file(&self) -> String {
        match self {
            Value::Prim(value) => value.file(),
            Value::Symbol(value) => value.file(),
            Value::Form(value) => value.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            Value::Prim(value) => value.loc(),
            Value::Symbol(value) => value.loc(),
            Value::Form(value) => value.loc(),
        }
    }

    pub fn typing(&self) -> Vec<Type> {
        match self {
            Value::Prim(value) => vec![value.typing.clone()],
            Value::Symbol(value) => vec![value.typing.clone()],
            Value::Form(value) => value.typing.clone(),
        }
    }

    pub fn head_to_string(&self) -> String {
        match self {
            Value::Prim(value) => value.value.clone(),
            Value::Symbol(value) => value.value.clone(),
            Value::Form(value) => value.head().to_string(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            Value::Prim(value) => value.to_string(),
            Value::Symbol(value) => value.to_string(),
            Value::Form(values) => values.to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
