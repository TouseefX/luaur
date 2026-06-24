//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1873:frontend_queue_check_propagates_ice`
//! Source: `tests/Frontend.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Frontend.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//! - incoming:
//!   - declares <- source_file tests/Frontend.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> function fromString (tests/Fixture.cpp)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
//!   - calls -> method Frontend::checkQueuedModules (Analysis/src/Frontend.cpp)
//!   - type_ref -> record InternalCompilerError (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item frontend_queue_check_propagates_ice

#[cfg(test)]
#[test]
fn frontend_queue_check_propagates_ice() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::internal_compiler_error::InternalCompilerError;
    use luaur_common::FFlag;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    let _magic_types = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true);
    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    let module_name = String::from("MainModule");
    fixture.base.base.file_resolver.source.insert(
        module_name.clone(),
        String::from(
            r#"
        --!strict
        local a: _luau_ice = 55
    "#,
        ),
    );
    fixture.get_frontend().mark_dirty(&module_name, None);
    fixture
        .get_frontend()
        .queue_module_check_module_name(&String::from("MainModule"));

    let result = catch_unwind(AssertUnwindSafe(|| {
        fixture.get_frontend().check_queued_modules(
            None,
            Box::new(|tasks| {
                for task in tasks {
                    task();
                }
            }),
            Box::new(|_, _| true),
        );
    }));

    let panic = result.expect_err("expected InternalCompilerError");
    assert!(
        panic.downcast_ref::<InternalCompilerError>().is_some(),
        "expected InternalCompilerError panic payload"
    );
}
