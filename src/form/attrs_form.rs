use crate::error::{Error, SyntacticError};
use crate::form::form::{Form, FormTailElement};
use crate::form::prod_form::{ProdForm, ProdFormValue};
use crate::form::simple_value::SimpleValue;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AttrsFormValue {
    Empty(SimpleValue),
    ValueKeyword(SimpleValue),
    TypeKeyword(SimpleValue),
    ValueSymbol(SimpleValue),
    TypeSymbol(SimpleValue),
    ValuePathSymbol(SimpleValue),
    TypePathSymbol(SimpleValue),
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
            AttrsFormValue::ValueKeyword(keyword) => keyword.file(),
            AttrsFormValue::TypeKeyword(keyword) => keyword.file(),
            AttrsFormValue::ValueSymbol(symbol) => symbol.file(),
            AttrsFormValue::TypeSymbol(symbol) => symbol.file(),
            AttrsFormValue::ValuePathSymbol(symbol) => symbol.file(),
            AttrsFormValue::TypePathSymbol(symbol) => symbol.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            AttrsFormValue::Empty(empty) => empty.loc(),
            AttrsFormValue::ValueKeyword(keyword) => keyword.loc(),
            AttrsFormValue::TypeKeyword(keyword) => keyword.loc(),
            AttrsFormValue::ValueSymbol(symbol) => symbol.loc(),
            AttrsFormValue::TypeSymbol(symbol) => symbol.loc(),
            AttrsFormValue::ValuePathSymbol(symbol) => symbol.loc(),
            AttrsFormValue::TypePathSymbol(symbol) => symbol.loc(),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            AttrsFormValue::Empty(_) => "()".into(),
            AttrsFormValue::ValueKeyword(keyword) => keyword.to_string(),
            AttrsFormValue::TypeKeyword(keyword) => keyword.to_string(),
            AttrsFormValue::ValueSymbol(symbol) => symbol.to_string(),
            AttrsFormValue::TypeSymbol(symbol) => symbol.to_string(),
            AttrsFormValue::ValuePathSymbol(symbol) => symbol.to_string(),
            AttrsFormValue::TypePathSymbol(symbol) => symbol.to_string(),
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
        match self.values.len() {
            1 => self.values[0].to_string(),
            x if x > 1 => format!(
                "(prod {})",
                self.values
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            _ => "()".to_string(),
        }
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
                SimpleValue::ValueKeyword(_) => {
                    attrs.values.push(AttrsFormValue::ValueKeyword(value));
                }
                SimpleValue::TypeKeyword(_) => {
                    attrs.values.push(AttrsFormValue::TypeKeyword(value));
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
                if let Ok(prod) = ProdForm::from_form(&form) {
                    for value in prod.values.iter() {
                        match value.clone() {
                            ProdFormValue::TypeKeyword(keyword) => {
                                attrs.values.push(AttrsFormValue::TypeKeyword(keyword));
                            }
                            ProdFormValue::ValueKeyword(keyword) => {
                                attrs.values.push(AttrsFormValue::ValueKeyword(keyword));
                            }
                            ProdFormValue::TypeSymbol(symbol) => {
                                attrs.values.push(AttrsFormValue::TypeSymbol(symbol));
                            }
                            ProdFormValue::ValueSymbol(symbol) => {
                                attrs.values.push(AttrsFormValue::ValueSymbol(symbol));
                            }
                            ProdFormValue::TypePathSymbol(symbol) => {
                                attrs.values.push(AttrsFormValue::TypePathSymbol(symbol));
                            }
                            ProdFormValue::ValuePathSymbol(symbol) => {
                                attrs.values.push(AttrsFormValue::ValuePathSymbol(symbol));
                            }
                            x => {
                                return Err(Error::Syntactic(SyntacticError {
                                    loc: x.loc(),
                                    desc: "unexpected value".into(),
                                }));
                            }
                        }
                    }
                } else {
                    return Err(Error::Syntactic(SyntacticError {
                        loc: form.loc(),
                        desc: "unexpected form".into(),
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

        s = "(attrs x (prod union a moduleA.A Type))";

        res = AttrsForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "x".to_string());
        assert!(form.is_value_attributes());
        assert_eq!(
            form.values_to_string(),
            "(prod union a moduleA.A Type)".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
