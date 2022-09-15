use crate::loc::Loc;
use std::convert;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug, Eq, PartialEq)]
pub struct SyntaxError {
    pub loc: Option<Loc>,
    pub desc: String,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(loc) = self.loc.as_ref() {
            let file = loc.file.clone().unwrap_or_else(|| "".into());

            write!(
                f,
                "{} at position {} of line {} of file {}",
                self.desc, loc.pos, loc.line, file
            )
        } else {
            write!(f, "{}", self.desc)
        }
    }
}

impl error::Error for SyntaxError {}

#[derive(Debug, Eq, PartialEq)]
pub struct ParsingError {
    pub loc: Option<Loc>,
    pub desc: String,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(loc) = self.loc.as_ref() {
            let file = loc.file.clone().unwrap_or_else(|| "".into());

            write!(
                f,
                "{} at position {} of line {} of file {}",
                self.desc, loc.pos, loc.line, file
            )
        } else {
            write!(f, "{}", self.desc)
        }
    }
}

impl error::Error for ParsingError {}

#[derive(Debug, Eq, PartialEq)]
pub struct SemanticError {
    pub loc: Option<Loc>,
    pub desc: String,
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(loc) = self.loc.as_ref() {
            let file = loc.file.clone().unwrap_or_else(|| "".into());

            write!(
                f,
                "{} at position {} of line {} of file {}",
                self.desc, loc.pos, loc.line, file
            )
        } else {
            write!(f, "{}", self.desc)
        }
    }
}

impl error::Error for SemanticError {}

#[derive(Debug)]
pub enum Error {
    Syntax(SyntaxError),
    Parsing(ParsingError),
    Semantic(SemanticError),
    IO(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax(err) => err.fmt(f),
            Self::Parsing(err) => err.fmt(f),
            Self::Semantic(err) => err.fmt(f),
            Self::IO(err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Syntax(err) => err.source(),
            Self::Parsing(err) => err.source(),
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
