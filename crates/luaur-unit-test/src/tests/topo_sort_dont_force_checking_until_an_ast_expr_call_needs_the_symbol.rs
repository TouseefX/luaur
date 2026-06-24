//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:239:topo_sort_dont_force_checking_until_an_ast_expr_call_needs_the_symbol`
//! Source: `tests/TopoSort.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TopoSort.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TopoSort.test.cpp
//! - outgoing:
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - calls -> method SubtypeFixture::obj (tests/Subtyping.test.cpp)
//!   - calls -> function toposort (tests/TopoSort.test.cpp)
//!   - translates_to -> rust_item topo_sort_dont_force_checking_until_an_ast_expr_call_needs_the_symbol

#[cfg(test)]
#[test]
fn topo_sort_dont_force_checking_until_an_ast_expr_call_needs_the_symbol() {
    use crate::functions::toposort::toposort;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let program = fixture.parse(
        r#"
        function A(obj)
            C(obj)
        end

        local B = A             -- It would be an error to force checking of A at this point just because the definition of B is an imperative

        function C(player)
        end

        local D = A(nil)        -- The real dependency on A is here, where A is invoked.
    "#,
        &ParseOptions::default(),
    );

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(4, sorted.len());

    let a = unsafe { *program.body.data.add(0) };
    let b = unsafe { *program.body.data.add(1) };
    let c = unsafe { *program.body.data.add(2) };
    let d = unsafe { *program.body.data.add(3) };

    assert_eq!(sorted[0], c);
    assert_eq!(sorted[1], a);
    assert_eq!(sorted[2], b);
    assert_eq!(sorted[3], d);
}
