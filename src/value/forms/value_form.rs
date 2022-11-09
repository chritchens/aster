use crate::error::{Error, SyntacticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_value_symbol, symbol_name};
use crate::token::Tokens;
use crate::value::forms::form::{Form, FormParam};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ValueFormParam {
    Empty,
    Prim(String),
    Keyword(String),
    Symbol(String),
    Form(Form),
}

impl Default for ValueFormParam {
    fn default() -> ValueFormParam {
        ValueFormParam::Empty
    }
}

impl ValueFormParam {
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            ValueFormParam::Empty => "()".into(),
            ValueFormParam::Prim(prim) => prim.clone(),
            ValueFormParam::Keyword(keyword) => keyword.clone(),
            ValueFormParam::Symbol(symbol) => symbol.clone(),
            ValueFormParam::Form(form) => form.to_string(),
        }
    }
}

impl fmt::Display for ValueFormParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct ValueForm {
    pub tokens: Tokens,
    pub name: String,
    pub params: Vec<ValueFormParam>,
}

impl ValueForm {
    pub fn new() -> ValueForm {
        ValueForm::default()
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

    pub fn from_form(form: &Form) -> Result<ValueForm> {
        if !is_value_symbol(&symbol_name(&form.name)) {
            return Err(Error::Syntactic(SyntacticError {
                loc: form.loc(),
                desc: "expected a value".into(),
            }));
        }

        let mut type_form = ValueForm::new();
        type_form.tokens = form.tokens.clone();
        type_form.name = form.name.clone();

        for param in form.params.iter() {
            match param {
                FormParam::Prim(prim) => {
                    type_form.params.push(ValueFormParam::Prim(prim.clone()));
                }
                FormParam::ValueKeyword(keyword) => {
                    if keyword.to_string() == "()" {
                        type_form.params.push(ValueFormParam::Empty);
                    } else {
                        type_form
                            .params
                            .push(ValueFormParam::Keyword(keyword.clone()));
                    }
                }
                FormParam::ValueSymbol(symbol) => {
                    type_form
                        .params
                        .push(ValueFormParam::Symbol(symbol.clone()));
                }
                FormParam::Form(form) => {
                    if form.is_value_form() {
                        type_form.params.push(ValueFormParam::Form(form.clone()));
                    } else {
                        return Err(Error::Syntactic(SyntacticError {
                            loc: form.loc(),
                            desc: "expected a form of values".into(),
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
        }

        Ok(type_form)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<ValueForm> {
        let form = Form::from_tokens(tokens)?;

        ValueForm::from_form(&form)
    }

    pub fn from_str(s: &str) -> Result<ValueForm> {
        let tokens = Tokens::from_str(s)?;

        ValueForm::from_tokens(&tokens)
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("({} {})", self.name, self.params_to_string(),)
    }
}

impl fmt::Display for ValueForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn value_form_from_str() {
        use super::ValueForm;

        let mut s = "(fun (prod a b c d) (f a b c d))";

        let mut res = ValueForm::from_str(s);

        assert!(res.is_ok());

        let mut form = res.unwrap();

        assert_eq!(form.name, "fun".to_string());
        assert_eq!(form.params_to_string(), "(prod a b c d) (f a b c d)");
        assert_eq!(form.to_string(), s.to_string());

        s = "(prod a b c (math.* a b c))";

        res = ValueForm::from_str(s);

        assert!(res.is_ok());

        form = res.unwrap();

        assert_eq!(form.name, "prod".to_string());
        assert_eq!(form.params_to_string(), "a b c (math.* a b c)");
        assert_eq!(form.to_string(), s.to_string());
    }
}
