use super::{FunAppForm, FunAppFormParam};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ValueProdFormValue {
    Empty,
    Prim(String),
    Symbol(String),
    App(FunAppForm),
}

impl Default for ValueProdFormValue {
    fn default() -> ValueProdFormValue {
        ValueProdFormValue::Empty
    }
}

impl ValueProdFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ValueProdFormValue::Empty => "()".into(),
            ValueProdFormValue::Prim(prim) => prim.clone(),
            ValueProdFormValue::Symbol(symbol) => symbol.clone(),
            ValueProdFormValue::App(app) => app.to_string(),
        }
    }
}

impl fmt::Display for ValueProdFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ValueProdForm {
    pub tokens: Tokens,
    pub values: Vec<ValueProdFormValue>,
}

impl ValueProdForm {
    pub fn new() -> ValueProdForm {
        ValueProdForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn from_fun_app(fun_app: &FunAppForm) -> Result<ValueProdForm> {
        if fun_app.name != "prod" {
            return Err(Error::Semantic(SemanticError {
                loc: fun_app.loc(),
                desc: "expected a prod keyword".into(),
            }));
        }

        if fun_app.params.is_empty() {
            return Err(Error::Semantic(SemanticError {
                loc: fun_app.loc(),
                desc: "expected at least a parameter".into(),
            }));
        }

        let mut prod = ValueProdForm::new();
        prod.tokens = fun_app.tokens.clone();

        for param in fun_app.params.iter() {
            match param.clone() {
                FunAppFormParam::Empty => {
                    prod.values.push(ValueProdFormValue::Empty);
                }
                FunAppFormParam::Prim(prim) => {
                    prod.values.push(ValueProdFormValue::Prim(prim));
                }
                FunAppFormParam::Symbol(symbol) => {
                    prod.values.push(ValueProdFormValue::Symbol(symbol));
                }
                FunAppFormParam::App(app) => {
                    prod.values.push(ValueProdFormValue::App(app));
                }
            }
        }

        Ok(prod)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<ValueProdForm> {
        let fun_app = FunAppForm::from_tokens(tokens)?;

        ValueProdForm::from_fun_app(&fun_app)
    }

    pub fn from_str(s: &str) -> Result<ValueProdForm> {
        let tokens = Tokens::from_str(s)?;

        ValueProdForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(prod {})",
            self.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl fmt::Display for ValueProdForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn prod_form_from_str() {
        use super::ValueProdForm;

        let mut s = "(prod 0)";

        let mut res = ValueProdForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(
            form.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec!["0".to_string()]
        );
        assert_eq!(form.to_string(), s.to_string());

        s = "(prod moduleX.x y 'a')";

        res = ValueProdForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec!["moduleX.x".to_string(), "y".to_string(), "'a'".to_string(),]
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
