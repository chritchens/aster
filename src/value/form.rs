use super::{PrimValue, SymbolValue, Value};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::is_type_symbol;
use crate::typing::Type;
use std::fmt;

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
