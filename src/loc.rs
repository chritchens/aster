use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Loc {
    pub file: Option<String>,
    pub line: usize,
    pub pos: usize,
}

impl Loc {
    pub fn new() -> Self {
        Loc::default()
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        let file = self.file.clone().unwrap_or_else(|| "none".into());
        format!("(file: {}, line: {}, pos: {})", file, self.line, self.pos)
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file = self.file.clone().unwrap_or_else(|| "none".into());
        write!(
            f,
            "(file: {}, line: {}, pos: {})",
            file, self.line, self.pos
        )
    }
}
