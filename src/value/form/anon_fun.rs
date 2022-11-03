use super::{FunAppForm, FunForm, FunFormBody};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::WILDCARD;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct AnonFunForm {
    pub tokens: Tokens,
    pub params: Vec<String>,
    pub body: FunFormBody,
}

impl AnonFunForm {
    pub fn new() -> AnonFunForm {
        AnonFunForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn from_fun(fun_form: &FunForm) -> Result<AnonFunForm> {
        if fun_form.name != WILDCARD.to_string() {
            Err(Error::Semantic(SemanticError {
                loc: fun_form.loc(),
                desc: "expected a wildcard name".into(),
            }))
        } else {
            let anon_fun = AnonFunForm {
                tokens: fun_form.tokens.clone(),
                params: fun_form.params.clone(),
                body: fun_form.body.clone(),
            };

            Ok(anon_fun)
        }
    }

    pub fn from_fun_app(fun_app: &FunAppForm) -> Result<AnonFunForm> {
        let fun = FunForm::from_fun_app(fun_app)?;

        AnonFunForm::from_fun(&fun)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<AnonFunForm> {
        let fun_app = FunAppForm::from_tokens(tokens)?;

        AnonFunForm::from_fun_app(&fun_app)
    }

    pub fn from_str(s: &str) -> Result<AnonFunForm> {
        let tokens = Tokens::from_str(s)?;

        AnonFunForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        let params = if self.params.is_empty() {
            "()".to_string()
        } else {
            format!(
                "(prod _ {})",
                self.params
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        };

        format!("(fun _ {} {})", params, self.body.to_string())
    }
}

impl fmt::Display for AnonFunForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn anon_fun_form_from_str() {
        use super::AnonFunForm;

        let mut s = "(fun _ () 10)";

        let mut res = AnonFunForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert!(form.params.is_empty());
        assert_eq!(form.body.to_string(), "10".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(fun _ (prod _ a b c d) (+ a b c d 10))";

        res = AnonFunForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.params,
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
            ]
        );
        assert_eq!(form.body.to_string(), "(+ a b c d 10)".to_string());
    }
}
