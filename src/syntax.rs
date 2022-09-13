use crate::error::{Error, SyntaxError};
use crate::result::Result;
use std::convert;
use std::fmt;

pub const KEYWORDS: [&str; 7] = [
    "include", "deftype", "defsig", "defvar", "defsum", "defprod", "defun",
];

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Keyword {
    Include,
    Deftype,
    Defsig,
    Defvar,
    Defsum,
    Defprod,
    Defun,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::Include => write!(f, "include"),
            Keyword::Deftype => write!(f, "deftype"),
            Keyword::Defsig => write!(f, "defsig"),
            Keyword::Defvar => write!(f, "defvar"),
            Keyword::Defsum => write!(f, "defsum"),
            Keyword::Defprod => write!(f, "defprod"),
            Keyword::Defun => write!(f, "defun"),
        }
    }
}

impl Keyword {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "include" => Ok(Keyword::Include),
            "deftype" => Ok(Keyword::Deftype),
            "defsig" => Ok(Keyword::Defsig),
            "defvar" => Ok(Keyword::Defvar),
            "defsum" => Ok(Keyword::Defsum),
            "defprod" => Ok(Keyword::Defprod),
            "defun" => Ok(Keyword::Defun),
            _ => Err(Error::Syntax(SyntaxError {
                loc: None,
                desc: "expected keyword".into(),
            })),
        }
    }

    pub fn from_string(s: String) -> Result<Self> {
        Keyword::from_str(&s)
    }
}

impl std::str::FromStr for Keyword {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Keyword::from_str(s)
    }
}

impl convert::TryFrom<String> for Keyword {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        Keyword::from_string(s)
    }
}

pub fn is_separator_char(c: char) -> bool {
    c.is_ascii_whitespace()
}

pub const COMMENT_MARK: char = '#';

pub const COMMENT_MARK_POSTFIX: char = '!';

pub const SINGLE_QUOTE: char = '\'';

pub const DOUBLE_QUOTE: char = '"';

pub const SYMBOL_START_PUNCTUATION: [char; 23] = [
    '!', '$', '%', '&', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '\\', '^',
    '_', '`', '|', '~',
];

pub fn is_symbol_punctuation(c: char) -> bool {
    for a in SYMBOL_START_PUNCTUATION.iter() {
        if &c == a {
            return true;
        }
    }

    false
}

pub fn is_symbol_start_char(c: char) -> bool {
    for a in ('A'..='z').into_iter() {
        if c == a {
            return true;
        }
    }

    is_symbol_punctuation(c)
}

pub fn is_symbol_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || (c != COMMENT_MARK
            && c != FORM_START
            && c != FORM_END
            && c != SINGLE_QUOTE
            && c != DOUBLE_QUOTE
            && !c.is_whitespace())
}

pub fn is_symbol_char_no_punctuation(c: char) -> bool {
    is_symbol_char(c) && (!is_symbol_punctuation(c) || c == '.')
}

pub const FORM_START: char = '(';

pub const FORM_END: char = ')';
