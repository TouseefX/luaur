use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_table::DenseDefault;

/// C++ `LintUnusedFunction::Global` (`Analysis/src/Linter.cpp:960`).
///
/// ```cpp
/// struct Global
/// {
///     Location location;
///     bool function;
///     bool used;
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct Global {
    pub(crate) location: Location,
    pub(crate) function: bool,
    pub(crate) used: bool,
}

impl DenseDefault for Global {
    fn dense_default() -> Self {
        Self::default()
    }
}
