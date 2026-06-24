use crate::records::repl_requirer::ReplRequirer;
use crate::type_aliases::bool_check::BoolCheck;
use crate::type_aliases::compile_options::CompileOptions;
use crate::type_aliases::coverage::Coverage;

pub fn repl_requirer_repl_requirer(
    copts: CompileOptions,
    coverage_active: BoolCheck,
    codegen_enabled: BoolCheck,
    coverage_track: Coverage,
    counters_active: BoolCheck,
    counters_track: Coverage,
) -> ReplRequirer {
    ReplRequirer {
        copts,
        coverageActive: coverage_active,
        codegenEnabled: codegen_enabled,
        coverageTrack: coverage_track,
        countersActive: counters_active,
        countersTrack: counters_track,
        vfs: Default::default(),
    }
}
