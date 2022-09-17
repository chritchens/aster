use crate::chunk::StringChunk;
use crate::chunks::StringChunks;
use crate::loc::Loc;

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
    pub chunks: Option<StringChunks>,
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

    pub fn new_value_symbol() -> Self {
        Token {
            kind: TokenKind::ValueSymbol,
            chunks: None,
        }
    }

    pub fn new_type_symbol() -> Self {
        Token {
            kind: TokenKind::TypeSymbol,
            chunks: None,
        }
    }

    pub fn new_path_symbol() -> Self {
        Token {
            kind: TokenKind::PathSymbol,
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

    pub fn file(&self) -> Option<String> {
        if let Some(ref chunks) = self.chunks {
            if !chunks.files.is_empty() {
                Some(chunks.files[0].clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn loc(&self) -> Option<Loc> {
        if let Some(ref chunks) = self.chunks {
            if !chunks.content.is_empty() {
                Some(chunks.content[0].loc.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn push(&mut self, chunk: StringChunk) {
        let mut chunks = self.chunks.clone().unwrap_or_default();
        chunks.push(chunk);
        self.chunks.replace(chunks);
    }
}
