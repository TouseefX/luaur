//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:73:type_infer_tables_basic`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
//!   - calls -> method Fixture::getPrimitiveType (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_infer_tables_basic

#[cfg(test)]
#[test]
fn type_infer_tables_basic() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::primitive_type::PrimitiveType;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from("local t = {foo = \"bar\", baz = 9, quux = nil}"),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let t_type = unsafe {
        get_type_id::<TableType>(fixture.require_type_string(&String::from("t"))).as_ref()
    }
    .expect("expected table type");

    let foo_ty = t_type
        .props
        .get("foo")
        .and_then(|prop| prop.read_ty)
        .expect("expected foo read type");
    assert_eq!(
        Some(PrimitiveType::String),
        fixture.get_primitive_type(foo_ty)
    );

    let baz_ty = t_type
        .props
        .get("baz")
        .and_then(|prop| prop.read_ty)
        .expect("expected baz read type");
    assert_eq!(
        Some(PrimitiveType::Number),
        fixture.get_primitive_type(baz_ty)
    );

    let quux_ty = t_type
        .props
        .get("quux")
        .and_then(|prop| prop.read_ty)
        .expect("expected quux read type");
    assert_eq!(
        Some(PrimitiveType::NilType),
        fixture.get_primitive_type(quux_ty)
    );
}
