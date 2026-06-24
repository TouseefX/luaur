use crate::records::test_file_resolver::TestFileResolver;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name_table::AstNameTable;

#[derive(Debug)]
#[repr(C)]
pub struct RequireTracerFixture {
    pub allocator: Box<Allocator>,
    pub names: Box<AstNameTable>,
    pub file_resolver: TestFileResolver,
}
