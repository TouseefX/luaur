//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1473:fragment_autocomplete_can_typecheck_simple_fragment`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_can_typecheck_simple_fragment() {
    use crate::functions::linear_search_for_binding::linear_search_for_binding;
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FragmentAutocompleteFixture::default();
    let res = fixture.base.check_with_options(&String::from(
        r#"
local x = 4
local y = 5
"#,
    ));

    assert_eq!(0, res.errors.len(), "{:?}", res.errors);

    let fragment = fixture.base.check_fragment(
        &String::from(
            r#"
local x = 4
local y = 5
local z = x + y
"#,
        ),
        Position {
            line: 3,
            column: 15,
        },
        None,
    );

    let scope_ptr = alloc::sync::Arc::as_ptr(&fragment.fresh_scope)
        as *mut luaur_analysis::records::scope::Scope;
    let opt = linear_search_for_binding(scope_ptr, "z");
    LUAU_ASSERT!(opt.is_some());
    assert_eq!("number", to_string_type_id(opt.unwrap()));
}
