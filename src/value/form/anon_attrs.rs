use super::{AttrsForm, FunAppForm};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::WILDCARD;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct AnonAttrsForm {
    pub tokens: Tokens,
    pub values: Vec<String>,
}

impl AnonAttrsForm {
    pub fn new() -> AnonAttrsForm {
        AnonAttrsForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn from_attrs(attrs_form: &AttrsForm) -> Result<AnonAttrsForm> {
        if attrs_form.name != WILDCARD.to_string() {
            Err(Error::Semantic(SemanticError {
                loc: attrs_form.loc(),
                desc: "expected a wildcard name".into(),
            }))
        } else {
            let anon_attrs = AnonAttrsForm {
                tokens: attrs_form.tokens.clone(),
                values: attrs_form.values.clone(),
            };

            Ok(anon_attrs)
        }
    }

    pub fn from_fun_app(fun_app: &FunAppForm) -> Result<AnonAttrsForm> {
        let attrs = AttrsForm::from_fun_app(fun_app)?;

        AnonAttrsForm::from_attrs(&attrs)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<AnonAttrsForm> {
        let fun_app = FunAppForm::from_tokens(tokens)?;

        AnonAttrsForm::from_fun_app(&fun_app)
    }

    pub fn from_str(s: &str) -> Result<AnonAttrsForm> {
        let tokens = Tokens::from_str(s)?;

        AnonAttrsForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(attrs _ (prod {}))",
            self.values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl fmt::Display for AnonAttrsForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn anon_attrs_form_from_str() {
        use super::AnonAttrsForm;

        let mut s = "(attrs _ (prod attr))";

        let mut res = AnonAttrsForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.values, vec!["attr".to_string()]);
        assert_eq!(form.to_string(), s.to_string());

        s = "(attrs _ (prod attr1 attr2 attr3))";

        res = AnonAttrsForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.values,
            vec![
                "attr1".to_string(),
                "attr2".to_string(),
                "attr3".to_string(),
            ]
        );
    }
}
