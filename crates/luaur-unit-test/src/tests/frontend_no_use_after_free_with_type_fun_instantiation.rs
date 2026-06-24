//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1165:frontend_no_use_after_free_with_type_fun_instantiation`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record FrontendFixture (tests/Frontend.test.cpp)
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - translates_to -> rust_item frontend_no_use_after_free_with_type_fun_instantiation

#[cfg(test)]
#[test]
fn frontend_no_use_after_free_with_type_fun_instantiation() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _freeze_arena = ScopedFastFlag::new(&FFlag::DebugLuauFreezeArena, true);

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("Module/A"),
        String::from(
            r#"
export type Foo<V> = typeof(setmetatable({}, {}))
return false;
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("Module/B"),
        String::from(
            r#"
local A = require(script.Parent.A)
export type Foo<V> = A.Foo<V>
return false;
    "#,
        ),
    );

    // We don't care about the result. That we haven't crashed is enough.
    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/B"), None);
}
