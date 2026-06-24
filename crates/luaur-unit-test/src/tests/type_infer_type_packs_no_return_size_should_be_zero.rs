//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:119:type_infer_type_packs_no_return_size_should_be_zero`
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
//!   - translates_to -> rust_item type_infer_type_packs_no_return_size_should_be_zero

#[cfg(test)]
#[test]
fn type_infer_type_packs_no_return_size_should_be_zero() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::flatten_type_pack::flatten_type_pack_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::function_type::FunctionType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(a:any) return a end
        function g() return end
        function h() end

        g(h())
        f(g(),h())
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let f_type = fixture.require_type_string(&String::from("f"));
    let f_type = unsafe { get_type_id::<FunctionType>(f_type).as_ref() }
        .expect("expected FunctionType for f");
    assert_eq!(1, flatten_type_pack_id(f_type.ret_types()).0.len());

    let g_type = fixture.require_type_string(&String::from("g"));
    let g_type = unsafe { get_type_id::<FunctionType>(g_type).as_ref() }
        .expect("expected FunctionType for g");
    assert_eq!(0, flatten_type_pack_id(g_type.ret_types()).0.len());

    let h_type = fixture.require_type_string(&String::from("h"));
    let h_type = unsafe { get_type_id::<FunctionType>(h_type).as_ref() }
        .expect("expected FunctionType for h");
    assert_eq!(0, flatten_type_pack_id(h_type.ret_types()).0.len());
}
