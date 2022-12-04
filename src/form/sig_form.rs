use crate::error::{Error, SyntacticError};
use crate::form::form::{Form, FormTailElement};
use crate::form::types_form::{TypesForm, TypesFormTailElement};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::value::SimpleValue;
use std::fmt;

pub type SigFormValue = TypesFormTailElement;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct SigForm {
    pub tokens: Box<Tokens>,
    pub name: SimpleValue,
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

    pub fn is_empty_type(&self) -> bool {
        match self.value {
            SigFormValue::Empty(_) => true,
            _ => false,
        }
    }

    pub fn is_atomic_type(&self) -> bool {
        match self.value {
            SigFormValue::Atomic(_) => true,
            _ => false,
        }
    }

    pub fn is_type_keyword(&self) -> bool {
        match self.value {
            SigFormValue::Keyword(_) => true,
            _ => false,
        }
    }

    pub fn is_type_symbol(&self) -> bool {
        match self.value {
            SigFormValue::Symbol(_) => true,
            _ => false,
        }
    }

    pub fn is_types_form(&self) -> bool {
        match self.value {
            SigFormValue::Form(_) => true,
            _ => false,
        }
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        vec![]
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        vec![]
    }

    pub fn from_form(form: &Form) -> Result<SigForm> {
        if form.head.to_string() != "sig" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected a sig keyword".into(),
            }));
        }

        if form.tail.len() != 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a name and a type".into(),
            }));
        }

        let mut sig_form = SigForm::new();
        sig_form.tokens = form.tokens.clone();

        match form.tail[0].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::ValueSymbol(_) => {
                    sig_form.name = value;
                }
                _ => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected an unqualified value symbol".into(),
                    }));
                }
            },
            x => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: x.loc(),
                    desc: "unexpected form".into(),
                }));
            }
        }

        match form.tail[1].clone() {
            FormTailElement::Simple(value) => match value.clone() {
                SimpleValue::TypeKeyword(keyword) => match keyword.to_string().as_str() {
                    "Empty" => {
                        sig_form.value = SigFormValue::Empty(value);
                    }
                    "Atomic" => {
                        sig_form.value = SigFormValue::Atomic(value);
                    }
                    _ => {
                        sig_form.value = SigFormValue::Keyword(value);
                    }
                },
                SimpleValue::TypeSymbol(_) => {
                    sig_form.value = SigFormValue::Symbol(value);
                }
                SimpleValue::TypePathSymbol(_) => {
                    sig_form.value = SigFormValue::PathSymbol(value);
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: format!("unexpected value: {}", x.to_string()),
                    }));
                }
            },
            FormTailElement::Form(form) => {
                if let Ok(form) = TypesForm::from_form(&form) {
                    sig_form.value = SigFormValue::Form(Box::new(form));
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a form of types".into(),
                    }));
                }
            }
        }

        Ok(sig_form)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<SigForm> {
        let form = Form::from_tokens(tokens)?;

        SigForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<SigForm> {
        let tokens = Tokens::from_str(s)?;

        SigForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(sig {} {})", self.name, self.value.to_string(),)
    }
}

impl fmt::Display for SigForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for SigForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sig_form_from_str() {
        use super::SigForm;

        let mut s = "(sig t Empty)";

        let mut res = SigForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name.to_string(), "t".to_string());
        assert!(form.is_empty_type());
        assert_eq!(form.value.to_string(), "Empty".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(sig t Atomic)";

        res = SigForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "t".to_string());
        assert!(form.is_atomic_type());
        assert_eq!(form.value.to_string(), "Atomic".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(sig t Char)";

        res = SigForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "t".to_string());
        assert!(form.is_type_keyword());
        assert_eq!(form.value.to_string(), "Char".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(sig t X)";

        res = SigForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "t".to_string());
        assert!(form.is_type_symbol());
        assert_eq!(form.value.to_string(), "X".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(sig t (Fun moduleX.X Char (Pair A B)))";

        res = SigForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "t".to_string());
        assert!(form.is_types_form());
        assert_eq!(
            form.value.to_string(),
            "(Fun moduleX.X Char (Pair A B))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
