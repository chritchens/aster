use crate::chunk::{CharChunk, StringChunk};
use crate::loc::Loc;
use crate::syntax::is_separator_char;
use crate::syntax::{COMMENT_MARK, COMMENT_MARK_POSTFIX};
use std::convert;
use std::iter;
use std::ops;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct CharChunks {
    pub files: Vec<String>,
    pub content: Vec<CharChunk>,
}

impl CharChunks {
    pub fn new() -> Self {
        CharChunks::default()
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, chunk: CharChunk) {
        let chunk_file = chunk.loc.file.clone();

        let has_file = self.files.iter().any(|file| {
            chunk
                .loc
                .file
                .as_ref()
                .map(|chunk_file| chunk_file == file)
                .unwrap_or(false)
        });

        if chunk_file.is_some() && !has_file {
            if let Some(chunk_file) = chunk_file {
                self.files.push(chunk_file);
            }
        }

        self.content.push(chunk)
    }

    pub fn from_chunk(chunk: CharChunk) -> Self {
        CharChunks {
            files: chunk
                .loc
                .file
                .clone()
                .map(|file| vec![file])
                .or_else(|| Some(vec![]))
                .unwrap(),
            content: vec![chunk],
        }
    }

    pub fn from_str(s: &str) -> Self {
        let mut line = 0;
        let mut pos = 0;

        let chunks: Vec<CharChunk> = s
            .chars()
            .map(|content| {
                let chunk = CharChunk {
                    loc: Loc {
                        file: None,
                        line,
                        pos,
                    },
                    content,
                };

                if content == '\n' {
                    line += 1;
                }

                pos += 1;

                chunk
            })
            .collect();

        CharChunks {
            files: Vec::new(),
            content: chunks,
        }
    }

    pub fn from_string(s: String) -> Self {
        Self::from_str(&s)
    }
}

impl ops::Index<usize> for CharChunks {
    type Output = CharChunk;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.content[idx]
    }
}

impl iter::IntoIterator for CharChunks {
    type Item = CharChunk;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.content.into_iter()
    }
}

impl iter::FromIterator<CharChunk> for CharChunks {
    fn from_iter<I: iter::IntoIterator<Item = CharChunk>>(iter: I) -> Self {
        let mut chunks = CharChunks::new();

        for chunk in iter {
            chunks.push(chunk);
        }

        chunks
    }
}

impl convert::From<Vec<CharChunk>> for CharChunks {
    fn from(vchunks: Vec<CharChunk>) -> Self {
        let mut chunks = CharChunks::new();

        for chunk in vchunks {
            chunks.push(chunk);
        }

        chunks
    }
}

impl convert::From<String> for CharChunks {
    fn from(s: String) -> Self {
        CharChunks::from_string(s)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct StringChunks {
    pub files: Vec<String>,
    pub content: Vec<StringChunk>,
}

impl StringChunks {
    pub fn new() -> Self {
        StringChunks::default()
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, chunk: StringChunk) {
        let chunk_file = chunk.loc.file.clone();

        let has_file = self.files.iter().any(|file| {
            chunk
                .loc
                .file
                .as_ref()
                .map(|chunk_file| chunk_file == file)
                .unwrap_or(false)
        });

        if chunk_file.is_some() && !has_file {
            if let Some(chunk_file) = chunk_file {
                self.files.push(chunk_file);
            }
        }

        self.content.push(chunk)
    }

    pub fn from_chunk(chunk: StringChunk) -> Self {
        StringChunks {
            files: chunk
                .loc
                .file
                .clone()
                .map(|file| vec![file])
                .or_else(|| Some(vec![]))
                .unwrap(),
            content: vec![chunk],
        }
    }

    pub fn from_char_chunks(ccs: CharChunks) -> Self {
        let mut scs = StringChunks::new();
        let mut tmp_ccs: Vec<CharChunk> = vec![];
        let len = ccs.len();
        let mut idx = 0;

        while idx < len {
            let cc = ccs[idx].clone();
            let c = cc.content;

            if is_separator_char(c) {
                if !tmp_ccs.is_empty() {
                    let sc = StringChunk::from_char_chunks(tmp_ccs.clone());
                    scs.push(sc);

                    tmp_ccs = Vec::new();
                }

                let mut sc = StringChunk::from_char_chunk(cc);

                if c == COMMENT_MARK && idx < len && ccs[idx + 1].content == COMMENT_MARK_POSTFIX {
                    sc.content.push(ccs[idx + 1].content);
                    idx += 1;
                }

                scs.push(sc);

                idx += 1;
            } else {
                tmp_ccs.push(cc);

                idx += 1;
            }
        }

        if !tmp_ccs.is_empty() {
            let sc = StringChunk::from_char_chunks(tmp_ccs);
            scs.push(sc);
        }

        scs
    }

    pub fn from_str(s: &str) -> Self {
        StringChunks::from_char_chunks(CharChunks::from_str(s))
    }

    pub fn from_string(s: String) -> Self {
        Self::from_str(&s)
    }
}

impl ops::Index<usize> for StringChunks {
    type Output = StringChunk;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.content[idx]
    }
}

impl iter::IntoIterator for StringChunks {
    type Item = StringChunk;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.content.into_iter()
    }
}

impl iter::FromIterator<StringChunk> for StringChunks {
    fn from_iter<I: iter::IntoIterator<Item = StringChunk>>(iter: I) -> Self {
        let mut chunks = StringChunks::new();

        for chunk in iter {
            chunks.push(chunk);
        }

        chunks
    }
}

impl convert::From<Vec<StringChunk>> for StringChunks {
    fn from(vchunks: Vec<StringChunk>) -> Self {
        let mut chunks = StringChunks::new();

        for chunk in vchunks {
            chunks.push(chunk);
        }

        chunks
    }
}

impl convert::From<String> for StringChunks {
    fn from(s: String) -> Self {
        StringChunks::from_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn char_chunks_from_str() {
        use super::CharChunks;

        let s = "(include std.io)\n\n(printf \"hello world\\n\")";

        let chunks = CharChunks::from_str(s);

        assert_eq!(chunks.len(), s.len());
        assert_eq!(chunks.files.len(), 0);

        let chunk = &chunks[17];

        assert_eq!(chunk.content, '\n');
        assert_eq!(chunk.loc.file, None);
        assert_eq!(chunk.loc.line, 1);
        assert_eq!(chunk.loc.pos, 17);
    }

    #[test]
    fn string_chunks_from_str() {
        use super::StringChunks;

        let s = "(include std.io)\n\n(printf \"hello world\\n\")";

        let chunks = StringChunks::from_str(s);

        assert_eq!(chunks.len(), 16);
        assert_eq!(chunks.files.len(), 0);
    }
}
