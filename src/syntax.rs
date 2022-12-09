use crate::error::{Error, SyntacticError};
use crate::result::Result;
use std::convert;
use std::fmt;

pub const KEYWORDS: [&str; 55] = [
    "module", "block", "_", "builtin", "import", "export", "val", "type", "atomic", "pair", "list",
    "arr", "vec", "map", "sig", "fun", "attrs", "app", "case", "id", "default", "match", "others",
    "size", "load", "store", "ref", "deref", "cast", "dup", "drop", "panic", "Builtin", "Empty",
    "Atomic", "UInt", "Int", "Float", "Size", "Pointer", "Ref", "Char", "String", "Mem", "Path",
    "IO", "Ctx", "Enum", "Pair", "List", "Arr", "Vec", "Map", "Fun", "Type",
];

pub fn is_keyword(s: &str) -> bool {
    KEYWORDS.iter().any(|&k| k == s)
}

pub fn is_value_keyword(s: &str) -> bool {
    is_keyword(s) && is_value_symbol_start_char(s.chars().next().unwrap())
}

pub fn is_type_keyword(s: &str) -> bool {
    is_keyword(s) && is_type_symbol_start_char(s.chars().next().unwrap())
}

pub const IGNORE: &str = "_";

pub fn is_ignore_keyword(s: &str) -> bool {
    s == IGNORE
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum Keyword {
    Module,
    Block,
    Ignore,
    Builtin,
    Import,
    Export,
    Val,
    Type,
    Atomic,
    Pair,
    List,
    Arr,
    Vec,
    Map,
    Sig,
    Fun,
    Attrs,
    App,
    Id,
    Default,
    Case,
    Match,
    Others,
    Size,
    Ref,
    Deref,
    Load,
    Store,
    Cast,
    Dup,
    Drop,
    Panic,
    BuiltinT,
    EmptyT,
    AtomicT,
    UIntT,
    IntT,
    FloatT,
    SizeT,
    PointerT,
    RefT,
    CharT,
    StringT,
    MemT,
    PathT,
    IOT,
    CtxT,
    EnumT,
    PairT,
    ListT,
    ArrT,
    VecT,
    MapT,
    FunT,
    TypeT,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::Module => write!(f, "module"),
            Keyword::Block => write!(f, "block"),
            Keyword::Ignore => write!(f, "_"),
            Keyword::Builtin => write!(f, "builtin"),
            Keyword::Import => write!(f, "import"),
            Keyword::Export => write!(f, "export"),
            Keyword::Val => write!(f, "val"),
            Keyword::Type => write!(f, "type"),
            Keyword::Atomic => write!(f, "atomic"),
            Keyword::Pair => write!(f, "pair"),
            Keyword::List => write!(f, "list"),
            Keyword::Arr => write!(f, "arr"),
            Keyword::Vec => write!(f, "vec"),
            Keyword::Map => write!(f, "map"),
            Keyword::Sig => write!(f, "sig"),
            Keyword::Fun => write!(f, "fun"),
            Keyword::Attrs => write!(f, "attrs"),
            Keyword::App => write!(f, "app"),
            Keyword::Id => write!(f, "id"),
            Keyword::Default => write!(f, "default"),
            Keyword::Case => write!(f, "case"),
            Keyword::Match => write!(f, "match"),
            Keyword::Others => write!(f, "others"),
            Keyword::Size => write!(f, "size"),
            Keyword::Ref => write!(f, "ref"),
            Keyword::Deref => write!(f, "deref"),
            Keyword::Load => write!(f, "load"),
            Keyword::Store => write!(f, "store"),
            Keyword::Cast => write!(f, "cast"),
            Keyword::Dup => write!(f, "dup"),
            Keyword::Drop => write!(f, "drop"),
            Keyword::Panic => write!(f, "panic"),
            Keyword::BuiltinT => write!(f, "Builtin"),
            Keyword::EmptyT => write!(f, "Empty"),
            Keyword::AtomicT => write!(f, "Atomic"),
            Keyword::UIntT => write!(f, "UInt"),
            Keyword::IntT => write!(f, "Int"),
            Keyword::FloatT => write!(f, "Float"),
            Keyword::SizeT => write!(f, "Size"),
            Keyword::PointerT => write!(f, "Pointer"),
            Keyword::RefT => write!(f, "Ref"),
            Keyword::CharT => write!(f, "Char"),
            Keyword::StringT => write!(f, "String"),
            Keyword::MemT => write!(f, "Mem"),
            Keyword::PathT => write!(f, "Path"),
            Keyword::IOT => write!(f, "IO"),
            Keyword::CtxT => write!(f, "Ctx"),
            Keyword::EnumT => write!(f, "Enum"),
            Keyword::PairT => write!(f, "Pair"),
            Keyword::ListT => write!(f, "List"),
            Keyword::ArrT => write!(f, "Arr"),
            Keyword::VecT => write!(f, "Vec"),
            Keyword::MapT => write!(f, "Map"),
            Keyword::FunT => write!(f, "Fun"),
            Keyword::TypeT => write!(f, "Type"),
        }
    }
}

impl Keyword {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "module" => Ok(Keyword::Module),
            "block" => Ok(Keyword::Block),
            "_" => Ok(Keyword::Ignore),
            "builtin" => Ok(Keyword::Builtin),
            "import" => Ok(Keyword::Import),
            "export" => Ok(Keyword::Export),
            "val" => Ok(Keyword::Val),
            "type" => Ok(Keyword::Type),
            "atomic" => Ok(Keyword::Atomic),
            "pair" => Ok(Keyword::Pair),
            "list" => Ok(Keyword::List),
            "arr" => Ok(Keyword::Arr),
            "vec" => Ok(Keyword::Vec),
            "map" => Ok(Keyword::Map),
            "sig" => Ok(Keyword::Sig),
            "fun" => Ok(Keyword::Fun),
            "attrs" => Ok(Keyword::Attrs),
            "app" => Ok(Keyword::App),
            "id" => Ok(Keyword::Id),
            "default" => Ok(Keyword::Default),
            "case" => Ok(Keyword::Case),
            "match" => Ok(Keyword::Match),
            "others" => Ok(Keyword::Others),
            "size" => Ok(Keyword::Size),
            "ref" => Ok(Keyword::Ref),
            "deref" => Ok(Keyword::Deref),
            "load" => Ok(Keyword::Load),
            "store" => Ok(Keyword::Store),
            "cast" => Ok(Keyword::Cast),
            "dup" => Ok(Keyword::Dup),
            "drop" => Ok(Keyword::Drop),
            "panic" => Ok(Keyword::Panic),
            "Builtin" => Ok(Keyword::BuiltinT),
            "Empty" => Ok(Keyword::EmptyT),
            "Atomic" => Ok(Keyword::AtomicT),
            "UInt" => Ok(Keyword::UIntT),
            "Int" => Ok(Keyword::IntT),
            "Float" => Ok(Keyword::FloatT),
            "Size" => Ok(Keyword::SizeT),
            "Pointer" => Ok(Keyword::PointerT),
            "Ref" => Ok(Keyword::RefT),
            "Char" => Ok(Keyword::CharT),
            "String" => Ok(Keyword::StringT),
            "Mem" => Ok(Keyword::MemT),
            "Path" => Ok(Keyword::PathT),
            "IO" => Ok(Keyword::IOT),
            "Ctx" => Ok(Keyword::CtxT),
            "Enum" => Ok(Keyword::EnumT),
            "Pair" => Ok(Keyword::PairT),
            "List" => Ok(Keyword::ListT),
            "Arr" => Ok(Keyword::ArrT),
            "Vec" => Ok(Keyword::VecT),
            "Map" => Ok(Keyword::MapT),
            "Fun" => Ok(Keyword::FunT),
            "Type" => Ok(Keyword::TypeT),
            _ => Err(Error::Syntactic(SyntacticError {
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
    ('A'..='Z').any(|l| l == c) || ('a'..='z').any(|l| l == c) || is_symbol_punctuation(c)
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

    if is_keyword(s) {
        return false;
    }

    if !s.starts_with(is_symbol_start_char) {
        return false;
    }

    let only_punctuation = s.starts_with(is_symbol_punctuation);
    let is_path = s.contains(SYMBOL_PATH_SEPARATOR);
    let len = s.len();

    if only_punctuation {
        if len > 3 {
            return false;
        }

        s.chars().all(is_symbol_punctuation)
    } else if is_path {
        if s.split(SYMBOL_PATH_SEPARATOR).any(is_keyword) {
            return false;
        }

        let mut chars_iter = s.chars();

        is_path_symbol_start_char(chars_iter.next().unwrap())
            && chars_iter.all(|c| is_path_symbol_char(c) || c == SYMBOL_PATH_SEPARATOR)
            && !s.ends_with(SYMBOL_PATH_SEPARATOR)
    } else if let Some(pos) = s.chars().position(is_symbol_punctuation) {
        pos == len - 1
    } else {
        true
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

pub fn is_type_path_symbol(s: &str) -> bool {
    let unqualified = symbol_name(s);

    is_type_symbol(&unqualified)
}

pub fn is_value_path_symbol(s: &str) -> bool {
    let unqualified = symbol_name(s);

    is_value_symbol(&unqualified)
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
