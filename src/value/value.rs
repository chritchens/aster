use crate::error::Error;
use crate::form::Form;
use crate::loc::Loc;
use crate::result::Result;
use crate::token::{Token, Tokens};
use crate::value::{FormValue, SimpleValue};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
    Simple(SimpleValue),
    Form(Box<FormValue>),
}

impl Default for Value {
    fn default() -> Value {
        Value::Simple(SimpleValue::new())
    }
}

impl Value {
    pub fn new() -> Value {
        Value::default()
    }

    pub fn file(&self) -> String {
        match self {
            Value::Simple(value) => value.file(),
            Value::Form(form) => form.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            Value::Simple(value) => value.loc(),
            Value::Form(form) => form.loc(),
        }
    }

    pub fn is_simple(&self) -> bool {
        match self {
            Value::Simple(_) => true,
            _ => false,
        }
    }

    pub fn is_form(&self) -> bool {
        match self {
            Value::Form(_) => true,
            _ => false,
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            Value::Simple(value) => value.to_string(),
            Value::Form(form) => form.to_string(),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Value> {
        let tokens = Tokens::from_str(s)?;

        Value::from_tokens(&tokens)
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<Value> {
        let value = if tokens.len() == 1 {
            Value::from_token(&tokens[0])?
        } else {
            let form = Form::from_tokens(tokens)?;

            Value::from_form(&form)?
        };

        Ok(value)
    }

    pub fn from_token(token: &Token) -> Result<Value> {
        let simple_value = SimpleValue::from_token(token)?;

        Ok(Value::Simple(simple_value))
    }

    pub fn from_form(form: &Form) -> Result<Value> {
        let form_value = FormValue::from_form(form)?;

        Ok(Value::Form(Box::new(form_value)))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn value_from_str() {
        use super::Value;

        let mut s = "()";

        let mut res = Value::from_str(s);

        assert!(res.is_ok());

        let value = res.unwrap();

        assert!(value.is_simple());

        s = "
        (block
            (import std.io _ printf)
            (export printChar)
            
            (sig printChar (Fun IO Char IO))
            (val printChar (fun io c 
                (printf io \"char: {}\n\" c))))";

        res = Value::from_str(s);

        assert!(res.is_ok());

        let value = res.unwrap();

        assert!(value.is_form());

        s = "
        (block
            (import std.io _ printf)
            (export printProd)

            (sig printProd (Fun IO (Prod Int Char) IO))
            (val printProd (fun io (prod a b)
                (printf io \"a: {}, b: {}\n\" (prod (+ a 100) b)))))";

        res = Value::from_str(s);

        assert!(res.is_ok());

        let value = res.unwrap();

        assert!(value.is_form());
    }
}
