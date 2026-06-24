use crate::records::require_tracer_fixture::RequireTracerFixture;
use crate::records::test_file_resolver::TestFileResolver;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name_table::AstNameTable;

pub fn require_tracer_fixture_require_tracer_fixture() -> RequireTracerFixture {
    let mut allocator = Box::new(Allocator::allocator());
    let names = Box::new(AstNameTable::new(&mut allocator));

    RequireTracerFixture {
        allocator,
        names,
        file_resolver: TestFileResolver::default(),
    }
}
