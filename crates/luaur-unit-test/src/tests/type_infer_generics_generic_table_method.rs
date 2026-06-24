//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1174:type_infer_generics_generic_table_method`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_generics_generic_table_method

#[cfg(test)]
#[test]
fn type_infer_generics_generic_table_method() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::flatten_type_pack::flatten_type_pack_id;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::generic_type::GenericType;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local T = {}

        function T:bar(i)
            return i
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let t_type = fixture.require_type_string(&String::from("T"));
    let t_table = unsafe { get_type_id::<TableType>(t_type).as_ref() }.expect("expected TableType");

    let bar = t_table
        .props
        .get(&String::from("bar"))
        .expect("expected bar property");
    let bar_type = bar.read_ty.expect("expected readable bar type");
    assert!(!bar_type.is_null());

    let bar_type = unsafe { follow_type_id(bar_type) };
    let ftv = unsafe { get_type_id::<FunctionType>(bar_type).as_ref() }.unwrap_or_else(|| {
        panic!("should be a function: {}", unsafe {
            luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id(bar_type)
        })
    });

    let (args, _) = flatten_type_pack_id(ftv.arg_types());
    let arg_type = *args.get(1).expect("expected self and method parameter");

    assert!(
        !unsafe { get_type_id::<GenericType>(arg_type) }.is_null(),
        "should be generic: {}",
        luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id(bar_type)
    );
}
