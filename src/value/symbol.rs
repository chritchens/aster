use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::is_type_symbol;
use crate::syntax::Keyword;
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
        Keyword::is(&self.value)
    }

    pub fn is_type(&self) -> bool {
        self.kind == SymbolKind::Type
    }

    pub fn is_value(&self) -> bool {
        self.kind == SymbolKind::Value
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
                    symbol.typing = if Keyword::is(&string_value) {
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
