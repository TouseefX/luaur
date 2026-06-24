//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:105:type_infer_infer_locals_with_nil_value`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method Fixture::getPrimitiveType (tests/Fixture.cpp)
//!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_infer_locals_with_nil_value

#[cfg(test)]
#[test]
fn type_infer_infer_locals_with_nil_value() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::primitive_type::PrimitiveType;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from("local f = nil; f = 'hello world'"),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let f_type = fixture.require_type_string(&String::from("f"));
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!("string?", to_string_type_id(f_type));
    } else {
        assert_eq!(
            Some(PrimitiveType::String),
            fixture.get_primitive_type(f_type)
        );
    }
}
