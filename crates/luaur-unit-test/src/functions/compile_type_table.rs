//! Test fixture: faithful port of `compileTypeTable` (tests/Compiler.test.cpp).
use luaur_ast::records::parse_options::ParseOptions;
use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;
use luaur_compiler::records::compile_options::CompileOptions;

pub fn compile_type_table(source: &str) -> alloc::string::String {
    let mut bcb = BytecodeBuilder::new(None);
    bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);
    let mut options = CompileOptions::default();
    options.vector_type = c"Vector3".as_ptr();
    options.type_info_level = 1;
    let source = alloc::string::String::from(source);
    let parse_options = ParseOptions::default();
    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bcb,
        &source,
        &options,
        &parse_options,
    );
    bcb.dump_type_info()
}
