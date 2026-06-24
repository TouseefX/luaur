//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5423:fragment_autocomplete_isinstance_refines_for_autocomplete`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_isinstance_refines_for_autocomplete() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;
    use luaur_common::FFlag;

    let _sff0 = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _sff1 = ScopedFastFlag::new(&FFlag::LuauAllowGlobalDeclarationToBeCalledClass, true);

    let source = String::from(
        r#"
class Point
    public x
    public y
end

local function f(v: Point | string)
    if class.isinstance(v, Point) then

    end
end
"#,
    );

    let dest = String::from(
        r#"
class Point
    public x
    public y
end

local function f(v: Point | string)
    if class.isinstance(v, Point) then
        v.@1
    end
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
        &source,
        &dest,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(frag.result.is_some());
            let ac = &frag.result.as_ref().unwrap().ac_results;
            assert!(ac.entry_map.contains_key("x"));
            assert!(ac.entry_map.contains_key("y"));
        }),
        None,
    );
}
