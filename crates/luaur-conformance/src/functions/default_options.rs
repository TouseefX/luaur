use luaur_compiler::records::compile_options::CompileOptions;

pub fn default_options() -> CompileOptions {
    let mut copts = CompileOptions::default();
    copts.optimization_level = 1;
    copts.debug_level = 1;
    copts.type_info_level = 1;

    copts
}
