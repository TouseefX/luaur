//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1471:frontend_get_required_scripts`
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
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method Frontend::getRequiredScripts (Analysis/src/Frontend.cpp)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item frontend_get_required_scripts

#[cfg(test)]
#[test]
fn frontend_get_required_scripts() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use luaur_analysis::records::type_check_limits::TypeCheckLimits;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("game/workspace/MyScript"),
        String::from(
            r#"
        local MyModuleScript = require(game.workspace.MyModuleScript)
        local MyModuleScript2 = require(game.workspace.MyModuleScript2)
        MyModuleScript.myPrint()
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("game/workspace/MyModuleScript"),
        String::from(
            r#"
        local module = {}
        function module.myPrint()
            print("Hello World")
        end
        return module
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("game/workspace/MyModuleScript2"),
        String::from(
            r#"
        local module = {}
        return module
    "#,
        ),
    );

    fixture
        .get_frontend()
        .mark_dirty(&String::from("game/workspace/MyScript"), None);
    let mut required_scripts = fixture.get_frontend().get_required_scripts(
        &String::from("game/workspace/MyScript"),
        &TypeCheckLimits::default(),
    );
    assert_eq!(2, required_scripts.len(), "{:?}", required_scripts);
    assert_eq!("game/workspace/MyModuleScript", required_scripts[0]);
    assert_eq!("game/workspace/MyModuleScript2", required_scripts[1]);

    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(
            &String::from("game/workspace/MyScript"),
            None,
        );
    required_scripts = fixture.get_frontend().get_required_scripts(
        &String::from("game/workspace/MyScript"),
        &TypeCheckLimits::default(),
    );
    assert_eq!(2, required_scripts.len(), "{:?}", required_scripts);
    assert_eq!("game/workspace/MyModuleScript", required_scripts[0]);
    assert_eq!("game/workspace/MyModuleScript2", required_scripts[1]);
}
