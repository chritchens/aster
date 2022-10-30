use crate::chunk::{StringChunk, StringChunks};
use crate::loc::Loc;
use crate::syntax::EMPTY;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TokenKind {
    Comment,
    DocComment,
    Keyword,
    EmptyLiteral,
    UIntLiteral,
    IntLiteral,
    FloatLiteral,
    CharLiteral,
    StringLiteral,
    ValueSymbol,
    TypeSymbol,
    PathSymbol,
    FormStart,
    FormEnd,
}

impl Default for TokenKind {
    fn default() -> Self {
        TokenKind::Comment
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Token {
    pub kind: TokenKind,
    pub chunks: StringChunks,
}

impl Token {
    pub fn new() -> Self {
        Token::default()
    }

    pub fn new_from_kind(kind: TokenKind) -> Self {
        Token {
            kind,
            chunks: StringChunks::new(),
        }
    }

    pub fn new_comment() -> Self {
        Token::new_from_kind(TokenKind::Comment)
    }

    pub fn new_doc_comment() -> Self {
        Token::new_from_kind(TokenKind::DocComment)
    }

    pub fn new_keyword() -> Self {
        Token::new_from_kind(TokenKind::Keyword)
    }

    pub fn new_empty_literal() -> Self {
        Token::new_from_kind(TokenKind::EmptyLiteral)
    }

    pub fn new_uint_literal() -> Self {
        Token::new_from_kind(TokenKind::UIntLiteral)
    }

    pub fn new_int_literal() -> Self {
        Token::new_from_kind(TokenKind::IntLiteral)
    }

    pub fn new_float_literal() -> Self {
        Token::new_from_kind(TokenKind::FloatLiteral)
    }

    pub fn new_char_literal() -> Self {
        Token::new_from_kind(TokenKind::CharLiteral)
    }

    pub fn new_string_literal() -> Self {
        Token::new_from_kind(TokenKind::StringLiteral)
    }

    pub fn new_value_symbol() -> Self {
        Token::new_from_kind(TokenKind::ValueSymbol)
    }

    pub fn new_type_symbol() -> Self {
        Token::new_from_kind(TokenKind::TypeSymbol)
    }

    pub fn new_path_symbol() -> Self {
        Token::new_from_kind(TokenKind::PathSymbol)
    }

    pub fn new_form_start() -> Self {
        Token::new_from_kind(TokenKind::FormStart)
    }

    pub fn new_form_end() -> Self {
        Token::new_from_kind(TokenKind::FormEnd)
    }

    pub fn file(&self) -> String {
        if !self.chunks.files.is_empty() {
            self.chunks.files[0].clone()
        } else {
            EMPTY.into()
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        if !self.chunks.content.is_empty() {
            Some(self.chunks.content[0].loc.clone())
        } else {
            None
        }
    }

    pub fn push(&mut self, chunk: StringChunk) {
        self.chunks.push(chunk)
    }
}
