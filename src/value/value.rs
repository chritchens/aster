use super::{FormParam, FormValue};
use super::{PrimValue, SymbolValue};
use crate::loc::Loc;
use crate::result::Result;
use crate::typing::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
    Prim(PrimValue),
    Symbol(SymbolValue),
    Form(FormValue),
}

impl Default for Value {
    fn default() -> Self {
        Value::Prim(PrimValue::default())
    }
}

impl Value {
    pub fn new() -> Value {
        Value::default()
    }

    pub fn file(&self) -> String {
        match self {
            Value::Prim(value) => value.file(),
            Value::Symbol(value) => value.file(),
            Value::Form(value) => value.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            Value::Prim(value) => value.loc(),
            Value::Symbol(value) => value.loc(),
            Value::Form(value) => value.loc(),
        }
    }

    pub fn typing(&self) -> Vec<Type> {
        match self {
            Value::Prim(value) => vec![value.typing.clone()],
            Value::Symbol(value) => vec![value.typing.clone()],
            Value::Form(value) => value.typing.clone(),
        }
    }

    pub fn head_to_string(&self) -> String {
        match self {
            Value::Prim(value) => value.value.clone(),
            Value::Symbol(value) => value.value.clone(),
            Value::Form(value) => value.head().to_string(),
        }
    }

    pub fn params(&self) -> Vec<FormParam> {
        match self {
            Value::Form(value) => value.params(),
            _ => vec![],
        }
    }

    pub fn body(&self) -> Result<Option<Value>> {
        match self {
            Value::Form(value) => value.body(),
            _ => Ok(None),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            Value::Prim(value) => value.to_string(),
            Value::Symbol(value) => value.to_string(),
            Value::Form(values) => values.to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn value_params_and_body() {
        use crate::value::Values;

        let mut s = "(def fun f a b (+ a b))";

        let mut values = Values::from_str(s).unwrap();

        let mut value = values[0].clone();

        let mut params = value.params();

        assert_eq!(params.len(), 2);
        assert_eq!(params[0].to_string(), "a");
        assert_eq!(params[1].to_string(), "b");

        if let Some(body) = value.body().unwrap() {
            assert_eq!(body.to_string(), "(+ a b)");
        } else {
            panic!("expected a function body");
        }

        s = "(def f (fun f a b (+ a b)))";

        values = Values::from_str(s).unwrap();

        value = values[0].clone();

        params = value.params();

        assert_eq!(params.len(), 2);
        assert_eq!(params[0].to_string(), "a");
        assert_eq!(params[1].to_string(), "b");

        if let Some(body) = value.body().unwrap() {
            assert_eq!(body.to_string(), "(+ a b)");
        } else {
            panic!("expected a function body");
        }

        s = "(def type F A B (Fun A B))";

        values = Values::from_str(s).unwrap();

        value = values[0].clone();

        params = value.params();

        assert_eq!(params.len(), 2);
        assert_eq!(params[0].to_string(), "A");
        assert_eq!(params[1].to_string(), "B");

        if let Some(body) = value.body().unwrap() {
            assert_eq!(body.to_string(), "(Fun A B)");
        } else {
            panic!("expected a type body");
        }

        s = "(def F (type A B (Fun A B)))";

        values = Values::from_str(s).unwrap();

        value = values[0].clone();

        params = value.params();

        assert_eq!(params.len(), 2);
        assert_eq!(params[0].to_string(), "A");
        assert_eq!(params[1].to_string(), "B");

        if let Some(body) = value.body().unwrap() {
            assert_eq!(body.to_string(), "(Fun A B)");
        } else {
            panic!("expected a type body");
        }

        s = "(f a b c 10)";

        values = Values::from_str(s).unwrap();

        value = values[0].clone();

        params = value.params();

        assert_eq!(params.len(), 4);
        assert_eq!(params[0].to_string(), "a");
        assert_eq!(params[1].to_string(), "b");
        assert_eq!(params[2].to_string(), "c");
        assert_eq!(params[3].to_string(), "10");

        assert!(value.body().unwrap().is_none());

        s = "(app f a b c 10)";

        values = Values::from_str(s).unwrap();

        value = values[0].clone();

        params = value.params();

        assert_eq!(params.len(), 4);
        assert_eq!(params[0].to_string(), "a");
        assert_eq!(params[1].to_string(), "b");
        assert_eq!(params[2].to_string(), "c");
        assert_eq!(params[3].to_string(), "10");

        assert!(value.body().unwrap().is_none());

        s = "(Fun A B)";

        values = Values::from_str(s).unwrap();

        value = values[0].clone();

        params = value.params();

        assert_eq!(params.len(), 2);
        assert_eq!(params[0].to_string(), "A");
        assert_eq!(params[1].to_string(), "B");

        assert!(value.body().unwrap().is_none());

        s = "(type (Fun A B))";

        values = Values::from_str(s).unwrap();

        value = values[0].clone();

        params = value.params();

        assert_eq!(params.len(), 0);

        if let Some(body) = value.body().unwrap() {
            assert_eq!(body.to_string(), "(Fun A B)");
        } else {
            panic!("expected a type body");
        }
    }
}
