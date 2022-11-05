use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_keyword, is_type_symbol};
use crate::token::{Token, TokenKind};
use crate::typing::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SymbolKind {
    Type,
    Value,
}

impl Default for SymbolKind {
    fn default() -> SymbolKind {
        SymbolKind::Type
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct SymbolValue {
    pub token: Token,
    pub kind: SymbolKind,
    pub typing: Type,
    pub value: String,
}

impl SymbolValue {
    pub fn new() -> SymbolValue {
        SymbolValue::default()
    }

    pub fn file(&self) -> String {
        self.token.file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.token.loc()
    }

    pub fn is_keyword(&self) -> bool {
        is_keyword(&self.value)
    }

    pub fn is_type(&self) -> bool {
        self.kind == SymbolKind::Type
    }

    pub fn is_value(&self) -> bool {
        self.kind == SymbolKind::Value
    }

    pub fn validate(&self) -> Result<()> {
        let expected = SymbolValue::from_token(self.token.clone())?;

        if self != &expected {
            Err(Error::Semantic(SemanticError {
                loc: self.token.loc(),
                desc: "expected symbol to represent its token".into(),
            }))
        } else {
            Ok(())
        }
    }

    pub fn from_token(token: Token) -> Result<SymbolValue> {
        match token.kind {
            TokenKind::Keyword
            | TokenKind::ValueSymbol
            | TokenKind::TypeSymbol
            | TokenKind::PathSymbol => {
                let string_value = token.chunks[0].to_string();
                let mut symbol = SymbolValue::new();

                if is_type_symbol(&string_value) {
                    symbol.kind = SymbolKind::Type;
                    symbol.typing = Type::Type;
                } else {
                    symbol.kind = SymbolKind::Value;
                    symbol.typing = if is_keyword(&string_value) {
                        Type::Builtin
                    } else {
                        Type::Unknown(string_value.clone())
                    };
                }

                symbol.value = string_value;
                symbol.token = token;

                Ok(symbol)
            }
            _ => Err(Error::Semantic(SemanticError {
                loc: token.loc(),
                desc: "expected a symbol".into(),
            })),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl fmt::Display for SymbolValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn symbol_value_validate() {
        use super::SymbolValue;
        use crate::token::Tokens;

        let s = "a";
        let s1 = "A";

        let tokens = Tokens::from_str(s).unwrap();
        let token = tokens[0].clone();

        let tokens1 = Tokens::from_str(s1).unwrap();
        let token1 = tokens1[0].clone();

        let mut symbol = SymbolValue::from_token(token).unwrap();

        assert!(symbol.validate().is_ok());

        symbol.token = token1;

        assert!(symbol.validate().is_err());
    }
}
