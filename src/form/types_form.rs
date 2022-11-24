use crate::error::{Error, SemanticError, SyntacticError};
use crate::form::form::{Form, FormParam};
use crate::form::simple_value::SimpleValue;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypesFormParam {
    Ignore(SimpleValue),
    Empty(SimpleValue),
    Prim(SimpleValue),
    Keyword(SimpleValue),
    Symbol(SimpleValue),
    PathSymbol(SimpleValue),
    Form(Box<TypesForm>),
}

impl Default for TypesFormParam {
    fn default() -> TypesFormParam {
        TypesFormParam::Empty(SimpleValue::new())
    }
}

impl TypesFormParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            TypesFormParam::Ignore(_) => "_".into(),
            TypesFormParam::Empty(_) => "Empty".into(),
            TypesFormParam::Prim(_) => "Prim".into(),
            TypesFormParam::Keyword(keyword) => keyword.to_string(),
            TypesFormParam::Symbol(symbol) => symbol.to_string(),
            TypesFormParam::PathSymbol(symbol) => symbol.to_string(),
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
    pub name: SimpleValue,
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

    pub fn check_linearly_ordered_on_params(&self, params: &mut Vec<String>) -> Result<()> {
        let bound_variables = self
            .params
            .iter()
            .map(|p| p.to_string())
            .filter(|v| params.iter().any(|p| p == v))
            .collect::<Vec<String>>();

        if params != &bound_variables {
            if bound_variables.len() != params.len() {
                return Err(Error::Semantic(SemanticError {
                    loc: self.loc(),
                    desc: format!(
                        "non-linear use of params {}: {}",
                        params.join(", "),
                        bound_variables.join(" ")
                    ),
                }));
            } else {
                return Err(Error::Semantic(SemanticError {
                    loc: self.loc(),
                    desc: format!(
                        "non-ordered use of params {}: {}",
                        params.join(", "),
                        bound_variables.join(" ")
                    ),
                }));
            }
        }

        params.clear();

        Ok(())
    }

    pub fn as_form(&self) -> Form {
        let mut form = Form::new();
        form.tokens = self.tokens.clone();
        form.name = self.name.clone();

        for param in self.params.iter() {
            match param.clone() {
                TypesFormParam::Ignore(ignore) => {
                    form.params.push(FormParam::Simple(ignore));
                }
                TypesFormParam::Empty(empty) => {
                    form.params.push(FormParam::Simple(empty));
                }
                TypesFormParam::Prim(prim) => {
                    form.params.push(FormParam::Simple(prim));
                }
                TypesFormParam::Keyword(keyword) => {
                    form.params.push(FormParam::Simple(keyword));
                }
                TypesFormParam::Symbol(symbol) => {
                    form.params.push(FormParam::Simple(symbol));
                }
                TypesFormParam::PathSymbol(symbol) => {
                    form.params.push(FormParam::Simple(symbol));
                }
                TypesFormParam::Form(types_form) => {
                    form.params
                        .push(FormParam::Form(Box::new(types_form.as_form())));
                }
            }
        }

        form
    }

    pub fn from_form(form: &Form) -> Result<TypesForm> {
        let mut types_form = TypesForm::new();
        types_form.tokens = form.tokens.clone();

        let name = form.name.clone();

        match name {
            SimpleValue::TypeKeyword(_) => {
                types_form.name = name;
            }
            SimpleValue::TypeSymbol(_) => {
                types_form.name = name;
            }
            SimpleValue::TypePathSymbol(_) => {
                types_form.name = name;
            }
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "expected a type keyword, a type symbol or a type path symbol".into(),
                }));
            }
        }

        for param in form.params.iter() {
            match param.clone() {
                FormParam::Simple(value) => match value.clone() {
                    SimpleValue::TypeKeyword(keyword) => {
                        if keyword.to_string() == "Empty" {
                            types_form.params.push(TypesFormParam::Empty(value));
                        } else if keyword.to_string() == "Prim" {
                            types_form.params.push(TypesFormParam::Prim(value));
                        } else {
                            types_form.params.push(TypesFormParam::Keyword(value));
                        }
                    }
                    SimpleValue::TypeSymbol(_) => {
                        types_form.params.push(TypesFormParam::Symbol(value));
                    }
                    SimpleValue::TypePathSymbol(_) => {
                        types_form.params.push(TypesFormParam::PathSymbol(value));
                    }
                    x => {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: format!("unexpected type value: {}", x.to_string()),
                        }));
                    }
                },
                FormParam::Form(form) => {
                    if form.is_types_form() {
                        let inner_types_form = TypesForm::from_form(&form)?;
                        types_form
                            .params
                            .push(TypesFormParam::Form(Box::new(inner_types_form)));
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "expected a form of types".into(),
                        }));
                    }
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

        assert_eq!(form.name.to_string(), "Fun".to_string());
        assert_eq!(form.params_to_string(), "Empty Empty");
        assert_eq!(form.to_string(), s.to_string());

        s = "(Prod (Fun A B) Char C)";

        res = TypesForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name.to_string(), "Prod".to_string());
        assert_eq!(form.params_to_string(), "(Fun A B) Char C");
        assert_eq!(form.to_string(), s.to_string());
    }
}
