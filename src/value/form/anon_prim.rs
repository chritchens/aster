use super::{FunAppForm, PrimForm, PrimFormValue};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::WILDCARD;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct AnonPrimForm {
    pub tokens: Tokens,
    pub value: PrimFormValue,
}

impl AnonPrimForm {
    pub fn new() -> AnonPrimForm {
        AnonPrimForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn from_prim(prim_form: &PrimForm) -> Result<AnonPrimForm> {
        if prim_form.name != WILDCARD.to_string() {
            Err(Error::Semantic(SemanticError {
                loc: prim_form.loc(),
                desc: "expected a wildcard name".into(),
            }))
        } else {
            let anon_prim = AnonPrimForm {
                tokens: prim_form.tokens.clone(),
                value: prim_form.value.clone(),
            };

            Ok(anon_prim)
        }
    }

    pub fn from_fun_app(fun_app: &FunAppForm) -> Result<AnonPrimForm> {
        let prim = PrimForm::from_fun_app(fun_app)?;

        AnonPrimForm::from_prim(&prim)
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
        format!("(prim _ {})", self.value.to_string())
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

        let mut s = "(prim _ 'a')";

        let mut res = AnonPrimForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.value.to_string(), "'a'".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(prim _ (/ 32.4E-2 10))";

        res = AnonPrimForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.value.to_string(), "(/ 32.4E-2 10)".to_string());
    }
}
