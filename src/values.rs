use crate::error::{Error, ParsingError};
use crate::result::Result;
use crate::token::TokenKind;
use crate::tokens::Tokens;
use crate::value::Value;
use std::convert;
use std::iter;
use std::ops;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Values(Vec<Value>);

impl Values {
    pub fn new() -> Self {
        Values::default()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, value: Value) {
        self.0.push(value)
    }

    pub fn from_str(s: &str) -> Result<Self> {
        let mut tokens = Tokens::from_str(s)?;

        tokens = tokens
            .into_iter()
            .filter(|token| token.kind != TokenKind::Comment && token.kind != TokenKind::DocComment)
            .collect();

        let mut values = Values::new();

        for token in tokens.into_iter() {
            match token.kind {
                TokenKind::Comment => {
                    return Err(Error::Parsing(ParsingError {
                        loc: token.chunks.unwrap()[0].loc.clone(),
                        desc: "unexpected comment token".into(),
                    }));
                }
                TokenKind::DocComment => {
                    return Err(Error::Parsing(ParsingError {
                        loc: token.chunks.unwrap()[0].loc.clone(),
                        desc: "unexpected doc comment token".into(),
                    }));
                }
                _ => values.push(Value::new()),
            }
        }

        Ok(values)
    }

    pub fn from_string(s: String) -> Result<Self> {
        Self::from_str(&s)
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

impl iter::FromIterator<Value> for Values {
    fn from_iter<I: iter::IntoIterator<Item = Value>>(iter: I) -> Self {
        let mut values = Values::new();

        for value in iter {
            values.push(value);
        }

        values
    }
}

impl convert::From<Vec<Value>> for Values {
    fn from(values: Vec<Value>) -> Self {
        Values(values)
    }
}

impl std::str::FromStr for Values {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Values::from_str(s)
    }
}

impl convert::TryFrom<String> for Values {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        Values::from_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ignore_comment_tokens() {
        use super::Values;

        let s = "# comment\n#! doc comment";

        let values = Values::from_str(s).unwrap();

        assert!(values.is_empty());
    }
}
