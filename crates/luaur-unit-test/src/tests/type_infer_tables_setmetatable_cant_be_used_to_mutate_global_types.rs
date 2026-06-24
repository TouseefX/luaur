//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3158:type_infer_tables_setmetatable_cant_be_used_to_mutate_global_types`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> record Fixture (tests/Fixture.h)
//!   - calls -> method DfgScope::inherit (Analysis/src/DataFlowGraph.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method BytecodeBuilder::validate (Bytecode/src/BytecodeBuilder.cpp)
//!   - translates_to -> rust_item type_infer_tables_setmetatable_cant_be_used_to_mutate_global_types

#[cfg(test)]
#[test]
fn type_infer_tables_setmetatable_cant_be_used_to_mutate_global_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let parent_global_scope = fixture.get_frontend().globals.global_scope();

    {
        let mut fix = Fixture::fixture_bool(false);
        fix.get_frontend()
            .globals
            .set_global_scope(parent_global_scope.clone());

        let _ = fix.check_string_optional_frontend_options(
            &String::from(
                r#"
--!nonstrict
type MT = typeof(setmetatable)
function wtf(arg: {MT}): typeof(table)
    arg = wtf(arg)
end
"#,
            ),
            None,
        );
    }

    let frontend = fixture.get_frontend();
    let global_scope = frontend.globals.global_scope();
    for binding in global_scope.bindings.values() {
        let _ = to_string_type_id(binding.type_id);
    }
}
