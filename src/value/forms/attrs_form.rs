use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use crate::value::forms::form::{Form, FormParam};
use crate::value::forms::prod_form::{ProdForm, ProdFormValue};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AttrsFormValue {
    Empty,
    ValueKeyword(String),
    TypeKeyword(String),
    ValueSymbol(String),
    TypeSymbol(String),
}

impl Default for AttrsFormValue {
    fn default() -> AttrsFormValue {
        AttrsFormValue::Empty
    }
}

impl AttrsFormValue {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            AttrsFormValue::Empty => "()".into(),
            AttrsFormValue::ValueKeyword(keyword) => keyword.clone(),
            AttrsFormValue::TypeKeyword(keyword) => keyword.clone(),
            AttrsFormValue::ValueSymbol(symbol) => symbol.clone(),
            AttrsFormValue::TypeSymbol(symbol) => symbol.clone(),
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
    pub tokens: Tokens,
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

    pub fn values_to_string(&self) -> String {
        let len = self.values.len();

        if len > 1 {
            format!(
                "(prod {})",
                self.values
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        } else if len == 1 {
            self.values[0].to_string()
        } else {
            "()".to_string()
        }
    }

    pub fn from_form(form: &Form) -> Result<AttrsForm> {
        if form.name != "attrs" {
            return Err(Error::Semantic(SemanticError {
                loc: form.loc(),
                desc: "expected a attrs keyword".into(),
            }));
        }

        if form.params.len() != 1 {
            return Err(Error::Semantic(SemanticError {
                loc: form.loc(),
                desc: "expected a keyword or symbol or a product of keywords or products".into(),
            }));
        }

        let mut attrs = AttrsForm::new();
        attrs.tokens = form.tokens.clone();

        match form.params[0].clone() {
            FormParam::Empty => {}
            FormParam::ValueKeyword(keyword) => {
                attrs.values.push(AttrsFormValue::ValueKeyword(keyword));
            }
            FormParam::TypeKeyword(keyword) => {
                attrs.values.push(AttrsFormValue::TypeKeyword(keyword));
            }
            FormParam::ValueSymbol(symbol) => {
                attrs.values.push(AttrsFormValue::ValueSymbol(symbol));
            }
            FormParam::TypeSymbol(symbol) => {
                attrs.values.push(AttrsFormValue::TypeSymbol(symbol));
            }
            FormParam::Form(form) => {
                if let Ok(prod) = ProdForm::from_form(&form) {
                    for value in prod.values.iter() {
                        match value {
                            ProdFormValue::TypeKeyword(keyword) => {
                                attrs
                                    .values
                                    .push(AttrsFormValue::TypeKeyword(keyword.clone()));
                            }
                            ProdFormValue::ValueKeyword(keyword) => {
                                attrs
                                    .values
                                    .push(AttrsFormValue::ValueKeyword(keyword.clone()));
                            }
                            ProdFormValue::TypeSymbol(symbol) => {
                                attrs
                                    .values
                                    .push(AttrsFormValue::TypeSymbol(symbol.clone()));
                            }
                            ProdFormValue::ValueSymbol(symbol) => {
                                attrs
                                    .values
                                    .push(AttrsFormValue::ValueSymbol(symbol.clone()));
                            }
                            _ => {
                                return Err(Error::Semantic(SemanticError {
                                    loc: form.loc(),
                                    desc: "expected a product of symbols".into(),
                                }));
                            }
                        }
                    }
                } else {
                    return Err(Error::Semantic(SemanticError {
                        loc: form.loc(),
                        desc: "expected a product of symbols".into(),
                    }));
                }
            }
            x => {
                return Err(Error::Semantic(SemanticError {
                    loc: form.loc(),
                    desc: format!("unexpected attrsction params: {}", x.to_string()),
                }));
            }
        }

        Ok(attrs)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<AttrsForm> {
        let form = Form::from_tokens(tokens)?;

        AttrsForm::from_form(&form)
    }

    pub fn from_str(s: &str) -> Result<AttrsForm> {
        let tokens = Tokens::from_str(s)?;

        AttrsForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("(attrs {})", self.values_to_string(),)
    }
}

impl fmt::Display for AttrsForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn attrs_form_from_str() {
        use super::AttrsForm;
        use super::AttrsFormValue;

        let mut s = "(attrs ())";

        let mut res = AttrsForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert!(form.values.is_empty());
        assert_eq!(form.values_to_string(), "()".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(attrs x)";

        res = AttrsForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.values,
            vec![AttrsFormValue::ValueSymbol("x".to_string())]
        );
        assert_eq!(form.values_to_string(), "x".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(attrs moduleX.X)";

        res = AttrsForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.values,
            vec![AttrsFormValue::TypeSymbol("moduleX.X".to_string())]
        );
        assert_eq!(form.values_to_string(), "moduleX.X".to_string());
        assert_eq!(form.to_string(), s.to_string());

        s = "(attrs (prod type a moduleA.A Type))";

        res = AttrsForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(
            form.values_to_string(),
            "(prod type a moduleA.A Type)".to_string()
        );
        assert_eq!(form.to_string(), s.to_string());
    }
}
