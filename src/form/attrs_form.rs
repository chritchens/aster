use crate::error::{Error, SyntacticError};
use crate::form::form::{Form, FormTailElement};
use crate::form::map_form::MapForm;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::value::SimpleValue;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AttrsFormValue {
    Empty(SimpleValue),
    Panic(SimpleValue),
    Atomic(SimpleValue),
    ValueSymbol(SimpleValue),
    TypeSymbol(SimpleValue),
    ValuePathSymbol(SimpleValue),
    TypePathSymbol(SimpleValue),
    Map(Box<MapForm>),
}

impl Default for AttrsFormValue {
    fn default() -> AttrsFormValue {
        AttrsFormValue::Empty(SimpleValue::new())
    }
}

impl AttrsFormValue {
    pub fn file(&self) -> String {
        match self {
            AttrsFormValue::Empty(empty) => empty.file(),
            AttrsFormValue::Panic(empty) => empty.file(),
            AttrsFormValue::Atomic(value) => value.file(),
            AttrsFormValue::ValueSymbol(value) => value.file(),
            AttrsFormValue::TypeSymbol(value) => value.file(),
            AttrsFormValue::ValuePathSymbol(value) => value.file(),
            AttrsFormValue::TypePathSymbol(value) => value.file(),
            AttrsFormValue::Map(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            AttrsFormValue::Empty(empty) => empty.loc(),
            AttrsFormValue::Panic(value) => value.loc(),
            AttrsFormValue::Atomic(value) => value.loc(),
            AttrsFormValue::ValueSymbol(value) => value.loc(),
            AttrsFormValue::TypeSymbol(value) => value.loc(),
            AttrsFormValue::ValuePathSymbol(value) => value.loc(),
            AttrsFormValue::TypePathSymbol(value) => value.loc(),
            AttrsFormValue::Map(form) => form.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            AttrsFormValue::Empty(_) => "()".into(),
            AttrsFormValue::Panic(value) => value.to_string(),
            AttrsFormValue::Atomic(value) => value.to_string(),
            AttrsFormValue::ValueSymbol(value) => value.to_string(),
            AttrsFormValue::TypeSymbol(value) => value.to_string(),
            AttrsFormValue::ValuePathSymbol(value) => value.to_string(),
            AttrsFormValue::TypePathSymbol(value) => value.to_string(),
            AttrsFormValue::Map(form) => form.to_string(),
        }
    }
}

impl fmt::Display for AttrsFormValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct AttrsForm {
    pub tokens: Box<Tokens>,
    pub name: SimpleValue,
    pub values: Vec<AttrsFormValue>,
}

impl AttrsForm {
    pub fn new() -> AttrsForm {
        AttrsForm::default()
    }

    pub fn file(&self) -> String {
        self.tokens[0].file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.tokens[0].loc()
    }

    pub fn is_type_attributes(&self) -> bool {
        match self.name {
            SimpleValue::TypeSymbol(_) => true,
            _ => false,
        }
    }

    pub fn is_value_attributes(&self) -> bool {
        match self.name {
            SimpleValue::ValueSymbol(_) => true,
            _ => false,
        }
    }

    pub fn values_to_string(&self) -> String {
        self.values
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn all_parameters(&self) -> Vec<SimpleValue> {
        vec![]
    }

    pub fn all_variables(&self) -> Vec<SimpleValue> {
        vec![]
    }

    pub fn from_form(form: &Form) -> Result<AttrsForm> {
        if form.head.to_string() != "attrs" {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.head.loc(),
                desc: "expected a attrs keyword".into(),
            }));
        }

        if form.tail.len() != 2 {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a name and a value".into(),
            }));
        }

        let mut attrs = AttrsForm::new();
        attrs.tokens = form.tokens.clone();

        match form.tail[0].clone() {
            FormTailElement::Simple(value) => match value {
                SimpleValue::ValueSymbol(_) => {
                    attrs.name = value;
                }
                SimpleValue::TypeSymbol(_) => {
                    attrs.name = value;
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: x.loc(),
                        desc: "expected an unqualified symbol".into(),
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
            FormTailElement::Simple(value) => match value {
                SimpleValue::Empty(_) => {
                    attrs.values.push(AttrsFormValue::Empty(value));
                }
                SimpleValue::Panic(_) => {
                    attrs.values.push(AttrsFormValue::Panic(value));
                }
                SimpleValue::Atomic(_) => {
                    attrs.values.push(AttrsFormValue::Atomic(value));
                }
                SimpleValue::ValueSymbol(_) => {
                    attrs.values.push(AttrsFormValue::ValueSymbol(value));
                }
                SimpleValue::TypeSymbol(_) => {
                    attrs.values.push(AttrsFormValue::TypeSymbol(value));
                }
                SimpleValue::ValuePathSymbol(_) => {
                    attrs.values.push(AttrsFormValue::ValuePathSymbol(value));
                }
                SimpleValue::TypePathSymbol(_) => {
                    attrs.values.push(AttrsFormValue::TypePathSymbol(value));
                }
                x => {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: x.loc(),
                        desc: "unexpected value".into(),
                    }));
                }
            },
            FormTailElement::Form(form) => {
                if let Ok(map) = MapForm::from_form(&form) {
                    attrs.values.push(AttrsFormValue::Map(Box::new(map)));
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "expected a map form".into(),
                    }));
                }
            }
        }

        Ok(attrs)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<AttrsForm> {
        let form = Form::from_tokens(tokens)?;

        AttrsForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<AttrsForm> {
        let tokens = Tokens::from_str(s)?;

        AttrsForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(attrs {} {})", self.name, self.values_to_string(),)
    }
}

impl fmt::Display for AttrsForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for AttrsForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn attrs_form_from_str() {
        use super::AttrsForm;

        let mut s = "(attrs x ())";

        let mut res = AttrsForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name.to_string(), "x".to_string());
        assert!(form.is_value_attributes());
        assert_eq!(form.values_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(attrs T x)";

        res = AttrsForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "T".to_string());
        assert!(form.is_type_attributes());
        assert_eq!(form.values_to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(attrs T moduleX.X)";

        res = AttrsForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "T".to_string());
        assert!(form.is_type_attributes());
        assert_eq!(form.values_to_string(), "moduleX.X".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(attrs x (map (pair union a) (pair moduleA.A Type)))";

        res = AttrsForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "x".to_string());
        assert!(form.is_value_attributes());
        assert_eq!(
            form.values_to_string(),
            "(map (pair union a) (pair moduleA.A Type))".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
