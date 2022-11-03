use super::{FunAppForm, FunAppFormParam};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PrimFormValue {
    Prim(String),
    Symbol(String),
    FunApp(FunAppForm),
}

impl Default for PrimFormValue {
    fn default() -> PrimFormValue {
        PrimFormValue::Prim("()".into())
    }
}

impl PrimFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            PrimFormValue::Prim(prim) => prim.clone(),
            PrimFormValue::Symbol(symbol) => symbol.clone(),
            PrimFormValue::FunApp(fun_app) => fun_app.to_string(),
        }
    }
}

impl fmt::Display for PrimFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct PrimForm {
    pub tokens: Tokens,
    pub name: String,
    pub value: PrimFormValue,
}

impl PrimForm {
    pub fn new() -> PrimForm {
        PrimForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn from_fun_app(fun_app: &FunAppForm) -> Result<PrimForm> {
        if fun_app.name != "prim" {
            return Err(Error::Semantic(SemanticError {
                loc: fun_app.loc(),
                desc: "expected a prim keyword".into(),
            }));
        }

        if fun_app.params.len() != 2 {
            return Err(Error::Semantic(SemanticError {
                loc: fun_app.loc(),
                desc: "expected a name and a form or a primitive".into(),
            }));
        }

        let mut prim = PrimForm::new();
        prim.tokens = fun_app.tokens.clone();

        match fun_app.params[0].clone() {
            FunAppFormParam::Symbol(symbol) => {
                prim.name = symbol;
            }
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: fun_app.loc(),
                    desc: "expected a symbol".into(),
                }));
            }
        }

        match fun_app.params[1].clone() {
            FunAppFormParam::Prim(prim_value) => {
                prim.value = PrimFormValue::Prim(prim_value);
                Ok(prim)
            }
            FunAppFormParam::Symbol(symbol) => {
                prim.value = PrimFormValue::Symbol(symbol);
                Ok(prim)
            }
            FunAppFormParam::FunApp(form) => {
                prim.value = PrimFormValue::FunApp(form);
                Ok(prim)
            }
        }
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<PrimForm> {
        let fun_app = FunAppForm::from_tokens(tokens)?;

        PrimForm::from_fun_app(&fun_app)
    }

    pub fn from_str(s: &str) -> Result<PrimForm> {
        let tokens = Tokens::from_str(s)?;

        PrimForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(prim {} {})",
            self.name.to_string(),
            self.value.to_string()
        )
    }
}

impl fmt::Display for PrimForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn prim_form_from_str() {
        use super::PrimForm;

        let mut s = "(prim a 'a')";

        let mut res = PrimForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "a".to_string());
        assert_eq!(form.value.to_string(), "'a'".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(prim x (/ 32.4E-2 10))";

        res = PrimForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "x".to_string());
        assert_eq!(form.value.to_string(), "(/ 32.4E-2 10)".to_string());
    }
}
