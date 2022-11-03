use super::{MixedAppForm, SigForm, SigFormValue};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::WILDCARD;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct AnonSigForm {
    pub tokens: Tokens,
    pub value: SigFormValue,
}

impl AnonSigForm {
    pub fn new() -> AnonSigForm {
        AnonSigForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn from_sig(sig_form: &SigForm) -> Result<AnonSigForm> {
        if sig_form.name != WILDCARD.to_string() {
            Err(Error::Semantic(SemanticError {
                loc: sig_form.loc(),
                desc: "expected a wildcard name".into(),
            }))
        } else {
            let anon_sig = AnonSigForm {
                tokens: sig_form.tokens.clone(),
                value: sig_form.value.clone(),
            };

            Ok(anon_sig)
        }
    }

    pub fn from_mixed_app(mixed_app: &MixedAppForm) -> Result<AnonSigForm> {
        let sig = SigForm::from_mixed_app(mixed_app)?;

        AnonSigForm::from_sig(&sig)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<AnonSigForm> {
        let mixed_app = MixedAppForm::from_tokens(tokens)?;

        AnonSigForm::from_mixed_app(&mixed_app)
    }

    pub fn from_str(s: &str) -> Result<AnonSigForm> {
        let tokens = Tokens::from_str(s)?;

        AnonSigForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(sig _ {})", self.value.to_string())
    }
}

impl fmt::Display for AnonSigForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn anon_sig_form_from_str() {
        use super::AnonSigForm;

        let mut s = "(sig _ T)";

        let mut res = AnonSigForm::from_str(s);

        //assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.value.to_string(), "T".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(sig _ (Fun IO IO))";

        res = AnonSigForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.value.to_string(), "(Fun IO IO)".to_string());
        assert_eq!(form.to_string(), s.to_string());
    }
}
