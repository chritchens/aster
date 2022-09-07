use crate::loc::Loc;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Chunk {
    pub loc: Loc,
    pub content: char,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk::default()
    }
}
