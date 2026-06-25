//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1608:fragment_autocomplete_typecheck_fragment_handles_unusable_module`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_typecheck_fragment_handles_unusable_module() {
    use crate::functions::get_options::get_options;
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::enums::fragment_type_check_status::FragmentTypeCheckStatus;
    use luaur_ast::records::position::Position;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;
    use luaur_common::FFlag;

    let source_a = String::from("MainModule");
    let source_b = String::from("game/Gui/Modules/B");

    let source_a_text = String::from(
        r#"
local Modules = game:GetService('Gui').Modules
local B = require(Modules.B)
return { hello = B }
"#,
    );
    let source_b_text = String::from(r#"return {hello = "hello"}"#);

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture
        .base
        .base
        .base
        .file_resolver
        .source
        .insert(source_a.clone(), source_a_text.clone());
    fixture
        .base
        .base
        .base
        .file_resolver
        .source
        .insert(source_b.clone(), source_b_text.clone());

    let for_autocomplete = get_options().for_autocomplete;

    {
        let frontend = fixture.base.base.get_frontend();
        let _result =
            frontend.check_module_name_optional_frontend_options(&source_a, Some(get_options()));
        assert!(!frontend.is_dirty(&source_a, for_autocomplete));
    }

    // C++ `getModuleResolver(frontend)` selects `moduleResolver` under the new
    // solver and `moduleResolverForAutocomplete` under the old solver. The
    // weak_ptr / expired() semantics map to `Arc::downgrade` / `upgrade`.
    let weak_module = {
        let frontend = fixture.base.base.get_frontend();
        let resolver = if !FFlag::DebugLuauForceOldSolver.get() {
            &frontend.module_resolver
        } else {
            &frontend.module_resolver_for_autocomplete
        };
        let module = resolver.modules.get(&source_b);
        LUAU_ASSERT!(module.is_some());
        alloc::sync::Arc::downgrade(module.unwrap())
    };
    // `REQUIRE(!weakModule.expired())`
    assert!(weak_module.upgrade().is_some());

    {
        let frontend = fixture.base.base.get_frontend();
        frontend.mark_dirty(&source_b, None);
        assert!(frontend.is_dirty(&source_a, for_autocomplete));

        frontend.check_module_name_optional_frontend_options(&source_b, Some(get_options()));
    }
    // `CHECK(weakModule.expired())`
    assert!(weak_module.upgrade().is_none());

    let (status, _) = fixture.base.typecheck_fragment_for_module(
        &source_a,
        &source_a_text,
        Position { line: 0, column: 0 },
        None,
    );
    assert_eq!(FragmentTypeCheckStatus::SkipAutocomplete, status);

    let (status2, _2) = fixture.base.typecheck_fragment_for_module(
        &source_b,
        &source_b_text,
        Position {
            line: 3,
            column: 20,
        },
        None,
    );
    assert_eq!(FragmentTypeCheckStatus::Success, status2);
}
