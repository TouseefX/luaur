//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:18:type_infer_type_packs_infer_multi_return`
//! Source: `tests/TypeInfer.typePacks.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_type_packs_infer_multi_return

#[cfg(test)]
#[test]
fn type_infer_type_packs_infer_multi_return() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::flatten_type_pack::flatten_type_pack_id;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::function_type::FunctionType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function take_two()
            return 2, 2
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let take_two_type = fixture.require_type_string(&String::from("take_two"));
    let take_two_type = unsafe { get_type_id::<FunctionType>(take_two_type).as_ref() }
        .expect("expected FunctionType");
    let (returns, tail) = flatten_type_pack_id(take_two_type.ret_types());

    assert_eq!(2, returns.len());
    assert_eq!(
        "number",
        to_string_type_id(unsafe { follow_type_id(returns[0]) })
    );
    assert_eq!(
        "number",
        to_string_type_id(unsafe { follow_type_id(returns[1]) })
    );
    assert!(tail.is_none());
}
