//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.primitives.test.cpp:94:type_infer_primitives_singleton_types`
//! Source: `tests/TypeInfer.primitives.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.primitives.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.primitives.test.cpp
//! - outgoing:
//!   - type_ref -> record BuiltinsFixture (tests/Fixture.h)
//!   - type_ref -> record Frontend (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
//!   - translates_to -> rust_item type_infer_primitives_singleton_types

#[cfg(test)]
#[test]
fn type_infer_primitives_singleton_types() {
    use crate::records::builtins_fixture::BuiltinsFixture;

    let mut fixture_a = BuiltinsFixture::default();
    fixture_a.get_frontend();

    {
        let mut fixture_b = BuiltinsFixture::default();
        fixture_b.get_frontend();
    }

    let result = fixture_a.base.check_string_optional_frontend_options(
        &String::from("local s: string = 'hello' local t = s:lower()"),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
}
