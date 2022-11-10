use crate::error::{Error, SyntacticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_type_symbol, symbol_name};
use crate::token::Tokens;
use crate::value::forms::form::{Form, FormParam};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypeFormParam {
    Empty,
    Keyword(String),
    Symbol(String),
    Form(Box<Form>),
}

impl Default for TypeFormParam {
    fn default() -> TypeFormParam {
        TypeFormParam::Empty
    }
}

impl TypeFormParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            TypeFormParam::Empty => "Empty".into(),
            TypeFormParam::Keyword(keyword) => keyword.clone(),
            TypeFormParam::Symbol(symbol) => symbol.clone(),
            TypeFormParam::Form(form) => form.to_string(),
        }
    }
}

impl fmt::Display for TypeFormParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct TypeForm {
    pub tokens: Box<Tokens>,
    pub name: String,
    pub params: Vec<TypeFormParam>,
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

    pub fn params_to_string(&self) -> String {
        self.params
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn as_form(&self) -> Form {
        let mut form = Form::new();
        form.tokens = self.tokens.clone();
        form.name = self.name.clone();

        for param in self.params.iter() {
            match param.clone() {
                TypeFormParam::Empty => {
                    form.params.push(FormParam::Empty);
                }
                TypeFormParam::Keyword(keyword) => {
                    form.params.push(FormParam::TypeKeyword(keyword));
                }
                TypeFormParam::Symbol(symbol) => {
                    form.params.push(FormParam::TypeSymbol(symbol));
                }
                TypeFormParam::Form(type_form) => {
                    form.params.push(FormParam::Form(type_form));
                }
            }
        }

        form
    }

    pub fn from_form(form: &Form) -> Result<TypeForm> {
        if !is_type_symbol(&symbol_name(&form.name)) {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a type".into(),
            }));
        }

        let mut type_form = TypeForm::new();
        type_form.tokens = form.tokens.clone();
        type_form.name = form.name.clone();

        for param in form.params.iter() {
            match param.clone() {
                FormParam::TypeKeyword(keyword) => {
                    if keyword == "Empty" {
                        type_form.params.push(TypeFormParam::Empty);
                    } else {
                        type_form.params.push(TypeFormParam::Keyword(keyword));
                    }
                }
                FormParam::TypeSymbol(symbol) => {
                    type_form.params.push(TypeFormParam::Symbol(symbol));
                }
                FormParam::Form(form) => {
                    if form.is_type_form() {
                        type_form.params.push(TypeFormParam::Form(form));
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
                        desc: format!("unexpected type value: {}", x.to_string()),
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

    pub fn from_str(s: &str) -> Result<TypeForm> {
        let tokens = Tokens::from_str(s)?;

        TypeForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("({} {})", self.name, self.params_to_string(),)
    }
}

impl fmt::Display for TypeForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn type_form_from_str() {
        use super::TypeForm;

        let mut s = "(Fun Empty Empty)";

        let mut res = TypeForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "Fun".to_string());
        assert_eq!(form.params_to_string(), "Empty Empty");
        assert_eq!(form.to_string(), s.to_string());

        s = "(Prod (Fun A B) Char C)";

        res = TypeForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "Prod".to_string());
        assert_eq!(form.params_to_string(), "(Fun A B) Char C");
        assert_eq!(form.to_string(), s.to_string());
    }
}
