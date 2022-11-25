use crate::error::{Error, SemanticError, SyntacticError};
use crate::form::form::{Form, FormTailElement};
use crate::form::simple_value::SimpleValue;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::Tokens;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypesFormTailElement {
    Ignore(SimpleValue),
    Empty(SimpleValue),
    Prim(SimpleValue),
    Keyword(SimpleValue),
    Symbol(SimpleValue),
    PathSymbol(SimpleValue),
    Form(Box<TypesForm>),
}

impl Default for TypesFormTailElement {
    fn default() -> TypesFormTailElement {
        TypesFormTailElement::Empty(SimpleValue::new())
    }
}

impl TypesFormTailElement {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            TypesFormTailElement::Ignore(_) => "_".into(),
            TypesFormTailElement::Empty(_) => "Empty".into(),
            TypesFormTailElement::Prim(_) => "Prim".into(),
            TypesFormTailElement::Keyword(keyword) => keyword.to_string(),
            TypesFormTailElement::Symbol(symbol) => symbol.to_string(),
            TypesFormTailElement::PathSymbol(symbol) => symbol.to_string(),
            TypesFormTailElement::Form(form) => form.to_string(),
        }
    }
}

impl fmt::Display for TypesFormTailElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct TypesForm {
    pub tokens: Box<Tokens>,
    pub head: SimpleValue,
    pub tail: Vec<TypesFormTailElement>,
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

    pub fn tail_to_string(&self) -> String {
        self.tail
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

    pub fn check_linearly_ordered_on_parameters(&self, parameters: &mut Vec<String>) -> Result<()> {
        let bound_variables = self
            .tail
            .iter()
            .map(|p| p.to_string())
            .filter(|v| parameters.iter().any(|p| p == v))
            .collect::<Vec<String>>();

        if parameters != &bound_variables {
            if bound_variables.len() != parameters.len() {
                return Err(Error::Semantic(SemanticError {
                    loc: self.loc(),
                    desc: format!(
                        "non-linear use of parameters {}: {}",
                        parameters.join(", "),
                        bound_variables.join(" ")
                    ),
                }));
            } else {
                return Err(Error::Semantic(SemanticError {
                    loc: self.loc(),
                    desc: format!(
                        "non-ordered use of parameters {}: {}",
                        parameters.join(", "),
                        bound_variables.join(" ")
                    ),
                }));
            }
        }

        parameters.clear();

        Ok(())
    }

    pub fn as_form(&self) -> Form {
        let mut form = Form::new();
        form.tokens = self.tokens.clone();
        form.head = self.head.clone();

        for param in self.tail.iter() {
            match param.clone() {
                TypesFormTailElement::Ignore(ignore) => {
                    form.tail.push(FormTailElement::Simple(ignore));
                }
                TypesFormTailElement::Empty(empty) => {
                    form.tail.push(FormTailElement::Simple(empty));
                }
                TypesFormTailElement::Prim(prim) => {
                    form.tail.push(FormTailElement::Simple(prim));
                }
                TypesFormTailElement::Keyword(keyword) => {
                    form.tail.push(FormTailElement::Simple(keyword));
                }
                TypesFormTailElement::Symbol(symbol) => {
                    form.tail.push(FormTailElement::Simple(symbol));
                }
                TypesFormTailElement::PathSymbol(symbol) => {
                    form.tail.push(FormTailElement::Simple(symbol));
                }
                TypesFormTailElement::Form(types_form) => {
                    form.tail
                        .push(FormTailElement::Form(Box::new(types_form.as_form())));
                }
            }
        }

        form
    }

    pub fn from_form(form: &Form) -> Result<TypesForm> {
        let mut types_form = TypesForm::new();
        types_form.tokens = form.tokens.clone();

        let name = form.head.clone();

        match name {
            SimpleValue::TypeKeyword(_) => {
                types_form.head = name;
            }
            SimpleValue::TypeSymbol(_) => {
                types_form.head = name;
            }
            SimpleValue::TypePathSymbol(_) => {
                types_form.head = name;
            }
            _ => {
                return Err(Error::Syntactic(SyntacticError {
                    loc: form.loc(),
                    desc: "expected a type keyword, a type symbol or a type path symbol".into(),
                }));
            }
        }

        for param in form.tail.iter() {
            match param.clone() {
                FormTailElement::Simple(value) => match value.clone() {
                    SimpleValue::TypeKeyword(keyword) => {
                        if keyword.to_string() == "Empty" {
                            types_form.tail.push(TypesFormTailElement::Empty(value));
                        } else if keyword.to_string() == "Prim" {
                            types_form.tail.push(TypesFormTailElement::Prim(value));
                        } else {
                            types_form.tail.push(TypesFormTailElement::Keyword(value));
                        }
                    }
                    SimpleValue::TypeSymbol(_) => {
                        types_form.tail.push(TypesFormTailElement::Symbol(value));
                    }
                    SimpleValue::TypePathSymbol(_) => {
                        types_form
                            .tail
                            .push(TypesFormTailElement::PathSymbol(value));
                    }
                    x => {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: format!("unexpected type value: {}", x.to_string()),
                        }));
                    }
                },
                FormTailElement::Form(form) => {
                    if form.is_types_form() {
                        let inner_types_form = TypesForm::from_form(&form)?;
                        types_form
                            .tail
                            .push(TypesFormTailElement::Form(Box::new(inner_types_form)));
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
        format!("({} {})", self.head, self.tail_to_string(),)
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

        assert_eq!(form.head.to_string(), "Fun".to_string());
        assert_eq!(form.tail_to_string(), "Empty Empty");
        assert_eq!(form.to_string(), s.to_string());

        s = "(Prod (Fun A B) Char C)";

        res = TypesForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.head.to_string(), "Prod".to_string());
        assert_eq!(form.tail_to_string(), "(Fun A B) Char C");
        assert_eq!(form.to_string(), s.to_string());
    }
}
