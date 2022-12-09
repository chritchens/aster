use crate::error::{Error, SyntacticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::types::{SimpleType, Type};
use crate::value::forms::form::{Form, FormTailElement};
use crate::value::SimpleValue;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Default)]
pub struct TypeForm {
    pub tokens: Box<Tokens>,
    pub name: SimpleValue,
    pub value: Box<Type>,
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
        matches!(self.value.as_ref(), Type::Simple(SimpleType::Empty(_)))
    }

    pub fn is_atomic_type(&self) -> bool {
        matches!(self.value.as_ref(), Type::Simple(SimpleType::Atomic(_)))
    }

    pub fn is_type_keyword(&self) -> bool {
        match self.value.as_ref() {
            Type::Simple(simple_type) => !matches!(
                simple_type,
                SimpleType::Symbol(_) | SimpleType::PathSymbol(_)
            ),
            _ => false,
        }
    }

    pub fn is_type_symbol(&self) -> bool {
        match self.value.as_ref() {
            Type::Simple(simple_type) => matches!(
                simple_type,
                SimpleType::Symbol(_) | SimpleType::PathSymbol(_)
            ),
            _ => false,
        }
    }

    pub fn is_types_form(&self) -> bool {
        !matches!(self.value.as_ref(), Type::Simple(_))
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        vec![]
    }

    pub fn all_type_variables(&self) -> Vec<Type> {
        self.value.all_type_variables()
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        self.value.all_variables()
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
                desc: "expected a name and a type".into(),
            }));
        }

        let mut type_form = TypeForm::new();
        type_form.tokens = form.tokens.clone();

        match form.tail[0].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::TypeSymbol(_) => {
                    type_form.name = value;
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: x.loc(),
                        desc: "expected an unqualified type symbol".into(),
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
            FormTailElement::Simple(value) => {
                type_form.value = Box::new(Type::from_simple_value(&value)?);
            }
            FormTailElement::Form(form) => {
                type_form.value = Box::new(Type::from_form(&form)?);
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
        format!("(type {} {})", self.name, self.value)
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

        s = "(type T Atomic)";

        res = TypeForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "T".to_string());
        assert!(form.is_atomic_type());
        assert_eq!(form.value.to_string(), "Atomic".to_string());
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

        s = "(type T (Fun moduleX.X Char (Pair A B)))";

        res = TypeForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "T".to_string());
        assert!(form.is_types_form());
        assert_eq!(
            form.value.to_string(),
            "(Fun moduleX.X Char (Pair A B))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
