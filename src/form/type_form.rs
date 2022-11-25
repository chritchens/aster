use crate::error::{Error, SyntacticError};
use crate::form::form::{Form, FormTailElement};
use crate::form::simple_value::SimpleValue;
use crate::form::types_form::{TypesForm, TypesFormTailElement};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

pub type TypeFormValue = TypesFormTailElement;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct TypeForm {
    pub tokens: Box<Tokens>,
    pub name: SimpleValue,
    pub value: TypeFormValue,
}

impl TypeForm {
    pub fn new() -> TypeForm {
        TypeForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn is_empty_type(&self) -> bool {
        match self.value {
            TypeFormValue::Empty(_) => true,
            _ => false,
        }
    }

    pub fn is_primitive_type(&self) -> bool {
        match self.value {
            TypeFormValue::Prim(_) => true,
            _ => false,
        }
    }

    pub fn is_type_keyword(&self) -> bool {
        match self.value {
            TypeFormValue::Keyword(_) => true,
            _ => false,
        }
    }

    pub fn is_type_symbol(&self) -> bool {
        match self.value {
            TypeFormValue::Symbol(_) => true,
            _ => false,
        }
    }

    pub fn is_types_form(&self) -> bool {
        match self.value {
            TypeFormValue::Form(_) => true,
            _ => false,
        }
    }

    pub fn from_form(form: &Form) -> Result<TypeForm> {
        if form.head.to_string() != "type" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a type keyword".into(),
            }));
        }

        if form.tail.len() != 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a name and a type keyword or a type symbol or a types form".into(),
            }));
        }

        let mut type_form = TypeForm::new();
        type_form.tokens = form.tokens.clone();

        match form.tail[0].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::TypeSymbol(_) => {
                    type_form.name = value;
                }
                _ => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a type symbol".into(),
                    }));
                }
            },
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "expected a type symbol".into(),
                }));
            }
        }

        match form.tail[1].clone() {
            FormTailElement::Simple(value) => match value.clone() {
                SimpleValue::TypeKeyword(keyword) => match keyword.to_string().as_str() {
                    "Empty" => {
                        type_form.value = TypeFormValue::Empty(value);
                    }
                    "Prim" => {
                        type_form.value = TypeFormValue::Prim(value);
                    }
                    _ => {
                        type_form.value = TypeFormValue::Keyword(value);
                    }
                },
                SimpleValue::TypeSymbol(_) => {
                    type_form.value = TypeFormValue::Symbol(value);
                }
                SimpleValue::TypePathSymbol(_) => {
                    type_form.value = TypeFormValue::PathSymbol(value);
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
                    type_form.value = TypeFormValue::Form(Box::new(form));
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a form of types".into(),
                    }));
                }
            }
        }

        Ok(type_form)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<TypeForm> {
        let form = Form::from_tokens(tokens)?;

        TypeForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<TypeForm> {
        let tokens = Tokens::from_str(s)?;

        TypeForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(type {} {})", self.name, self.value.to_string(),)
    }
}

impl fmt::Display for TypeForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for TypeForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn type_form_from_str() {
        use super::TypeForm;

        let mut s = "(type T Empty)";

        let mut res = TypeForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name.to_string(), "T".to_string());
        assert!(form.is_empty_type());
        assert_eq!(form.value.to_string(), "Empty".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(type T Prim)";

        res = TypeForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "T".to_string());
        assert!(form.is_primitive_type());
        assert_eq!(form.value.to_string(), "Prim".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(type T Char)";

        res = TypeForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "T".to_string());
        assert!(form.is_type_keyword());
        assert_eq!(form.value.to_string(), "Char".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(type T X)";

        res = TypeForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "T".to_string());
        assert!(form.is_type_symbol());
        assert_eq!(form.value.to_string(), "X".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(type T (Fun (Prod moduleX.X Char) (Prod A B C)))";

        res = TypeForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "T".to_string());
        assert!(form.is_types_form());
        assert_eq!(
            form.value.to_string(),
            "(Fun (Prod moduleX.X Char) (Prod A B C))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
