use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name_table::AstNameTable;

#[derive(Debug)]
#[repr(C)]
pub struct JsonEncoderFixture {
    pub allocator: Allocator,
    pub names: AstNameTable,
}

impl JsonEncoderFixture {
    pub fn new() -> Self {
        let mut allocator = Allocator::allocator();
        let names = AstNameTable::new(&mut allocator);
        Self { allocator, names }
    }
}
