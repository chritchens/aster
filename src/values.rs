use crate::error::{Error, ParsingError};
use crate::result::Result;
use crate::token::TokenKind;
use crate::tokens::Tokens;
use crate::value::Value;

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Values(Vec<Value>);

impl Values {
    pub fn new() -> Self {
        Values::default()
    }

    pub fn len(&self) -> usize {
        self.0.len()
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn ignore_comment_tokens() {
        use super::Values;

        let s = "# comment\n#! doc comment";

        let values = Values::from_str(s).unwrap();

        assert_eq!(values.len(), 0);
    }
}
