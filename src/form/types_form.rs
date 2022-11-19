use crate::error::{Error, SyntacticError};
use crate::form::form::{Form, FormParam};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::is_type_keyword;
use crate::syntax::{is_type_symbol, symbol_name};
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypesFormParam {
    Empty,
    Keyword(String),
    Symbol(String),
    Form(Box<Form>),
}

impl Default for TypesFormParam {
    fn default() -> TypesFormParam {
        TypesFormParam::Empty
    }
}

impl TypesFormParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            TypesFormParam::Empty => "Empty".into(),
            TypesFormParam::Keyword(keyword) => keyword.clone(),
            TypesFormParam::Symbol(symbol) => symbol.clone(),
            TypesFormParam::Form(form) => form.to_string(),
        }
    }
}

impl fmt::Display for TypesFormParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct TypesForm {
    pub tokens: Box<Tokens>,
    pub name: String,
    pub params: Vec<TypesFormParam>,
}

impl TypesForm {
    pub fn new() -> TypesForm {
        TypesForm::default()
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
                TypesFormParam::Empty => {
                    form.params.push(FormParam::Empty);
                }
                TypesFormParam::Keyword(keyword) => {
                    form.params.push(FormParam::TypeKeyword(keyword));
                }
                TypesFormParam::Symbol(symbol) => {
                    form.params.push(FormParam::TypeSymbol(symbol));
                }
                TypesFormParam::Form(types_form) => {
                    form.params.push(FormParam::Form(types_form));
                }
            }
        }

        form
    }

    pub fn from_form(form: &Form) -> Result<TypesForm> {
        if !is_type_symbol(&symbol_name(&form.name)) && !is_type_keyword(&form.name) {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a type".into(),
            }));
        }

        let mut types_form = TypesForm::new();
        types_form.tokens = form.tokens.clone();
        types_form.name = form.name.clone();

        for param in form.params.iter() {
            match param.clone() {
                FormParam::TypeKeyword(keyword) => {
                    if keyword == "Empty" {
                        types_form.params.push(TypesFormParam::Empty);
                    } else {
                        types_form.params.push(TypesFormParam::Keyword(keyword));
                    }
                }
                FormParam::TypeSymbol(symbol) => {
                    types_form.params.push(TypesFormParam::Symbol(symbol));
                }
                FormParam::Form(form) => {
                    if form.is_types_form() {
                        types_form.params.push(TypesFormParam::Form(form));
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

        Ok(types_form)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<TypesForm> {
        let form = Form::from_tokens(tokens)?;

        TypesForm::from_form(&form)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<TypesForm> {
        let tokens = Tokens::from_str(s)?;

        TypesForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("({} {})", self.name, self.params_to_string(),)
    }
}

impl fmt::Display for TypesForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for TypesForm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn types_form_from_str() {
        use super::TypesForm;

        let mut s = "(Fun Empty Empty)";

        let mut res = TypesForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "Fun".to_string());
        assert_eq!(form.params_to_string(), "Empty Empty");
        assert_eq!(form.to_string(), s.to_string());

        s = "(Prod (Fun A B) Char C)";

        res = TypesForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "Prod".to_string());
        assert_eq!(form.params_to_string(), "(Fun A B) Char C");
        assert_eq!(form.to_string(), s.to_string());
    }
}
