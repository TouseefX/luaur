//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1205:type_infer_generics_correctly_instantiate_polymorphic_member_functions`
//! Source: `tests/TypeInfer.generics.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.generics.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.generics.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> method Fixture::getPrimitiveType (tests/Fixture.cpp)
//!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_generics_correctly_instantiate_polymorphic_member_functions

#[cfg(test)]
#[test]
fn type_infer_generics_correctly_instantiate_polymorphic_member_functions() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::primitive_type::PrimitiveType;
    use luaur_analysis::records::table_type::TableType;
    use luaur_common::FFlag;

    let _assert_on_forced_constraint =
        ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local T = {}

        function T:foo()
            return T:bar(5)
        end

        function T:bar(i)
            return i
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let t = unsafe {
        get_type_id::<TableType>(fixture.require_type_string(&String::from("T"))).as_ref()
    }
    .expect("expected TableType");

    let foo_prop = t
        .props
        .get(&String::from("foo"))
        .expect("expected foo property");
    let foo_ty = foo_prop.read_ty.expect("expected readable foo type");
    let foo_ty = unsafe { follow_type_id(foo_ty) };
    let foo = unsafe { get_type_id::<FunctionType>(foo_ty).as_ref() }
        .expect("expected FunctionType for foo");

    let ret = first(foo.ret_types(), false).expect("expected return type");
    let ret = unsafe { follow_type_id(ret) };

    assert_eq!(Some(PrimitiveType::Number), fixture.get_primitive_type(ret));
}
