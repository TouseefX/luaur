#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ReportFormat {
    #[default]
    Default,
    Luacheck,
    Gnu,
}
