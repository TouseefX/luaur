//! Test fixture: faithful port of `compileFunction0Coverage` (tests/Compiler.test.cpp).
use luaur_ast::records::parse_options::ParseOptions;
use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;
use luaur_compiler::records::compile_options::CompileOptions;

pub fn compile_function_0_coverage(source: &str, level: i32) -> alloc::string::String {
    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE | BytecodeBuilder::DUMP_LINES);
    let source = alloc::string::String::from(source);
    let mut options = CompileOptions::default();
    options.coverage_level = level;
    let parse_options = ParseOptions::default();
    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );
    bcb.dump_function(0)
}
