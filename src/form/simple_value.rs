use crate::error::{Error, SyntacticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::is_value_keyword;
use crate::token::{Token, TokenKind};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SimpleValue {
    Ignore(Token),
    Empty(Token),
    Panic(Token),
    ValueKeyword(Token),
    TypeKeyword(Token),
    Prim(Token),
    ValueSymbol(Token),
    TypeSymbol(Token),
    ValuePathSymbol(Token),
    TypePathSymbol(Token),
}

impl Default for SimpleValue {
    fn default() -> SimpleValue {
        SimpleValue::Empty(Token::new())
    }
}

impl SimpleValue {
    pub fn new() -> SimpleValue {
        SimpleValue::default()
    }

    pub fn token(&self) -> Token {
        match self {
            SimpleValue::Ignore(token) => token.clone(),
            SimpleValue::Empty(token) => token.clone(),
            SimpleValue::Panic(token) => token.clone(),
            SimpleValue::Prim(token) => token.clone(),
            SimpleValue::ValueKeyword(token) => token.clone(),
            SimpleValue::TypeKeyword(token) => token.clone(),
            SimpleValue::ValueSymbol(token) => token.clone(),
            SimpleValue::TypeSymbol(token) => token.clone(),
            SimpleValue::ValuePathSymbol(token) => token.clone(),
            SimpleValue::TypePathSymbol(token) => token.clone(),
        }
    }

    pub fn file(&self) -> String {
        match self {
            SimpleValue::Ignore(token) => token.file(),
            SimpleValue::Empty(token) => token.file(),
            SimpleValue::Panic(token) => token.file(),
            SimpleValue::ValueKeyword(token) => token.file(),
            SimpleValue::TypeKeyword(token) => token.file(),
            SimpleValue::Prim(token) => token.file(),
            SimpleValue::ValueSymbol(token) => token.file(),
            SimpleValue::TypeSymbol(token) => token.file(),
            SimpleValue::ValuePathSymbol(token) => token.file(),
            SimpleValue::TypePathSymbol(token) => token.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            SimpleValue::Ignore(token) => token.loc(),
            SimpleValue::Empty(token) => token.loc(),
            SimpleValue::Panic(token) => token.loc(),
            SimpleValue::ValueKeyword(token) => token.loc(),
            SimpleValue::TypeKeyword(token) => token.loc(),
            SimpleValue::Prim(token) => token.loc(),
            SimpleValue::ValueSymbol(token) => token.loc(),
            SimpleValue::TypeSymbol(token) => token.loc(),
            SimpleValue::ValuePathSymbol(token) => token.loc(),
            SimpleValue::TypePathSymbol(token) => token.loc(),
        }
    }

    pub fn from_token(token: &Token) -> Result<SimpleValue> {
        let token = token.to_owned();

        match token.kind {
            TokenKind::Comment | TokenKind::DocComment => Err(Error::Syntactic(SyntacticError {
                loc: token.loc(),
                desc: "unexpected comment marker".into(),
            })),
            TokenKind::FormStart | TokenKind::FormEnd => Err(Error::Syntactic(SyntacticError {
                loc: token.loc(),
                desc: "unexpected form punctuation".into(),
            })),
            TokenKind::EmptyLiteral => Ok(SimpleValue::Empty(token)),
            TokenKind::UIntLiteral
            | TokenKind::IntLiteral
            | TokenKind::FloatLiteral
            | TokenKind::CharLiteral
            | TokenKind::StringLiteral => Ok(SimpleValue::Prim(token)),
            TokenKind::Keyword => match token.to_string().as_str() {
                "_" => Ok(SimpleValue::Ignore(token)),
                "panic" => Ok(SimpleValue::Panic(token)),
                x if is_value_keyword(x) => Ok(SimpleValue::ValueKeyword(token)),
                _ => Ok(SimpleValue::TypeKeyword(token)),
            },
            TokenKind::ValueSymbol => Ok(SimpleValue::ValueSymbol(token)),
            TokenKind::TypeSymbol => Ok(SimpleValue::TypeSymbol(token)),
            TokenKind::ValuePathSymbol => Ok(SimpleValue::ValuePathSymbol(token)),
            TokenKind::TypePathSymbol => Ok(SimpleValue::TypePathSymbol(token)),
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            SimpleValue::Ignore(_) => "_".into(),
            SimpleValue::Empty(_) => "()".into(),
            SimpleValue::Panic(_) => "panic".into(),
            SimpleValue::Prim(token) => token.to_string(),
            SimpleValue::ValueKeyword(token) => token.to_string(),
            SimpleValue::TypeKeyword(token) => token.to_string(),
            SimpleValue::ValueSymbol(token) => token.to_string(),
            SimpleValue::TypeSymbol(token) => token.to_string(),
            SimpleValue::ValuePathSymbol(token) => token.to_string(),
            SimpleValue::TypePathSymbol(token) => token.to_string(),
        }
    }
}

impl fmt::Display for SimpleValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
