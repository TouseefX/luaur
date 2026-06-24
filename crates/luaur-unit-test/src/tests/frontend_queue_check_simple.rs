//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1796:frontend_queue_check_simple`
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
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - calls -> method Frontend::checkQueuedModules (Analysis/src/Frontend.cpp)
//!   - calls -> method Frontend::getCheckResult (Analysis/src/Frontend.cpp)
//!   - translates_to -> rust_item frontend_queue_check_simple

#[cfg(test)]
#[test]
fn frontend_queue_check_simple() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::boxed::Box;
    use alloc::string::String;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/A"),
        String::from(
            r#"
        --!strict
        return {hello=5, world=true}
    "#,
        ),
    );
    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/B"),
        String::from(
            r#"
        --!strict
        local Modules = game:GetService('Gui').Modules
        local A = require(Modules.A)
        return {b_value = A.hello}
    "#,
        ),
    );

    fixture
        .get_frontend()
        .queue_module_check_module_name(&String::from("game/Gui/Modules/B"));
    fixture.get_frontend().check_queued_modules(
        None,
        Box::new(|tasks| {
            for task in tasks {
                task();
            }
        }),
        Box::new(|_, _| true),
    );

    let result = fixture
        .get_frontend()
        .get_check_result(&String::from("game/Gui/Modules/B"), true, false)
        .expect("expected queued check result");
    assert!(result.errors.is_empty(), "{:?}", result.errors);
}
