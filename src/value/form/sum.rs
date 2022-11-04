use super::{FunAppForm, FunAppFormParam};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SumFormValue {
    Prim(String),
    Symbol(String),
    FunApp(FunAppForm),
}

impl Default for SumFormValue {
    fn default() -> SumFormValue {
        SumFormValue::Prim("()".into())
    }
}

impl SumFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            SumFormValue::Prim(prim) => prim.clone(),
            SumFormValue::Symbol(symbol) => symbol.clone(),
            SumFormValue::FunApp(fun_app) => fun_app.to_string(),
        }
    }
}

impl fmt::Display for SumFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct SumForm {
    pub tokens: Tokens,
    pub value: SumFormValue,
}

impl SumForm {
    pub fn new() -> SumForm {
        SumForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn from_fun_app(fun_app: &FunAppForm) -> Result<SumForm> {
        if fun_app.name != "sum" {
            return Err(Error::Semantic(SemanticError {
                loc: fun_app.loc(),
                desc: "expected a sum keyword".into(),
            }));
        }

        if fun_app.params.len() != 1 {
            return Err(Error::Semantic(SemanticError {
                loc: fun_app.loc(),
                desc: "expected a form or a symbol or a primitive".into(),
            }));
        }

        let mut sum = SumForm::new();
        sum.tokens = fun_app.tokens.clone();

        match fun_app.params[0].clone() {
            FunAppFormParam::Prim(prim) => {
                sum.value = SumFormValue::Prim(prim);
                Ok(sum)
            }
            FunAppFormParam::Symbol(symbol) => {
                sum.value = SumFormValue::Symbol(symbol);
                Ok(sum)
            }
            FunAppFormParam::FunApp(form) => {
                sum.value = SumFormValue::FunApp(form);
                Ok(sum)
            }
            _ => Err(Error::Semantic(SemanticError {
                loc: fun_app.loc(),
                desc: "expected a function application form, or a symbol or a primitive".into(),
            })),
        }
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<SumForm> {
        let fun_app = FunAppForm::from_tokens(tokens)?;

        SumForm::from_fun_app(&fun_app)
    }

    pub fn from_str(s: &str) -> Result<SumForm> {
        let tokens = Tokens::from_str(s)?;

        SumForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(sum {})", self.value.to_string())
    }
}

impl fmt::Display for SumForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_form_from_str() {
        use super::SumForm;

        let mut s = "(sum 10)";

        let mut res = SumForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.value.to_string(), "10".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(sum (app f 10 20 \"a\"))";

        res = SumForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.value.to_string(), "(app f 10 20 \"a\")".to_string());
    }
}
