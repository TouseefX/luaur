use luaur_code_gen::records::assembly_options::AssemblyOptions;
use luaur_compiler::records::compile_options::CompileOptions;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct LoweringFixture {
    pub compilation_options: CompileOptions,
    pub compilation_options_c: luaur_compiler::records::compile_options::CompileOptions,
    pub assembly_options: AssemblyOptions,
}

impl Default for LoweringFixture {
    fn default() -> Self {
        let mut compilation_options = CompileOptions::default();
        compilation_options.optimization_level = 2;
        compilation_options.debug_level = 1;
        compilation_options.type_info_level = 1;

        let mut compilation_options_c = CompileOptions::default();
        compilation_options_c.optimization_level = 2;
        compilation_options_c.debug_level = 1;
        compilation_options_c.type_info_level = 1;

        Self {
            compilation_options,
            compilation_options_c,
            assembly_options: unsafe { core::mem::zeroed() },
        }
    }
}
