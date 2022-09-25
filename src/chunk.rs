use crate::loc::Loc;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct CharChunk {
    pub loc: Loc,
    pub content: char,
}

impl CharChunk {
    pub fn new() -> Self {
        CharChunk::default()
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        format!("{}", self.content)
    }
}

impl fmt::Display for CharChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.loc, self.content)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct StringChunk {
    pub loc: Loc,
    pub content: String,
}

impl fmt::Display for StringChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.loc, self.content)
    }
}

impl StringChunk {
    pub fn new() -> Self {
        StringChunk::default()
    }

    pub fn from_char_chunk(cc: CharChunk) -> Self {
        StringChunk {
            loc: cc.loc,
            content: cc.content.to_string(),
        }
    }

    pub fn from_char_chunks(ccs: Vec<CharChunk>) -> Self {
        if ccs.is_empty() {
            return StringChunk::default();
        }

        let content: String =
            ccs.iter()
                .map(|cc| cc.content)
                .fold("".into(), |mut acc: String, c: char| {
                    acc.push(c);
                    acc
                });

        StringChunk {
            loc: ccs[0].loc.clone(),
            content,
        }
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn to_str(&self) -> &str {
        self.content.as_str()
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.content.clone()
    }
}
