use crate::error::{Error, SemanticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::token::{Token, TokenKind};
use crate::typing::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct PrimValue {
    pub token: Token,
    pub typing: Type,
    pub value: String,
}

impl PrimValue {
    pub fn new() -> PrimValue {
        PrimValue::default()
    }

    pub fn file(&self) -> String {
        self.token.file()
    }

    pub fn loc(&self) -> Option<Loc> {
        self.token.loc()
    }

    pub fn from_token(token: Token) -> Result<PrimValue> {
        match token.kind {
            TokenKind::EmptyLiteral
            | TokenKind::UIntLiteral
            | TokenKind::IntLiteral
            | TokenKind::FloatLiteral
            | TokenKind::CharLiteral
            | TokenKind::StringLiteral => {
                let mut prim = PrimValue::new();

                prim.typing = match token.kind {
                    TokenKind::EmptyLiteral => Type::Empty,
                    TokenKind::UIntLiteral => Type::UInt,
                    TokenKind::IntLiteral => Type::Int,
                    TokenKind::FloatLiteral => Type::Float,
                    TokenKind::CharLiteral => Type::Char,
                    TokenKind::StringLiteral => Type::String,
                    _ => unreachable!(),
                };

                prim.value = token.chunks[0].to_string();
                prim.token = token;

                Ok(prim)
            }
            _ => Err(Error::Semantic(SemanticError {
                loc: token.loc(),
                desc: "expected a primitive value".into(),
            })),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl fmt::Display for PrimValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
