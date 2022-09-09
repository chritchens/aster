use crate::chunk::Chunk;
use crate::chunks::Chunks;

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
    Symbol,
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
    pub chunks: Option<Chunks>,
}

impl Token {
    pub fn new() -> Self {
        Token::default()
    }

    pub fn new_comment() -> Self {
        Token {
            kind: TokenKind::Comment,
            chunks: None,
        }
    }

    pub fn new_doc_comment() -> Self {
        Token {
            kind: TokenKind::DocComment,
            chunks: None,
        }
    }

    pub fn new_keyword() -> Self {
        Token {
            kind: TokenKind::Keyword,
            chunks: None,
        }
    }

    pub fn new_empty_literal() -> Self {
        Token {
            kind: TokenKind::EmptyLiteral,
            chunks: None,
        }
    }

    pub fn new_uint_literal() -> Self {
        Token {
            kind: TokenKind::UIntLiteral,
            chunks: None,
        }
    }

    pub fn new_int_literal() -> Self {
        Token {
            kind: TokenKind::IntLiteral,
            chunks: None,
        }
    }

    pub fn new_float_literal() -> Self {
        Token {
            kind: TokenKind::FloatLiteral,
            chunks: None,
        }
    }

    pub fn new_char_literal() -> Self {
        Token {
            kind: TokenKind::CharLiteral,
            chunks: None,
        }
    }

    pub fn new_string_literal() -> Self {
        Token {
            kind: TokenKind::StringLiteral,
            chunks: None,
        }
    }

    pub fn new_symbol() -> Self {
        Token {
            kind: TokenKind::Symbol,
            chunks: None,
        }
    }

    pub fn new_form_start() -> Self {
        Token {
            kind: TokenKind::FormStart,
            chunks: None,
        }
    }

    pub fn new_form_end() -> Self {
        Token {
            kind: TokenKind::FormEnd,
            chunks: None,
        }
    }

    pub fn push(&mut self, chunk: Chunk) {
        let mut chunks = self.chunks.clone().unwrap_or(Chunks::new());
        chunks.push(chunk);
        self.chunks.replace(chunks);
    }
}
