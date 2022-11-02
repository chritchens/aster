use super::{MixedAppForm, MixedAppFormParam, TypeAppForm};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::is_type_symbol;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AnonSigFormValue {
    TypeSymbol(String),
    TypeApp(TypeAppForm),
}

impl Default for AnonSigFormValue {
    fn default() -> AnonSigFormValue {
        AnonSigFormValue::TypeSymbol("Empty".into())
    }
}

impl AnonSigFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            AnonSigFormValue::TypeSymbol(symbol) => symbol.clone(),
            AnonSigFormValue::TypeApp(type_app) => type_app.to_string(),
        }
    }
}

impl fmt::Display for AnonSigFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct AnonSigForm {
    pub tokens: Tokens,
    pub name: String,
    pub value: AnonSigFormValue,
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

    pub fn from_mixed_app(mixed_app: &MixedAppForm) -> Result<AnonSigForm> {
        if mixed_app.name != "sig" {
            return Err(Error::Semantic(SemanticError {
                loc: mixed_app.loc(),
                desc: "expected a sig keyword".into(),
            }));
        }

        if mixed_app.params.len() != 2 {
            return Err(Error::Semantic(SemanticError {
                loc: mixed_app.loc(),
                desc: "expected a name and a type".into(),
            }));
        }

        let mut anon_sig = AnonSigForm::new();
        anon_sig.tokens = mixed_app.tokens.clone();

        match mixed_app.params[0].clone() {
            MixedAppFormParam::Symbol(symbol) => {
                anon_sig.name = symbol;
            }
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: mixed_app.loc(),
                    desc: "expected a symbol".into(),
                }));
            }
        }

        match mixed_app.params[1].clone() {
            MixedAppFormParam::Symbol(symbol) => {
                if !is_type_symbol(&symbol) {
                    return Err(Error::Semantic(SemanticError {
                        loc: mixed_app.loc(),
                        desc: "expected a type symbol".into(),
                    }));
                }

                anon_sig.value = AnonSigFormValue::TypeSymbol(symbol);
            }
            MixedAppFormParam::TypeApp(form) => {
                anon_sig.value = AnonSigFormValue::TypeApp(form);
            }
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: mixed_app.loc(),
                    desc: "expected a type symbol or a type form".into(),
                }));
            }
        }

        Ok(anon_sig)
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
        format!("(sig {} {})", self.name.to_string(), self.value.to_string())
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

        let mut s = "(sig t T)";

        let mut res = AnonSigForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "t".to_string());
        assert_eq!(form.value.to_string(), "T".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(sig main (Fun IO IO))";

        res = AnonSigForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "main".to_string());
        assert_eq!(form.value.to_string(), "(Fun IO IO)".to_string());
        assert_eq!(form.to_string(), s.to_string());
    }
}
