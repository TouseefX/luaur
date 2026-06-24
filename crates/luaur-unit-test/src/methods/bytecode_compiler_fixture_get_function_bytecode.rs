use crate::records::bytecode_compiler_fixture::BytecodeCompilerFixture;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name_table::AstNameTable;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_ast::records::parser::Parser;
use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;
use luaur_compiler::functions::compile_or_throw_compiler::compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options;
use luaur_compiler::records::compile_options::CompileOptions;

impl BytecodeCompilerFixture {
    pub fn get_function_bytecode(
        &mut self,
        src: &str,
        optimization_level: i32,
    ) -> Option<(String, Vec<String>)> {
        let mut allocator = Allocator::allocator();
        let mut names = AstNameTable::new(&mut allocator);
        let result = Parser::parse(
            src,
            src.len(),
            &mut names,
            &mut allocator,
            ParseOptions::default(),
        );

        if !result.errors.is_empty() {
            return None;
        }

        let mut bcb = BytecodeBuilder::new(None);
        bcb.set_dump_flags(BytecodeBuilder::DUMP_CODE);

        let mut opts = CompileOptions::default();
        opts.optimization_level = optimization_level;

        let compile_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            compile_or_throw_bytecode_builder_parse_result_ast_name_table_compile_options(
                &mut bcb, &result, &mut names, &opts,
            );
        }));

        if compile_result.is_err() {
            return None;
        }

        Some((bcb.get_function_data(0), self.extract_string_table(&bcb)))
    }
}
