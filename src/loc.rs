#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Loc {
    pub file: Option<String>,
    pub line: usize,
    pub pos: usize,
}
