use crate::error::{Error, SyntacticError};
use crate::loc::Loc;
use crate::result::Result;
use crate::syntax::{is_type_keyword, is_value_keyword};
use crate::syntax::{is_type_symbol, is_value_symbol, symbol_name};
use crate::token::{Token, TokenKind};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum BasicValue {
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
    Unknown(Token),
}

impl Default for BasicValue {
    fn default() -> BasicValue {
        BasicValue::Unknown(Token::new())
    }
}

impl BasicValue {
    pub fn new() -> BasicValue {
        BasicValue::default()
    }

    pub fn token(&self) -> Token {
        match self {
            BasicValue::Ignore(token) => token.clone(),
            BasicValue::Empty(token) => token.clone(),
            BasicValue::Panic(token) => token.clone(),
            BasicValue::Prim(token) => token.clone(),
            BasicValue::ValueKeyword(token) => token.clone(),
            BasicValue::TypeKeyword(token) => token.clone(),
            BasicValue::ValueSymbol(token) => token.clone(),
            BasicValue::TypeSymbol(token) => token.clone(),
            BasicValue::ValuePathSymbol(token) => token.clone(),
            BasicValue::TypePathSymbol(token) => token.clone(),
            BasicValue::Unknown(token) => token.clone(),
        }
    }

    pub fn file(&self) -> String {
        match self {
            BasicValue::Ignore(token) => token.file(),
            BasicValue::Empty(token) => token.file(),
            BasicValue::Panic(token) => token.file(),
            BasicValue::ValueKeyword(token) => token.file(),
            BasicValue::TypeKeyword(token) => token.file(),
            BasicValue::Prim(token) => token.file(),
            BasicValue::ValueSymbol(token) => token.file(),
            BasicValue::TypeSymbol(token) => token.file(),
            BasicValue::ValuePathSymbol(token) => token.file(),
            BasicValue::TypePathSymbol(token) => token.file(),
            BasicValue::Unknown(token) => token.file(),
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        match self {
            BasicValue::Ignore(token) => token.loc(),
            BasicValue::Empty(token) => token.loc(),
            BasicValue::Panic(token) => token.loc(),
            BasicValue::ValueKeyword(token) => token.loc(),
            BasicValue::TypeKeyword(token) => token.loc(),
            BasicValue::Prim(token) => token.loc(),
            BasicValue::ValueSymbol(token) => token.loc(),
            BasicValue::TypeSymbol(token) => token.loc(),
            BasicValue::ValuePathSymbol(token) => token.loc(),
            BasicValue::TypePathSymbol(token) => token.loc(),
            BasicValue::Unknown(token) => token.loc(),
        }
    }

    pub fn from_token(token: &Token) -> Result<BasicValue> {
        let token = token.to_owned();

        match token.kind {
            TokenKind::Comment | TokenKind::DocComment => {
                Err(Error::Syntactic(SyntacticError {
                    loc: token.loc(),
                    desc: "unexpected comment marker".into(),
                }))
            }
            TokenKind::FormStart | TokenKind::FormEnd => {
                Err(Error::Syntactic(SyntacticError {
                    loc: token.loc(),
                    desc: "unexpected form punctuation".into(),
                }))
            }
            TokenKind::EmptyLiteral => Ok(BasicValue::Empty(token)),
            TokenKind::UIntLiteral
            | TokenKind::IntLiteral
            | TokenKind::FloatLiteral
            | TokenKind::CharLiteral
            | TokenKind::StringLiteral => Ok(BasicValue::Prim(token)),
            TokenKind::Keyword => match token.to_string().as_str() {
                "_" => Ok(BasicValue::Ignore(token)),
                "panic" => Ok(BasicValue::Panic(token)),
                x if is_value_keyword(x) => Ok(BasicValue::ValueKeyword(token)),
                x if is_type_keyword(x) => Ok(BasicValue::TypeKeyword(token)),
                _ => unreachable!(),
            },
            TokenKind::ValueSymbol => Ok(BasicValue::ValueSymbol(token)),
            TokenKind::TypeSymbol => Ok(BasicValue::TypeSymbol(token)),
            TokenKind::PathSymbol => {
                let name = token.to_string();
                let unqualified = symbol_name(&name);

                if is_type_symbol(&unqualified) {
                    Ok(BasicValue::TypePathSymbol(token))
                } else if is_value_symbol(&unqualified) {
                    Ok(BasicValue::ValuePathSymbol(token))
                } else {
                    Err(Error::Syntactic(SyntacticError {
                        loc: token.loc(),
                        desc: "expected a qualified type symbol or a qualified value symbol".into(),
                    }))
                }
            }
        }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        match self {
            BasicValue::Ignore(_) => "_".into(),
            BasicValue::Empty(_) => "()".into(),
            BasicValue::Panic(_) => "panic".into(),
            BasicValue::Prim(token) => token.to_string(),
            BasicValue::ValueKeyword(token) => token.to_string(),
            BasicValue::TypeKeyword(token) => token.to_string(),
            BasicValue::ValueSymbol(token) => token.to_string(),
            BasicValue::TypeSymbol(token) => token.to_string(),
            BasicValue::ValuePathSymbol(token) => token.to_string(),
            BasicValue::TypePathSymbol(token) => token.to_string(),
            BasicValue::Unknown(token) => token.to_string(),
        }
    }
}

impl fmt::Display for BasicValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
