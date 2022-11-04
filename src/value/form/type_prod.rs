use super::{MixedAppForm, MixedAppFormParam, TypeAppForm};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypeProdFormValue {
    Symbol(String),
    App(TypeAppForm),
}

impl Default for TypeProdFormValue {
    fn default() -> TypeProdFormValue {
        TypeProdFormValue::Symbol("Empty".into())
    }
}

impl TypeProdFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            TypeProdFormValue::Symbol(symbol) => symbol.clone(),
            TypeProdFormValue::App(app) => app.to_string(),
        }
    }
}

impl fmt::Display for TypeProdFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct TypeProdForm {
    pub tokens: Tokens,
    pub values: Vec<TypeProdFormValue>,
}

impl TypeProdForm {
    pub fn new() -> TypeProdForm {
        TypeProdForm::default()
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

    pub fn from_mixed_app(mixed_app: &MixedAppForm) -> Result<TypeProdForm> {
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

        let mut prod = TypeProdForm::new();
        prod.tokens = mixed_app.tokens.clone();

        for param in mixed_app.params.iter() {
            match param.clone() {
                MixedAppFormParam::TypeSymbol(symbol) => {
                    prod.values.push(TypeProdFormValue::Symbol(symbol));
                }
                MixedAppFormParam::TypeApp(app) => {
                    prod.values.push(TypeProdFormValue::App(app));
                }
                _ => {
                    return Err(Error::Semantic(SemanticError {
                        loc: mixed_app.loc(),
                        desc: "expected a type symbol or a type application".into(),
                    }));
                }
            }
        }

        Ok(prod)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<TypeProdForm> {
        let mixed_app = MixedAppForm::from_tokens(tokens)?;

        TypeProdForm::from_mixed_app(&mixed_app)
    }

    pub fn from_str(s: &str) -> Result<TypeProdForm> {
        let tokens = Tokens::from_str(s)?;

        TypeProdForm::from_tokens(&tokens)
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

impl fmt::Display for TypeProdForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn prod_form_from_str() {
        use super::TypeProdForm;

        let mut s = "(prod Index)";

        let mut res = TypeProdForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(
            form.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec!["Index".to_string()]
        );
        assert_eq!(form.to_string(), s.to_string());

        s = "(prod X moduleY.Y A)";

        res = TypeProdForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
            vec!["X".to_string(), "moduleY.Y".to_string(), "A".to_string(),]
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
