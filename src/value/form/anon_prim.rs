use super::{FunAppForm, FunAppFormParam};
use crate::error::{Error, SemanticError};
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AnonPrimFormValue {
    Prim(String),
    Symbol(String),
    FunApp(FunAppForm),
}

impl Default for AnonPrimFormValue {
    fn default() -> AnonPrimFormValue {
        AnonPrimFormValue::Prim("()".into())
    }
}

impl AnonPrimFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            AnonPrimFormValue::Prim(prim) => prim.clone(),
            AnonPrimFormValue::Symbol(symbol) => symbol.clone(),
            AnonPrimFormValue::FunApp(fun_app) => fun_app.to_string(),
        }
    }
}

impl fmt::Display for AnonPrimFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct AnonPrimForm {
    pub tokens: Tokens,
    pub value: AnonPrimFormValue,
}

impl AnonPrimForm {
    pub fn new() -> AnonPrimForm {
        AnonPrimForm::default()
    }

    pub fn from_fun_app(fun_app: &FunAppForm) -> Result<AnonPrimForm> {
        if fun_app.name != "prim" {
            return Err(Error::Semantic(SemanticError {
                loc: fun_app.loc(),
                desc: "expected a prim keyword".into(),
            }));
        }

        if fun_app.params.len() != 1 {
            return Err(Error::Semantic(SemanticError {
                loc: fun_app.loc(),
                desc: "expected a form or a primitive".into(),
            }));
        }

        let mut anon_prim = AnonPrimForm::new();
        anon_prim.tokens = fun_app.tokens.clone();

        match fun_app.params[0].clone() {
            FunAppFormParam::Prim(prim) => {
                anon_prim.value = AnonPrimFormValue::Prim(prim);
                Ok(anon_prim)
            }
            FunAppFormParam::Symbol(symbol) => {
                anon_prim.value = AnonPrimFormValue::Symbol(symbol);
                Ok(anon_prim)
            }
            FunAppFormParam::FunApp(form) => {
                anon_prim.value = AnonPrimFormValue::FunApp(form);
                Ok(anon_prim)
            }
        }
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<AnonPrimForm> {
        let fun_app = FunAppForm::from_tokens(tokens)?;

        AnonPrimForm::from_fun_app(&fun_app)
    }

    pub fn from_str(s: &str) -> Result<AnonPrimForm> {
        let tokens = Tokens::from_str(s)?;

        AnonPrimForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(prim {})", self.value.to_string())
    }
}

impl fmt::Display for AnonPrimForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn anon_prim_form_from_str() {
        use super::AnonPrimForm;

        let mut s = "(prim 'a')";

        let mut res = AnonPrimForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.value.to_string(), "'a'".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(prim (/ 32.4E-2 10))";

        res = AnonPrimForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.value.to_string(), "(/ 32.4E-2 10)".to_string());
    }
}
