//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3020:fragment_autocomplete_fragment_autocomplete_ensures_memory_isolation`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_fragment_autocomplete_ensures_memory_isolation() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::enums::solver_mode::SolverMode;
    use luaur_analysis::functions::to_string_to_string_alt_b::to_string_type_id_to_string_options_mut;
    use luaur_analysis::records::scope::Scope;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_analysis::records::type_arena::TypeArena;
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

    let mut check_and_examine =
        |fixture: &mut FragmentAutocompleteFixture, src: &String, id_name: &str, id_string: &str| {
            fixture.base.check_with_options(src);
            let id = fixture
                .base
                .base
                .base
                .get_type(&String::from(id_name), true);
            LUAU_ASSERT!(id.is_some());
            assert_eq!(
                to_string_type_id_to_string_options_mut(id.unwrap(), opt.clone()),
                String::from(id_string)
            );
        };

    let fragment_ac_and_check =
        |fixture: &mut FragmentAutocompleteFixture, updated: &String, pos: Position, id_name: &str| {
            let frag = fixture.base.autocomplete_fragment(updated, pos, None);
            LUAU_ASSERT!(frag.result.is_some());
            let frag_id = get_type_from_module(
                &frag.result.as_ref().unwrap().incremental_module,
                id_name,
            );
            LUAU_ASSERT!(frag_id.is_some());

            let src_id = fixture
                .base
                .base
                .base
                .get_type(&String::from(id_name), true);
            LUAU_ASSERT!(src_id.is_some());

            let frag_id = frag_id.unwrap();
            let src_id = src_id.unwrap();
            unsafe {
                assert!((*frag_id).owning_arena != (*src_id).owning_arena);
                let internal_types_ptr = &frag.result.as_ref().unwrap().incremental_module.internal_types
                    as *const TypeArena as *mut TypeArena;
                assert!(internal_types_ptr == (*frag_id).owning_arena);
            }
        };

    {
        let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
        fixture.base.base.get_frontend().set_luau_solver_mode(SolverMode::Old);
        check_and_examine(&mut fixture, &source, "module", "{|  |}");
        // [TODO] CLI-140762 we shouldn't mutate stale module in autocompleteFragment
        // early return since the following checking will fail, which it shouldn't!
        fragment_ac_and_check(&mut fixture, &updated1, Position { line: 1, column: 17 }, "module");
        fragment_ac_and_check(&mut fixture, &updated2, Position { line: 1, column: 18 }, "module");
    }

    {
        let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
        fixture.base.base.get_frontend().set_luau_solver_mode(SolverMode::New);
        check_and_examine(&mut fixture, &source, "module", "{  }");
        // [TODO] CLI-140762 we shouldn't mutate stale module in autocompleteFragment
        // early return since the following checking will fail, which it shouldn't!
        fragment_ac_and_check(&mut fixture, &updated1, Position { line: 1, column: 17 }, "module");
        fragment_ac_and_check(&mut fixture, &updated2, Position { line: 1, column: 18 }, "module");
    }
}
