//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1635:type_infer_typeof_cannot_refine_builtin_alias`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - type_ref -> record GlobalTypes (Analysis/include/Luau/GlobalTypes.h)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record TypeFun (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_typeof_cannot_refine_builtin_alias

#[cfg(test)]
#[test]
fn type_infer_typeof_cannot_refine_builtin_alias() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use alloc::sync::Arc;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::functions::freeze::freeze;
    use luaur_analysis::functions::unfreeze::unfreeze;
    use luaur_analysis::records::scope::Scope;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_fun::TypeFun;
    use luaur_analysis::records::type_level::TypeLevel;

    let mut fixture = Fixture::default();
    let frontend = fixture.get_frontend();

    unsafe {
        let global_scope = frontend.globals.global_scope();
        let global_scope_ptr = Arc::as_ptr(&global_scope) as *mut Scope;
        let arena = frontend.globals.global_types_mut();

        unfreeze(arena);

        let global_table_ty = arena.add_type(TableType::table_type_table_state_type_level_scope(
            TableState::Sealed,
            TypeLevel::default(),
            core::ptr::null_mut(),
        ));

        (*global_scope_ptr).exported_type_bindings.insert(
            String::from("GlobalTable"),
            TypeFun::type_fun_type_id(global_table_ty),
        );

        freeze(arena);
    }

    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function foo(x)
            if typeof(x) == 'GlobalTable' then
            end
        end
    "#,
        ),
        None,
    );
}
