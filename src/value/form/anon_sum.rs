use super::{FunAppForm, FunAppFormParam};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AnonSumFormValue {
    Prim(String),
    Symbol(String),
    FunApp(FunAppForm),
}

impl Default for AnonSumFormValue {
    fn default() -> AnonSumFormValue {
        AnonSumFormValue::Prim("()".into())
    }
}

impl AnonSumFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            AnonSumFormValue::Prim(prim) => prim.clone(),
            AnonSumFormValue::Symbol(symbol) => symbol.clone(),
            AnonSumFormValue::FunApp(fun_app) => fun_app.to_string(),
        }
    }
}

impl fmt::Display for AnonSumFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct AnonSumForm {
    pub tokens: Tokens,
    pub value: AnonSumFormValue,
}

impl AnonSumForm {
    pub fn new() -> AnonSumForm {
        AnonSumForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn from_fun_app(fun_app: &FunAppForm) -> Result<AnonSumForm> {
        if fun_app.name != "sum" {
            return Err(Error::Semantic(SemanticError {
                loc: fun_app.loc(),
                desc: "expected a sum keyword".into(),
            }));
        }

        if fun_app.params.len() != 1 {
            return Err(Error::Semantic(SemanticError {
                loc: fun_app.loc(),
                desc: "expected a form or a primitive".into(),
            }));
        }

        let mut anon_sum = AnonSumForm::new();
        anon_sum.tokens = fun_app.tokens.clone();

        match fun_app.params[0].clone() {
            FunAppFormParam::Prim(prim) => {
                anon_sum.value = AnonSumFormValue::Prim(prim);
                Ok(anon_sum)
            }
            FunAppFormParam::Symbol(symbol) => {
                anon_sum.value = AnonSumFormValue::Symbol(symbol);
                Ok(anon_sum)
            }
            FunAppFormParam::FunApp(form) => {
                anon_sum.value = AnonSumFormValue::FunApp(form);
                Ok(anon_sum)
            }
        }
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<AnonSumForm> {
        let fun_app = FunAppForm::from_tokens(tokens)?;

        AnonSumForm::from_fun_app(&fun_app)
    }

    pub fn from_str(s: &str) -> Result<AnonSumForm> {
        let tokens = Tokens::from_str(s)?;

        AnonSumForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(sum {})", self.value.to_string())
    }
}

impl fmt::Display for AnonSumForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn anon_sum_form_from_str() {
        use super::AnonSumForm;

        let mut s = "(sum 10)";

        let mut res = AnonSumForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.value.to_string(), "10".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(sum (app f 10 20 \"a\"))";

        res = AnonSumForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.value.to_string(), "(app f 10 20 \"a\")".to_string());
    }
}
