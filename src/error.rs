use crate::loc::Loc;
use std::convert;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug, Eq, PartialEq)]
pub struct SyntacticError {
    pub loc: Option<Loc>,
    pub desc: String,
}

impl fmt::Display for SyntacticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref loc) = self.loc {
            write!(f, "syntactic error at {}: {}", loc.to_string(), self.desc)
        } else {
            write!(f, "syntactic error: {}", self.desc)
        }
    }
}

impl error::Error for SyntacticError {}

#[derive(Debug, Eq, PartialEq)]
pub struct SemanticError {
    pub loc: Option<Loc>,
    pub desc: String,
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref loc) = self.loc {
            write!(f, "semantic error at {}: {}", loc.to_string(), self.desc)
        } else {
            write!(f, "semantic error: {}", self.desc)
        }
    }
}

impl error::Error for SemanticError {}

#[derive(Debug)]
pub enum Error {
    Syntactic(SyntacticError),
    Semantic(SemanticError),
    IO(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntactic(err) => err.fmt(f),
            Self::Semantic(err) => err.fmt(f),
            Self::IO(err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Syntactic(err) => err.source(),
            Self::Semantic(err) => err.source(),
            Self::IO(err) => err.source(),
        }
    }
}

impl convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}
