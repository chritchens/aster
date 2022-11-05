use crate::error::{Error, SyntaxError};
use crate::result::Result;
use std::convert;
use std::fmt;

pub const KEYWORDS: [&str; 40] = [
    "builtin", "import", "export", "def", "type", "prim", "sum", "prod", "sig", "fun", "attrs",
    "app", "case", "size", "load", "store", "cast", "dup", "drop", "panic", "Builtin", "Empty",
    "Prim", "UInt", "Int", "Float", "Size", "Char", "String", "Mem", "Path", "IO", "Ctx", "Sum",
    "Prod", "Sig", "Fun", "App", "Attrs", "Type",
];

pub fn is_keyword(s: &str) -> bool {
    KEYWORDS.iter().any(|&k| k == s)
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Keyword {
    Builtin,
    Import,
    Export,
    Def,
    Type,
    Prim,
    Sum,
    Prod,
    Sig,
    Fun,
    Attrs,
    App,
    Case,
    Size,
    Load,
    Store,
    Cast,
    Dup,
    Drop,
    Panic,
    BuiltinT,
    EmptyT,
    PrimT,
    UIntT,
    IntT,
    FloatT,
    SizeT,
    CharT,
    StringT,
    MemT,
    PathT,
    IOT,
    CtxT,
    SumT,
    ProdT,
    SigT,
    FunT,
    AppT,
    AttrsT,
    TypeT,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::Builtin => write!(f, "builtin"),
            Keyword::Import => write!(f, "import"),
            Keyword::Export => write!(f, "export"),
            Keyword::Def => write!(f, "def"),
            Keyword::Type => write!(f, "type"),
            Keyword::Prim => write!(f, "prim"),
            Keyword::Sum => write!(f, "sum"),
            Keyword::Prod => write!(f, "prod"),
            Keyword::Sig => write!(f, "sig"),
            Keyword::Fun => write!(f, "fun"),
            Keyword::Attrs => write!(f, "attrs"),
            Keyword::App => write!(f, "app"),
            Keyword::Case => write!(f, "case"),
            Keyword::Size => write!(f, "size"),
            Keyword::Load => write!(f, "load"),
            Keyword::Store => write!(f, "store"),
            Keyword::Cast => write!(f, "cast"),
            Keyword::Dup => write!(f, "dup"),
            Keyword::Drop => write!(f, "drop"),
            Keyword::Panic => write!(f, "panic"),
            Keyword::BuiltinT => write!(f, "Builtin"),
            Keyword::EmptyT => write!(f, "Empty"),
            Keyword::PrimT => write!(f, "Prim"),
            Keyword::UIntT => write!(f, "UInt"),
            Keyword::IntT => write!(f, "Int"),
            Keyword::FloatT => write!(f, "Float"),
            Keyword::SizeT => write!(f, "Size"),
            Keyword::CharT => write!(f, "Char"),
            Keyword::StringT => write!(f, "String"),
            Keyword::MemT => write!(f, "Mem"),
            Keyword::PathT => write!(f, "Path"),
            Keyword::IOT => write!(f, "IO"),
            Keyword::CtxT => write!(f, "Ctx"),
            Keyword::SumT => write!(f, "Sum"),
            Keyword::ProdT => write!(f, "Prod"),
            Keyword::SigT => write!(f, "Sig"),
            Keyword::FunT => write!(f, "Fun"),
            Keyword::AppT => write!(f, "App"),
            Keyword::AttrsT => write!(f, "Attrs"),
            Keyword::TypeT => write!(f, "Type"),
        }
    }
}

impl Keyword {
    pub fn is(s: &str) -> bool {
        KEYWORDS.iter().any(|k| k == &s)
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "builtin" => Ok(Keyword::Builtin),
            "import" => Ok(Keyword::Import),
            "export" => Ok(Keyword::Export),
            "def" => Ok(Keyword::Def),
            "type" => Ok(Keyword::Type),
            "prim" => Ok(Keyword::Prim),
            "sum" => Ok(Keyword::Sum),
            "prod" => Ok(Keyword::Prod),
            "sig" => Ok(Keyword::Sig),
            "fun" => Ok(Keyword::Fun),
            "attrs" => Ok(Keyword::Attrs),
            "app" => Ok(Keyword::App),
            "case" => Ok(Keyword::Case),
            "size" => Ok(Keyword::Size),
            "load" => Ok(Keyword::Load),
            "store" => Ok(Keyword::Store),
            "cast" => Ok(Keyword::Cast),
            "dup" => Ok(Keyword::Dup),
            "drop" => Ok(Keyword::Drop),
            "panic" => Ok(Keyword::Panic),
            "Builtin" => Ok(Keyword::BuiltinT),
            "Empty" => Ok(Keyword::EmptyT),
            "Prim" => Ok(Keyword::PrimT),
            "UInt" => Ok(Keyword::UIntT),
            "Int" => Ok(Keyword::IntT),
            "Float" => Ok(Keyword::FloatT),
            "Size" => Ok(Keyword::SizeT),
            "Char" => Ok(Keyword::CharT),
            "String" => Ok(Keyword::StringT),
            "Mem" => Ok(Keyword::MemT),
            "Path" => Ok(Keyword::PathT),
            "IO" => Ok(Keyword::IOT),
            "Ctx" => Ok(Keyword::CtxT),
            "Sum" => Ok(Keyword::SumT),
            "Prod" => Ok(Keyword::ProdT),
            "Sig" => Ok(Keyword::SigT),
            "Fun" => Ok(Keyword::FunT),
            "App" => Ok(Keyword::AppT),
            "Attrs" => Ok(Keyword::AttrsT),
            "Type" => Ok(Keyword::TypeT),
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

    let len = s.len();

    match s {
        x if x.starts_with('b') => len > 1 && x[1..].chars().all(|c| c.is_ascii_digit() && c < '2'),
        x if x.starts_with('o') => len > 1 && x[1..].chars().all(|c| c.is_ascii_digit() && c < '8'),
        x if x.starts_with('x') => {
            len > 1 && x[1..].chars().all(|c| c.is_ascii_hexdigit() && c >= 'a')
        }
        x if x.starts_with('X') => {
            len > 1 && x[1..].chars().all(|c| c.is_ascii_hexdigit() && c <= 'F')
        }
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

pub fn is_symbol_punctuation(c: char) -> bool {
    SYMBOL_START_PUNCTUATION.iter().any(|p| p == &c)
}

pub const SYMBOL_PATH_SEPARATOR: char = '.';

pub fn is_symbol_path_separator(c: char) -> bool {
    c == SYMBOL_PATH_SEPARATOR
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
        && s.chars()
            .all(|c| is_path_symbol_char(c) || c == SYMBOL_PATH_SEPARATOR)
        && !s.ends_with(SYMBOL_PATH_SEPARATOR)
}

pub fn is_qualified(s: &str) -> bool {
    s.chars().any(|c| c == SYMBOL_PATH_SEPARATOR)
}

pub fn symbol_qualifier(s: &str) -> String {
    let mut parts: Vec<String> = s.split('.').map(|s| s.into()).collect();

    parts.remove(parts.len() - 1);

    parts.join(".")
}

pub fn symbol_name(s: &str) -> String {
    let parts: Vec<String> = s.split('.').map(|s| s.into()).collect();

    parts[parts.len() - 1].clone()
}

pub fn symbol_with_qualifier(s: &str, qualifier: &str) -> String {
    vec![qualifier, s].join(".")
}

pub const FORM_START: char = '(';

pub fn is_form_start(s: &str) -> bool {
    s == FORM_START.to_string()
}

pub const FORM_END: char = ')';

pub fn is_form_end(s: &str) -> bool {
    s == FORM_END.to_string()
}

pub const EMPTY: &str = "";

pub fn is_empty(s: &str) -> bool {
    s == EMPTY
}
