use crate::chunk::Chunk;
use crate::loc::Loc;
use std::ops;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Chunks {
    pub files: Vec<String>,
    pub lines: usize,
    pub content: Vec<Chunk>,
}

impl Chunks {
    pub fn new() -> Self {
        Chunks::default()
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
            lines: chunk.loc.line,
            content: vec![chunk],
        }
    }

    pub fn push(&mut self, chunk: Chunk) {
        if self
            .files
            .iter()
            .find(|file| {
                if let Some(ref chunk_file) = chunk.loc.file {
                    &chunk_file == file
                } else {
                    false
                }
            })
            .is_none()
        {
            if let Some(chunk_file) = chunk.loc.file.clone() {
                self.files.push(chunk_file);
            }

            self.lines += chunk.loc.line + 1;
        } else if self.lines != chunk.loc.line + 1 {
            self.lines += chunk.loc.line + 1;
        }

        self.content.push(chunk)
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
            lines: line + 1,
            content: chunks,
        }
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }
}

impl ops::Index<usize> for Chunks {
    type Output = Chunk;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.content[idx]
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
        assert_eq!(chunks.lines, 3);

        let chunk = &chunks[17];

        assert_eq!(chunk.content, '\n');
        assert_eq!(chunk.loc.file, None);
        assert_eq!(chunk.loc.line, 1);
        assert_eq!(chunk.loc.pos, 17);
    }
}
