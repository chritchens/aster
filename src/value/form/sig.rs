use super::{MixedAppForm, MixedAppFormParam, TypeAppForm};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_type_symbol, WILDCARD};
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SigFormValue {
    TypeSymbol(String),
    TypeApp(TypeAppForm),
}

impl Default for SigFormValue {
    fn default() -> SigFormValue {
        SigFormValue::TypeSymbol("Empty".into())
    }
}

impl SigFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            SigFormValue::TypeSymbol(symbol) => symbol.clone(),
            SigFormValue::TypeApp(type_app) => type_app.to_string(),
        }
    }
}

impl fmt::Display for SigFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct SigForm {
    pub tokens: Tokens,
    pub name: Option<String>,
    pub value: SigFormValue,
}

impl SigForm {
    pub fn new() -> SigForm {
        SigForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn is_anonymous(&self) -> bool {
        self.name.is_none()
    }

    pub fn from_mixed_app(mixed_app: &MixedAppForm) -> Result<SigForm> {
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

        let mut sig = SigForm::new();
        sig.tokens = mixed_app.tokens.clone();

        match mixed_app.params[0].clone() {
            MixedAppFormParam::Wildcard => {
                sig.name = None;
            }
            MixedAppFormParam::ValueSymbol(symbol) => {
                sig.name = Some(symbol);
            }
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: mixed_app.loc(),
                    desc: "expected a value symbol".into(),
                }));
            }
        }

        match mixed_app.params[1].clone() {
            MixedAppFormParam::TypeSymbol(symbol) => {
                if !is_type_symbol(&symbol) {
                    return Err(Error::Semantic(SemanticError {
                        loc: mixed_app.loc(),
                        desc: "expected a type symbol".into(),
                    }));
                }

                sig.value = SigFormValue::TypeSymbol(symbol);
            }
            MixedAppFormParam::TypeApp(form) => {
                sig.value = SigFormValue::TypeApp(form);
            }
            _ => {
                return Err(Error::Semantic(SemanticError {
                    loc: mixed_app.loc(),
                    desc: "expected a type symbol or a type form".into(),
                }));
            }
        }

        Ok(sig)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<SigForm> {
        let mixed_app = MixedAppForm::from_tokens(tokens)?;

        SigForm::from_mixed_app(&mixed_app)
    }

    pub fn from_str(s: &str) -> Result<SigForm> {
        let tokens = Tokens::from_str(s)?;

        SigForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!(
            "(sig {} {})",
            self.name.clone().unwrap_or_else(|| WILDCARD.to_string()),
            self.value.to_string()
        )
    }
}

impl fmt::Display for SigForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sig_form_from_str() {
        use super::SigForm;

        let mut s = "(sig t T)";

        let mut res = SigForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, Some("t".into()));
        assert_eq!(form.value.to_string(), "T".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(sig main (Fun IO IO))";

        res = SigForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, Some("main".into()));
        assert_eq!(form.value.to_string(), "(Fun IO IO)".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(sig _ Empty)";

        res = SigForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert!(form.name.is_none());
        assert!(form.is_anonymous());
    }
}
