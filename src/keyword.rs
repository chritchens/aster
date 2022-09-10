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

pub const COMMENT_MARK: char = '#';

pub const COMMENT_MARK_POSTFIX: char = '!';

pub const FORM_START: char = '(';

pub const FORM_END: char = ')';
