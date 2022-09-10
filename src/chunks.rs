use crate::chunk::Chunk;
use crate::loc::Loc;
use std::convert;
use std::iter;
use std::ops;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Chunks {
    pub files: Vec<String>,
    pub content: Vec<Chunk>,
}

impl Chunks {
    pub fn new() -> Self {
        Chunks::default()
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, chunk: Chunk) {
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

    pub fn from_chunk(chunk: Chunk) -> Self {
        Chunks {
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

        let chunks: Vec<Chunk> = s
            .chars()
            .map(|content| {
                let chunk = Chunk {
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

        Chunks {
            files: Vec::new(),
            content: chunks,
        }
    }

    pub fn from_string(s: String) -> Self {
        Self::from_str(&s)
    }
}

impl ops::Index<usize> for Chunks {
    type Output = Chunk;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.content[idx]
    }
}

impl iter::IntoIterator for Chunks {
    type Item = Chunk;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.content.into_iter()
    }
}

impl iter::FromIterator<Chunk> for Chunks {
    fn from_iter<I: iter::IntoIterator<Item = Chunk>>(iter: I) -> Self {
        let mut chunks = Chunks::new();

        for chunk in iter {
            chunks.push(chunk);
        }

        chunks
    }
}

impl convert::From<Vec<Chunk>> for Chunks {
    fn from(vchunks: Vec<Chunk>) -> Self {
        let mut chunks = Chunks::new();

        for chunk in vchunks {
            chunks.push(chunk);
        }

        chunks
    }
}

impl convert::From<String> for Chunks {
    fn from(s: String) -> Self {
        Chunks::from_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn chunks_from_str() {
        use super::Chunks;

        let s = "(include std.io)\n\n(printf \"hello world\\n\")";

        let chunks = Chunks::from_str(s);

        assert_eq!(chunks.len(), s.len());
        assert_eq!(chunks.files.len(), 0);

        let chunk = &chunks[17];

        assert_eq!(chunk.content, '\n');
        assert_eq!(chunk.loc.file, None);
        assert_eq!(chunk.loc.line, 1);
        assert_eq!(chunk.loc.pos, 17);
    }
}
