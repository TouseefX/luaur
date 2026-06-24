//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1328:frontend_attribute_ices_to_the_correct_module`
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
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - type_ref -> record InternalCompilerError (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item frontend_attribute_ices_to_the_correct_module

#[cfg(test)]
#[test]
fn frontend_attribute_ices_to_the_correct_module() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::internal_compiler_error::InternalCompilerError;
    use luaur_common::FFlag;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    let _magic_types = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true);
    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("game/one"),
        String::from(
            r#"
        require(game.two)
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("game/two"),
        String::from(
            r#"
        local a: _luau_ice
    "#,
        ),
    );

    let result = catch_unwind(AssertUnwindSafe(|| {
        fixture
            .get_frontend()
            .check_module_name_optional_frontend_options(&String::from("game/one"), None);
    }));

    let panic = result.expect_err("expected an InternalCompilerError");
    let ice = panic
        .downcast_ref::<InternalCompilerError>()
        .expect("expected InternalCompilerError panic payload");
    assert_eq!(Some(String::from("game/two")), ice.module_name.clone());
}
