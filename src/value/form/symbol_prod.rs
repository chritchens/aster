use super::{FunAppForm, FunAppFormParam};
use super::{MixedAppForm, MixedAppFormParam};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SymbolProdFormValue {
    ValueSymbol(String),
    TypeSymbol(String),
}

impl Default for SymbolProdFormValue {
    fn default() -> SymbolProdFormValue {
        SymbolProdFormValue::TypeSymbol("Empty".into())
    }
}

impl SymbolProdFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            SymbolProdFormValue::ValueSymbol(symbol) => symbol.clone(),
            SymbolProdFormValue::TypeSymbol(symbol) => symbol.clone(),
        }
    }
}

impl fmt::Display for SymbolProdFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct SymbolProdForm {
    pub tokens: Tokens,
    pub values: Vec<SymbolProdFormValue>,
}

impl SymbolProdForm {
    pub fn new() -> SymbolProdForm {
        SymbolProdForm::default()
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

    pub fn from_fun_app(fun_app: &FunAppForm) -> Result<SymbolProdForm> {
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

        let mut prod = SymbolProdForm::new();
        prod.tokens = fun_app.tokens.clone();

        for param in fun_app.params.iter() {
            match param.clone() {
                FunAppFormParam::Symbol(symbol) => {
                    prod.values.push(SymbolProdFormValue::ValueSymbol(symbol));
                }
                _ => {
                    return Err(Error::Semantic(SemanticError {
                        loc: fun_app.loc(),
                        desc: "expected a symbol".into(),
                    }));
                }
            }
        }

        Ok(prod)
    }

    pub fn from_mixed_app(mixed_app: &MixedAppForm) -> Result<SymbolProdForm> {
        if mixed_app.name != "prod" {
            return Err(Error::Semantic(SemanticError {
                loc: mixed_app.loc(),
                desc: "expected a prod keyword".into(),
            }));
        }

        if mixed_app.params.is_empty() {
            return Err(Error::Semantic(SemanticError {
                loc: mixed_app.loc(),
                desc: "expected at least a parameter".into(),
            }));
        }

        let mut prod = SymbolProdForm::new();
        prod.tokens = mixed_app.tokens.clone();

        for param in mixed_app.params.iter() {
            match param.clone() {
                MixedAppFormParam::ValueSymbol(symbol) => {
                    prod.values.push(SymbolProdFormValue::ValueSymbol(symbol));
                }
                MixedAppFormParam::TypeSymbol(symbol) => {
                    prod.values.push(SymbolProdFormValue::TypeSymbol(symbol));
                }
                _ => {
                    return Err(Error::Semantic(SemanticError {
                        loc: mixed_app.loc(),
                        desc: "expected a symbol".into(),
                    }));
                }
            }
        }

        Ok(prod)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<SymbolProdForm> {
        let mixed_app = MixedAppForm::from_tokens(tokens)?;

        SymbolProdForm::from_mixed_app(&mixed_app)
    }

    pub fn from_str(s: &str) -> Result<SymbolProdForm> {
        let tokens = Tokens::from_str(s)?;

        SymbolProdForm::from_tokens(&tokens)
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

impl fmt::Display for SymbolProdForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn prod_form_from_str() {
        use super::SymbolProdForm;

        let mut s = "(prod symbol)";

        let mut res = SymbolProdForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(
            form.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec!["symbol".to_string()]
        );
        assert_eq!(form.to_string(), s.to_string());

        s = "(prod moduleX.x y z)";

        res = SymbolProdForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec!["moduleX.x".to_string(), "y".to_string(), "z".to_string(),]
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
