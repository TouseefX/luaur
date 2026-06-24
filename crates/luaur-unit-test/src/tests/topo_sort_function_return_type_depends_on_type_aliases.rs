//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:340:topo_sort_function_return_type_depends_on_type_aliases`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function toposort (tests/TopoSort.test.cpp)
//!   - translates_to -> rust_item topo_sort_function_return_type_depends_on_type_aliases

#[cfg(test)]
#[test]
fn topo_sort_function_return_type_depends_on_type_aliases() {
    use crate::functions::toposort::toposort;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let program = fixture.parse(
        r#"
        type callbackFn<K, V> = (element: V, key: K, map: Map<K, V>) -> ()

        export type Map<K, V> = {
            forEach: (callback: callbackFn<K, V>) -> (),
        }

        function foo<K, V>(key: K, value: V): Map<K, V>
        end
    "#,
        &ParseOptions::default(),
    );

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(3, sorted.len());

    let callback_fn = unsafe { *program.body.data.add(0) };
    let map = unsafe { *program.body.data.add(1) };
    let foo = unsafe { *program.body.data.add(2) };

    assert_eq!(sorted[0], callback_fn);
    assert_eq!(sorted[1], map);
    assert_eq!(sorted[2], foo);
}
