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
            _ => Err(Error::Semantic(SemanticError {
                loc: token.loc(),
                desc: "expected a primitive value".into(),
            })),
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

    pub fn is_type(&self) -> bool {
        self.kind == SymbolKind::Type
    }

    pub fn is_value(&self) -> bool {
        self.kind == SymbolKind::Value
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
            _ => Err(Error::Semantic(SemanticError {
                loc: token.loc(),
                desc: "expected a symbol".into(),
            })),
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
                                    "app" => FormKind::DefApp,
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

        match self.kind {
            FormKind::AnonFun => {
                for value in self.values[2..].iter() {
                    match value {
                        Value::Prim(prim) => params.push(FormParam::Prim(prim.clone())),
                        Value::Symbol(symbol) => {
                            if !symbol.is_keyword() {
                                params.push(FormParam::Symbol(symbol.clone()))
                            }
                        }
                        _ => {}
                    }
                }
            }
            FormKind::AnonType => {
                for value in self.values[1..].iter() {
                    if let Value::Symbol(symbol) = value {
                        if !symbol.is_keyword() {
                            params.push(FormParam::Symbol(symbol.clone()))
                        }
                    }
                }
            }
            FormKind::DefFun => match &self.values[2] {
                Value::Form(form) => {
                    params = form.params();
                }
                _ => {
                    for value in self.values[3..].iter() {
                        match value {
                            Value::Prim(prim) => params.push(FormParam::Prim(prim.clone())),
                            Value::Symbol(symbol) => {
                                if !symbol.is_keyword() {
                                    params.push(FormParam::Symbol(symbol.clone()))
                                }
                            }
                            _ => {}
                        }
                    }
                }
            },
            FormKind::DefType => match &self.values[2] {
                Value::Form(form) => {
                    params = form.params();
                }
                _ => {
                    for value in self.values[3..].iter() {
                        match value {
                            Value::Prim(prim) => params.push(FormParam::Prim(prim.clone())),
                            Value::Symbol(symbol) => {
                                if !symbol.is_keyword() {
                                    params.push(FormParam::Symbol(symbol.clone()))
                                }
                            }
                            _ => {}
                        }
                    }
                }
            },
            FormKind::FunApp => {
                let mut found_first = false;

                for value in self.values.iter() {
                    match value {
                        Value::Prim(prim) => {
                            params.push(FormParam::Prim(prim.clone()));
                        }
                        Value::Symbol(symbol) => {
                            if !symbol.is_keyword() {
                                if found_first {
                                    params.push(FormParam::Symbol(symbol.clone()));
                                } else {
                                    found_first = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            FormKind::TypeApp => {
                for value in self.values[1..].iter() {
                    if let Value::Symbol(symbol) = value {
                        params.push(FormParam::Symbol(symbol.clone()));
                    }
                }
            }
            _ => {}
        }

        params
    }

    pub fn body(&self) -> Result<Option<Value>> {
        match self.kind {
            FormKind::DefType => match &self.values[self.values.len() - 1] {
                Value::Form(form) => match form.kind {
                    FormKind::AnonType => form.body(),
                    _ => Ok(Some(Value::Form(form.clone()))),
                },
                _ => Ok(None),
            },
            FormKind::DefFun => match &self.values[self.values.len() - 1] {
                Value::Form(form) => match form.kind {
                    FormKind::AnonFun => form.body(),
                    _ => Ok(Some(Value::Form(form.clone()))),
                },
                _ => Ok(None),
            },
            FormKind::AnonType => match &self.values[self.values.len() - 1] {
                Value::Form(form) => {
                    if form.kind != FormKind::TypeApp {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "expected a type application".into(),
                        }));
                    }

                    Ok(Some(Value::Form(form.clone())))
                }
                _ => Ok(None),
            },
            FormKind::AnonFun => match &self.values[self.values.len() - 1] {
                Value::Form(form) => {
                    if form.kind != FormKind::FunApp {
                        return Err(Error::Semantic(SemanticError {
                            loc: form.loc(),
                            desc: "expected a function application".into(),
                        }));
                    }

                    Ok(Some(Value::Form(form.clone())))
                }
                _ => Ok(None),
            },
            _ => Ok(None),
        }
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

    pub fn params(&self) -> Vec<FormParam> {
        match self {
            Value::Form(value) => value.params(),
            _ => vec![],
        }
    }

    pub fn body(&self) -> Result<Option<Value>> {
        match self {
            Value::Form(value) => value.body(),
            _ => Ok(None),
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

#[cfg(test)]
mod tests {
    #[test]
    fn value_params_and_body() {
        use crate::values::Values;

        let mut s = "(def fun f a b (+ a b))";

        let mut values = Values::from_str(s).unwrap();

        let mut value = values[0].clone();

        let mut params = value.params();

        assert_eq!(params.len(), 2);
        assert_eq!(params[0].to_string(), "a");
        assert_eq!(params[1].to_string(), "b");

        if let Some(body) = value.body().unwrap() {
            assert_eq!(body.to_string(), "(+ a b)");
        } else {
            panic!("expected a function body");
        }

        s = "(def f (fun f a b (+ a b)))";

        values = Values::from_str(s).unwrap();

        value = values[0].clone();

        params = value.params();

        assert_eq!(params.len(), 2);
        assert_eq!(params[0].to_string(), "a");
        assert_eq!(params[1].to_string(), "b");

        if let Some(body) = value.body().unwrap() {
            assert_eq!(body.to_string(), "(+ a b)");
        } else {
            panic!("expected a function body");
        }

        s = "(def type F A B (Fun A B))";

        values = Values::from_str(s).unwrap();

        value = values[0].clone();

        params = value.params();

        assert_eq!(params.len(), 2);
        assert_eq!(params[0].to_string(), "A");
        assert_eq!(params[1].to_string(), "B");

        if let Some(body) = value.body().unwrap() {
            assert_eq!(body.to_string(), "(Fun A B)");
        } else {
            panic!("expected a type body");
        }

        s = "(def F (type A B (Fun A B)))";

        values = Values::from_str(s).unwrap();

        value = values[0].clone();

        params = value.params();

        assert_eq!(params.len(), 2);
        assert_eq!(params[0].to_string(), "A");
        assert_eq!(params[1].to_string(), "B");

        if let Some(body) = value.body().unwrap() {
            assert_eq!(body.to_string(), "(Fun A B)");
        } else {
            panic!("expected a type body");
        }

        s = "(f a b c 10)";

        values = Values::from_str(s).unwrap();

        value = values[0].clone();

        params = value.params();

        assert_eq!(params.len(), 4);
        assert_eq!(params[0].to_string(), "a");
        assert_eq!(params[1].to_string(), "b");
        assert_eq!(params[2].to_string(), "c");
        assert_eq!(params[3].to_string(), "10");

        assert!(value.body().unwrap().is_none());

        s = "(app f a b c 10)";

        values = Values::from_str(s).unwrap();

        value = values[0].clone();

        params = value.params();

        assert_eq!(params.len(), 4);
        assert_eq!(params[0].to_string(), "a");
        assert_eq!(params[1].to_string(), "b");
        assert_eq!(params[2].to_string(), "c");
        assert_eq!(params[3].to_string(), "10");

        assert!(value.body().unwrap().is_none());

        s = "(Fun A B)";

        values = Values::from_str(s).unwrap();

        value = values[0].clone();

        params = value.params();

        assert_eq!(params.len(), 2);
        assert_eq!(params[0].to_string(), "A");
        assert_eq!(params[1].to_string(), "B");

        assert!(value.body().unwrap().is_none());

        s = "(type (Fun A B))";

        values = Values::from_str(s).unwrap();

        value = values[0].clone();

        params = value.params();

        assert_eq!(params.len(), 0);

        if let Some(body) = value.body().unwrap() {
            assert_eq!(body.to_string(), "(Fun A B)");
        } else {
            panic!("expected a type body");
        }
    }
}
