//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4962:type_infer_tables_instantiated_metatable_frozen_table_clone_mutation`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_tables_instantiated_metatable_frozen_table_clone_mutation

#[cfg(test)]
#[test]
fn type_infer_tables_instantiated_metatable_frozen_table_clone_mutation() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/worker"),
        String::from(
            r#"
type WorkerImpl<T..., R...> = {
    destroy: (self: Worker<T..., R...>) -> boolean,
}

type WorkerProps = { id: number }

export type Worker<T..., R...> = typeof(setmetatable({} :: WorkerProps, {} :: WorkerImpl<T..., R...>))

return {}
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/library"),
        String::from(
            r#"
local Worker = require(game.worker)

export type Worker<T..., R...> = Worker.Worker<T..., R...>

return {}
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/library"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
