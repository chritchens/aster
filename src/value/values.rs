use crate::error::{Error, SyntacticError};
use crate::result::Result;
use crate::token::{TokenKind, Tokens};
use crate::value::{FormKind, FormValue};
use crate::value::{PrimValue, SymbolValue, Value};
use std::fmt;
use std::fs;
use std::iter;
use std::ops;
use std::path::Path;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Values(Vec<Value>);

impl Values {
    pub fn new() -> Values {
        Values::default()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, value: Value) {
        self.0.push(value);
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn from_tokens(tokens: &Tokens) -> Result<Values> {
        let mut form = FormValue::new();
        let mut form_count = 0;
        let mut values = Values::new();

        let len = tokens.len();
        let mut idx = 0;

        while idx < len {
            let token = tokens[idx].clone();

            match token.kind {
                TokenKind::Comment | TokenKind::DocComment => {
                    idx += 1;
                }
                TokenKind::Keyword => {
                    let symbol = SymbolValue::from_token(token)?;

                    let value = Value::Symbol(symbol.clone());

                    form.typing.push(symbol.typing.clone());
                    form.values.push(value);

                    idx += 1;
                }
                TokenKind::EmptyLiteral
                | TokenKind::UIntLiteral
                | TokenKind::IntLiteral
                | TokenKind::FloatLiteral
                | TokenKind::CharLiteral
                | TokenKind::StringLiteral => {
                    let prim = PrimValue::from_token(token)?;

                    let value = Value::Prim(prim.clone());

                    if form_count == 1 {
                        form.typing.push(prim.typing.clone());
                        form.values.push(value);
                    } else {
                        let form_len = form.len() - 1;

                        match form.values[form_len - 1].clone() {
                            Value::Form(mut inner_form) => {
                                inner_form.typing.push(prim.typing.clone());
                                inner_form.values.push(value);
                                form.values[form_len - 1] = Value::Form(inner_form);
                            }
                            x => {
                                return Err(Error::Syntactic(SyntacticError {
                                    loc: x.loc(),
                                    desc: format!("expected {} to be a form", x.to_string()),
                                }));
                            }
                        }
                    }

                    idx += 1;
                }
                TokenKind::ValueSymbol | TokenKind::TypeSymbol | TokenKind::PathSymbol => {
                    let symbol = SymbolValue::from_token(token)?;

                    let value = Value::Symbol(symbol.clone());

                    form.typing.push(symbol.typing.clone());
                    form.values.push(value);

                    idx += 1;
                }
                TokenKind::FormStart => {
                    form_count += 1;

                    if form_count > 1 {
                        let mut new_count = 0;
                        let mut new_tokens = Tokens::new();

                        while idx < len {
                            let new_token = tokens[idx].clone();

                            if new_token.kind == TokenKind::FormStart {
                                new_count += 1;
                            } else if new_token.kind == TokenKind::FormEnd {
                                new_count -= 1;
                            }

                            new_tokens.push(new_token);

                            idx += 1;

                            if new_count == 0 {
                                break;
                            }
                        }

                        let new_form = Values::from_tokens(&new_tokens)?[0].clone();

                        form.values.push(new_form);

                        form_count -= 1;
                    } else {
                        idx += 1;
                    }
                }
                TokenKind::FormEnd => {
                    form_count -= 1;

                    if form_count == 0 {
                        form.kind = FormKind::from_form(&form)?;
                        let value = Value::Form(form.clone());
                        values.push(value);

                        form = FormValue::new();
                    }

                    idx += 1;
                }
            }
        }

        Ok(values)
    }

    pub fn from_str(s: &str) -> Result<Values> {
        let tokens = Tokens::from_str(s)?;

        Values::from_tokens(&tokens)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Values> {
        Values::from_str(&fs::read_to_string(path)?)
    }
}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ops::Index<usize> for Values {
    type Output = Value;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl iter::IntoIterator for Values {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn values_from_str() {
        use super::Values;
        use crate::value::{FormKind, Value};

        let s = "(import moduleX x (prod a b c D))";

        let res = Values::from_str(s);

        assert!(res.is_ok());

        let values = res.unwrap();

        assert_eq!(values.len(), 1);

        match &values[0] {
            Value::Form(form) => {
                assert_eq!(form.len(), 4);
                assert_eq!(form.kind, FormKind::ImportDefs);

                match &form.values[3] {
                    Value::Form(form) => {
                        assert_eq!(form.len(), 5);
                        assert_eq!(form.kind, FormKind::AnonProd);
                    }
                    _ => panic!("expected a form"),
                }
            }
            _ => panic!("expected a form"),
        }
    }

    #[test]
    fn values_from_file() {
        use super::Values;
        use crate::value::{FormKind, Value};
        use std::path::Path;

        let path = Path::new("./examples/sum.sp");

        let res = Values::from_file(path);

        assert!(res.is_ok());

        let values = res.unwrap();

        assert_eq!(values.len(), 5);

        match &values[2] {
            Value::Form(form) => {
                assert_eq!(form.len(), 4);
                assert_eq!(form.kind, FormKind::DefSig);

                match &form.values[3] {
                    Value::Form(form) => {
                        assert_eq!(form.len(), 3);
                        assert_eq!(form.kind, FormKind::TypeApp);
                    }
                    _ => panic!("expected a form"),
                }
            }
            _ => panic!("expected a form"),
        }
    }
}
