use luaur_ast::records::location::Location;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Statement {
    pub(crate) start: Location,
    pub(crate) lastLine: u32,
    pub(crate) flagged: bool,
}

impl Default for Statement {
    fn default() -> Self {
        Self {
            start: Location::default(),
            lastLine: 0,
            flagged: false,
        }
    }
}

#[allow(non_snake_case)]
impl Statement {
    pub const fn lastLine(&self) -> u32 {
        self.lastLine
    }
}
