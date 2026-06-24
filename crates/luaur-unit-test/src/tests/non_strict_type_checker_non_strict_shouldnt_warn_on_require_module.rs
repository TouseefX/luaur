//! Node: `cxx:Test:Luau.UnitTest:tests/NonStrictTypeChecker.test.cpp:639:non_strict_type_checker_non_strict_shouldnt_warn_on_require_module`
//! Source: `tests/NonStrictTypeChecker.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/NonStrictTypeChecker.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/NonStrictTypeChecker.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/IostreamHelpers.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/NonStrictTypeChecker.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record SourceCode (Analysis/include/Luau/FileResolver.h)
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method NonStrictTypeCheckerFixture::checkNonStrictModule (tests/NonStrictTypeChecker.test.cpp)
//!   - translates_to -> rust_item non_strict_type_checker_non_strict_shouldnt_warn_on_require_module

#[cfg(test)]
#[test]
fn non_strict_type_checker_non_strict_shouldnt_warn_on_require_module() {
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use alloc::string::String;

    let mut fixture = NonStrictTypeCheckerFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("Modules/A"),
        String::from(
            r#"
--!strict
type t = {x : number}
local e : t = {x = 3}
return e
"#,
        ),
    );
    fixture.base.file_resolver.source_types.insert(
        String::from("Modules/A"),
        luaur_analysis::enums::type_file_resolver::Type::Module,
    );

    fixture.base.file_resolver.source.insert(
        String::from("Modules/B"),
        String::from(
            r#"
--!nonstrict
local E = require(script.Parent.A)
"#,
        ),
    );

    let result = fixture.check_non_strict_module(&String::from("Modules/B"));

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
