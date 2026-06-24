use crate::type_aliases::bool_check::BoolCheck;
use crate::type_aliases::compile_options::CompileOptions;
use crate::type_aliases::coverage::Coverage;
use luaur_cli_lib::records::vfs_navigator::VfsNavigator;

#[derive(Debug, Clone)]
pub struct ReplRequirer {
    pub(crate) copts: CompileOptions,
    pub(crate) coverageActive: BoolCheck,
    pub(crate) codegenEnabled: BoolCheck,
    pub(crate) coverageTrack: Coverage,
    pub(crate) countersActive: BoolCheck,
    pub(crate) countersTrack: Coverage,
    pub(crate) vfs: VfsNavigator,
}

#[allow(non_snake_case)]
impl ReplRequirer {
    pub(crate) fn coverageActive(&self) -> bool {
        unsafe { (self.coverageActive)() }
    }

    pub(crate) fn codegenEnabled(&self) -> bool {
        unsafe { (self.codegenEnabled)() }
    }

    pub(crate) fn countersActive(&self) -> bool {
        unsafe { (self.countersActive)() }
    }
}
