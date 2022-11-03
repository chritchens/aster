use super::{FunAppForm, SumForm, SumFormValue};
use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::WILDCARD;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct AnonSumForm {
    pub tokens: Tokens,
    pub value: SumFormValue,
}

impl AnonSumForm {
    pub fn new() -> AnonSumForm {
        AnonSumForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn from_sum(sum_form: &SumForm) -> Result<AnonSumForm> {
        if sum_form.name != WILDCARD.to_string() {
            Err(Error::Semantic(SemanticError {
                loc: sum_form.loc(),
                desc: "expected a wildcard name".into(),
            }))
        } else {
            let anon_sum = AnonSumForm {
                tokens: sum_form.tokens.clone(),
                value: sum_form.value.clone(),
            };

            Ok(anon_sum)
        }
    }

    pub fn from_fun_app(fun_app: &FunAppForm) -> Result<AnonSumForm> {
        let sum = SumForm::from_fun_app(fun_app)?;

        AnonSumForm::from_sum(&sum)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<AnonSumForm> {
        let fun_app = FunAppForm::from_tokens(tokens)?;

        AnonSumForm::from_fun_app(&fun_app)
    }

    pub fn from_str(s: &str) -> Result<AnonSumForm> {
        let tokens = Tokens::from_str(s)?;

        AnonSumForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(sum _ {})", self.value.to_string())
    }
}

impl fmt::Display for AnonSumForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn anon_sum_form_from_str() {
        use super::AnonSumForm;

        let mut s = "(sum _ 10)";

        let mut res = AnonSumForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.value.to_string(), "10".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(sum _ (app f 10 20 \"a\"))";

        res = AnonSumForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.value.to_string(), "(app f 10 20 \"a\")".to_string());
    }
}
