use crate::error::{Error, SyntacticError};
use crate::form::form::{Form, FormParam};
use crate::form::types_form::TypesForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_qualified, is_value_symbol};
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SigFormValue {
    Empty,
    Prim,
    Keyword(String),
    Symbol(String),
    Form(Box<TypesForm>),
}

impl Default for SigFormValue {
    fn default() -> SigFormValue {
        SigFormValue::Empty
    }
}

impl SigFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            SigFormValue::Empty => "Empty".into(),
            SigFormValue::Prim => "Prim".into(),
            SigFormValue::Keyword(keyword) => keyword.clone(),
            SigFormValue::Symbol(symbol) => symbol.clone(),
            SigFormValue::Form(form) => form.to_string(),
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
    pub tokens: Box<Tokens>,
    pub name: String,
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
            SigFormValue::Empty => true,
            _ => false,
        }
    }

    pub fn is_primitive_type(&self) -> bool {
        match self.value {
            SigFormValue::Prim => true,
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

    pub fn from_form(form: &Form) -> Result<SigForm> {
        if form.name != "sig" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a sig keyword".into(),
            }));
        }

        if form.params.len() != 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a name and a type keyword or a type symbol or a types form".into(),
            }));
        }

        let mut sig_form = SigForm::new();
        sig_form.tokens = form.tokens.clone();

        let name = form.params[0].to_string();

        if is_qualified(&name) {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected an unqualified name".into(),
            }));
        }

        if !is_value_symbol(&name) {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a value symbol".into(),
            }));
        }

        sig_form.name = name;

        match form.params[1].clone() {
            FormParam::TypeKeyword(keyword) => match keyword.as_str() {
                "Empty" => {
                    sig_form.value = SigFormValue::Empty;
                }
                "Prim" => {
                    sig_form.value = SigFormValue::Prim;
                }
                _ => {
                    sig_form.value = SigFormValue::Keyword(keyword);
                }
            },
            FormParam::TypeSymbol(symbol) => {
                sig_form.value = SigFormValue::Symbol(symbol);
            }
            FormParam::Form(form) => {
                if let Ok(form) = TypesForm::from_form(&form) {
                    sig_form.value = SigFormValue::Form(Box::new(form));
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a form of types".into(),
                    }));
                }
            }
            x => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: format!("unexpected value: {}", x.to_string()),
                }));
            }
        }

        Ok(sig_form)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<SigForm> {
        let form = Form::from_tokens(tokens)?;

        SigForm::from_form(&form)
    }

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

#[cfg(test)]
mod tests {
    #[test]
    fn sig_form_from_str() {
        use super::SigForm;
        use super::SigFormValue;

        let mut s = "(sig t Empty)";

        let mut res = SigForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "t".to_string());
        assert!(form.is_empty_type());
        assert_eq!(form.value, SigFormValue::Empty);
        assert_eq!(form.value.to_string(), "Empty".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(sig t Prim)";

        res = SigForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "t".to_string());
        assert!(form.is_primitive_type());
        assert_eq!(form.value, SigFormValue::Prim);
        assert_eq!(form.value.to_string(), "Prim".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(sig t Char)";

        res = SigForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "t".to_string());
        assert!(form.is_type_keyword());
        assert_eq!(form.value.to_string(), "Char".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(sig t X)";

        res = SigForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "t".to_string());
        assert!(form.is_type_symbol());
        assert_eq!(form.value.to_string(), "X".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(sig t (Fun (Prod moduleX.X Char) (Prod A B C)))";

        res = SigForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "t".to_string());
        assert!(form.is_types_form());
        assert_eq!(
            form.value.to_string(),
            "(Fun (Prod moduleX.X Char) (Prod A B C))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
