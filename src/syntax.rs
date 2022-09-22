use crate::error::{Error, SyntaxError};
use crate::result::Result;
use std::convert;
use std::fmt;

pub const KEYWORDS: [&str; 18] = [
    "import", "deftype", "defsig", "defprim", "defsum", "defprod", "defun", "Empty", "UInt", "Int",
    "Float", "Char", "String", "Path", "Sum", "Prod", "Fun", "Type",
];

pub fn is_keyword(s: &str) -> bool {
    KEYWORDS.iter().any(|&k| k == s)
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Keyword {
    Import,
    Deftype,
    Defsig,
    Defprim,
    Defsum,
    Defprod,
    Defun,
    Empty,
    UInt,
    Int,
    Float,
    Char,
    String,
    Path,
    Sum,
    Prod,
    Fun,
    Type,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::Import => write!(f, "import"),
            Keyword::Deftype => write!(f, "deftype"),
            Keyword::Defsig => write!(f, "defsig"),
            Keyword::Defprim => write!(f, "defprim"),
            Keyword::Defsum => write!(f, "defsum"),
            Keyword::Defprod => write!(f, "defprod"),
            Keyword::Defun => write!(f, "defun"),
            Keyword::Empty => write!(f, "Empty"),
            Keyword::UInt => write!(f, "UInt"),
            Keyword::Int => write!(f, "Int"),
            Keyword::Float => write!(f, "Float"),
            Keyword::Char => write!(f, "Char"),
            Keyword::String => write!(f, "String"),
            Keyword::Path => write!(f, "Path"),
            Keyword::Sum => write!(f, "Sum"),
            Keyword::Prod => write!(f, "Prod"),
            Keyword::Fun => write!(f, "Fun"),
            Keyword::Type => write!(f, "Type"),
        }
    }
}

impl Keyword {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "import" => Ok(Keyword::Import),
            "deftype" => Ok(Keyword::Deftype),
            "defsig" => Ok(Keyword::Defsig),
            "defprim" => Ok(Keyword::Defprim),
            "defsum" => Ok(Keyword::Defsum),
            "defprod" => Ok(Keyword::Defprod),
            "defun" => Ok(Keyword::Defun),
            "Empty" => Ok(Keyword::Empty),
            "UInt" => Ok(Keyword::UInt),
            "Int" => Ok(Keyword::Int),
            "Float" => Ok(Keyword::Float),
            "Char" => Ok(Keyword::Char),
            "String" => Ok(Keyword::String),
            "Path" => Ok(Keyword::Path),
            "Sum" => Ok(Keyword::Sum),
            "Prod" => Ok(Keyword::Prod),
            "Fun" => Ok(Keyword::Fun),
            "Type" => Ok(Keyword::Type),
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

pub fn is_whitespace(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_whitespace())
}

pub const ESCAPE_CHAR: char = '\\';

pub fn is_escape_char(s: &str) -> bool {
    s == ESCAPE_CHAR.to_string()
}

pub fn is_separator_char(c: char) -> bool {
    c.is_ascii_whitespace()
        || c == COMMENT_MARK
        || c == SINGLE_QUOTE
        || c == DOUBLE_QUOTE
        || c == FORM_START
        || c == FORM_END
}

pub const COMMENT_MARK: char = '#';

pub const COMMENT_MARK_POSTFIX: char = '!';

pub fn is_comment_mark(s: &str) -> bool {
    s == COMMENT_MARK.to_string()
}

pub fn is_doc_comment_mark(s: &str) -> bool {
    s == [COMMENT_MARK.to_string(), COMMENT_MARK_POSTFIX.to_string()].join("")
}

pub fn is_uint_literal(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    match s {
        x if x.starts_with('b') => x[1..].chars().all(|c| c.is_ascii_digit() && c < '2'),
        x if x.starts_with('o') => x[1..].chars().all(|c| c.is_ascii_digit() && c < '8'),
        x if x.starts_with('x') => x[1..].chars().all(|c| c.is_ascii_hexdigit() && c >= 'a'),
        x if x.starts_with('X') => x[1..].chars().all(|c| c.is_ascii_hexdigit() && c <= 'F'),
        x => x.chars().all(|c| c.is_ascii_digit()),
    }
}

pub fn is_int_literal(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    if !s.starts_with('+') && !s.starts_with('-') {
        return false;
    }

    is_uint_literal(&s[1..])
}

pub fn is_float_literal(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    if !s.starts_with('+') && !s.starts_with('-') && !s.starts_with(|c: char| c.is_ascii_digit()) {
        return false;
    }

    let point_idxs: Vec<usize> = s.match_indices('.').map(|(idx, _)| idx).collect();

    if point_idxs.len() != 1 {
        return false;
    }

    let point_idx = point_idxs[0];

    let chars = s[1..].chars();
    let mut exp_idx = 0;

    for (mut idx, c) in chars.enumerate() {
        idx += 1;

        if !c.is_ascii_digit() {
            if c == '+' || c == '-' {
                if idx == 1 {
                    return false;
                }

                if idx != exp_idx + 1 {
                    return false;
                }
            } else if c == 'E' {
                exp_idx = idx;
                if idx < point_idx || idx == point_idx + 1 {
                    return false;
                }
            } else if c != '.' {
                return false;
            }
        }
    }

    true
}

pub fn is_hex_char_letter(c: char) -> bool {
    ('A'..='F').chain('a'..='f').any(|l| l == c)
}

pub const SINGLE_QUOTE: char = '\'';

pub fn is_single_quote(s: &str) -> bool {
    s == SINGLE_QUOTE.to_string()
}

pub const DOUBLE_QUOTE: char = '"';

pub fn is_double_quote(s: &str) -> bool {
    s == DOUBLE_QUOTE.to_string()
}

pub const SYMBOL_START_PUNCTUATION: [char; 23] = [
    '!', '$', '%', '&', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '\\', '^',
    '_', '`', '|', '~',
];

pub const SYMBOL_PATH_SEPARATOR: char = '.';

pub fn is_symbol_punctuation(c: char) -> bool {
    SYMBOL_START_PUNCTUATION.iter().any(|p| p == &c)
}

pub fn is_symbol_start_char(c: char) -> bool {
    ('A'..='z').any(|l| l == c) || is_symbol_punctuation(c)
}

pub fn is_type_symbol_start_char(c: char) -> bool {
    ('A'..='Z').any(|l| l == c)
}

pub fn is_value_symbol_start_char(c: char) -> bool {
    ('a'..='z').any(|l| l == c) || (is_symbol_punctuation(c) && c != SYMBOL_PATH_SEPARATOR)
}

pub fn is_path_symbol_start_char(c: char) -> bool {
    ('a'..='z').any(|l| l == c)
}

pub fn is_path_symbol_char(c: char) -> bool {
    ('A'..='Z').any(|l| l == c) || ('a'..='z').any(|l| l == c)
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
    is_symbol_char(c) && !is_symbol_punctuation(c)
}

pub fn is_symbol(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    if !s.starts_with(is_symbol_start_char) {
        return false;
    }

    let only_punctuation = s.starts_with(is_symbol_punctuation);
    let is_path = s.contains(SYMBOL_PATH_SEPARATOR);

    if only_punctuation {
        if s.len() > 3 {
            return false;
        }

        s.chars().all(is_symbol_punctuation)
    } else if is_path {
        let mut chars_iter = s.chars();

        is_path_symbol_start_char(chars_iter.next().unwrap())
            && chars_iter.all(|c| is_path_symbol_char(c) || c == SYMBOL_PATH_SEPARATOR)
            && !s.ends_with(SYMBOL_PATH_SEPARATOR)
    } else {
        s.chars().all(is_symbol_char_no_punctuation)
    }
}

pub fn is_value_symbol(s: &str) -> bool {
    is_symbol(s)
        && is_value_symbol_start_char(s.chars().next().unwrap())
        && !s.contains(SYMBOL_PATH_SEPARATOR)
}

pub fn is_type_symbol(s: &str) -> bool {
    is_symbol(s)
        && is_type_symbol_start_char(s.chars().next().unwrap())
        && !s.chars().any(is_symbol_punctuation)
}

pub fn is_path_symbol(s: &str) -> bool {
    is_symbol(s)
        && is_path_symbol_start_char(s.chars().next().unwrap())
        && s.chars().all(|c| is_path_symbol_char(c) || c == '.')
        && !s.ends_with('.')
}

pub const FORM_START: char = '(';

pub fn is_form_start(s: &str) -> bool {
    s == FORM_START.to_string()
}

pub const FORM_END: char = ')';

pub fn is_form_end(s: &str) -> bool {
    s == FORM_END.to_string()
}
