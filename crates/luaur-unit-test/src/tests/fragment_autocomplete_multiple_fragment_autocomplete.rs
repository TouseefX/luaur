//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1643:fragment_autocomplete_multiple_fragment_autocomplete`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_multiple_fragment_autocomplete() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::enums::solver_mode::SolverMode;
    use luaur_analysis::functions::to_string_to_string_alt_b::to_string_type_id_to_string_options_mut;
    use luaur_analysis::records::scope::Scope;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_analysis::type_aliases::module_ptr_module::ModulePtr;
    use luaur_analysis::type_aliases::type_id::TypeId;
    use luaur_ast::records::position::Position;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;
    use luaur_common::FFlag;

    let mut opt = ToStringOptions::default();
    opt.exhaustive = true;
    opt.exhaustive = true;
    opt.function_type_arguments = true;
    opt.max_table_length = 0;
    opt.max_type_length = 0;

    fn get_type_from_module(module: &ModulePtr, name: &str) -> Option<TypeId> {
        if !module.has_module_scope() {
            return None;
        }
        let scope = module.get_module_scope();
        crate::functions::lookup_name::lookup_name(
            alloc::sync::Arc::as_ptr(&scope) as *mut Scope,
            &String::from(name),
        )
    }

    let source = String::from(
        r#"local module = {}
f
return module"#,
    );

    let updated1 = String::from(
        r#"local module = {}
function module.a
return module"#,
    );

    let updated2 = String::from(
        r#"local module = {}
function module.ab
return module"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();

    {
        let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
        fixture
            .base
            .base
            .get_frontend()
            .set_luau_solver_mode(SolverMode::Old);

        // checkAndExamine(source, "module", "{|  |}")
        fixture.base.check_with_options(&source);
        let id = fixture.base.base.base.get_type(&String::from("module"), true);
        LUAU_ASSERT!(id.is_some());
        assert_eq!(
            to_string_type_id_to_string_options_mut(id.unwrap(), opt.clone()),
            String::from("{|  |}")
        );

        // fragmentACAndCheck(updated1, Position{1, 17}, "module", "{|  |}", "{| a: (%error-id%: unknown) -> () |}")
        {
            let frag =
                fixture
                    .base
                    .autocomplete_fragment(&updated1, Position { line: 1, column: 17 }, None);
            LUAU_ASSERT!(frag.result.is_some());
            let frag_id = get_type_from_module(
                &frag.result.as_ref().unwrap().incremental_module,
                "module",
            );
            LUAU_ASSERT!(frag_id.is_some());
            assert_eq!(
                to_string_type_id_to_string_options_mut(frag_id.unwrap(), opt.clone()),
                String::from("{| a: (%error-id%: unknown) -> () |}")
            );

            let src_id = fixture.base.base.base.get_type(&String::from("module"), true);
            LUAU_ASSERT!(src_id.is_some());
            assert_eq!(
                to_string_type_id_to_string_options_mut(src_id.unwrap(), opt.clone()),
                String::from("{|  |}")
            );
        }

        // fragmentACAndCheck(updated2, Position{1, 18}, "module", "{|  |}", "{| ab: (%error-id%: unknown) -> () |}")
        {
            let frag =
                fixture
                    .base
                    .autocomplete_fragment(&updated2, Position { line: 1, column: 18 }, None);
            LUAU_ASSERT!(frag.result.is_some());
            let frag_id = get_type_from_module(
                &frag.result.as_ref().unwrap().incremental_module,
                "module",
            );
            LUAU_ASSERT!(frag_id.is_some());
            assert_eq!(
                to_string_type_id_to_string_options_mut(frag_id.unwrap(), opt.clone()),
                String::from("{| ab: (%error-id%: unknown) -> () |}")
            );

            let src_id = fixture.base.base.base.get_type(&String::from("module"), true);
            LUAU_ASSERT!(src_id.is_some());
            assert_eq!(
                to_string_type_id_to_string_options_mut(src_id.unwrap(), opt.clone()),
                String::from("{|  |}")
            );
        }
    }
    {
        let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
        fixture
            .base
            .base
            .get_frontend()
            .set_luau_solver_mode(SolverMode::New);

        // checkAndExamine(source, "module", "{  }")
        fixture.base.check_with_options(&source);
        let id = fixture.base.base.base.get_type(&String::from("module"), true);
        LUAU_ASSERT!(id.is_some());
        assert_eq!(
            to_string_type_id_to_string_options_mut(id.unwrap(), opt.clone()),
            String::from("{  }")
        );
        // [TODO] CLI-140762 Fragment autocomplete still doesn't return correct result when LuauSolverV2 is on
        // #if 0 (fragmentACAndCheck calls disabled in C++)
    }
}
