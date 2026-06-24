//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.singletons.test.cpp:599:type_infer_singletons_indexing_on_union_of_string_singletons`
//! Source: `tests/TypeInfer.singletons.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.singletons.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.singletons.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_singletons_indexing_on_union_of_string_singletons

#[cfg(test)]
#[test]
fn type_infer_singletons_indexing_on_union_of_string_singletons() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a: string = "hi"
        if a == "hi" or a == "bye" then
            local x = a:byte()
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        r#""bye" | "hi""#,
        to_string_type_id(fixture.require_type_at_position_position(Position {
            line: 3,
            column: 22
        }))
    );
}
