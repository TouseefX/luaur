//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5221:autocomplete_autocomplete_react`
//! Source: `tests/Autocomplete.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Autocomplete.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
//!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Common/include/Luau/StringUtils.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Autocomplete.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> enum State (Analysis/src/TypePath.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item autocomplete_autocomplete_react

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_react() {
    use crate::records::ac_fixture::AcFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        type React_Node = any
        type ReactElement<P, T> = any

        type React_StatelessFunctionalComponent<Props> = (props: Props, context: any) -> React_Node
        type React_Component<Props, State = nil> = {}
        type createElementFn = <P, T>(
            type_:
              | React_StatelessFunctionalComponent<P>
              | React_Component<P>
              | string,
            props: P?,
            ...(React_Node | (...any) -> React_Node)
        ) -> ReactElement<P, T>

        local createElement: createElementFn = nil :: any

        local function MyComponent(props: { foobar: string, barbaz: { bazquxx: string } })
        	return nil
        end

        createElement(MyComponent, { f@1 })
        createElement(MyComponent, { barbaz = { b@2 } })
        createElement(MyComponent, { foobar = {}, b@3 })
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac.entry_map.contains_key("foobar"));

    let ac = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);
    assert!(ac.entry_map.contains_key("bazquxx"));

    let ac = fixture.base.autocomplete_marker(b'3' as core::ffi::c_char);
    assert!(ac.entry_map.contains_key("barbaz"));
}
