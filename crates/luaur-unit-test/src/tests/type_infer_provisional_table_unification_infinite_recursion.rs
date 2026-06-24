//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1065:type_infer_provisional_table_unification_infinite_recursion`
//! Source: `tests/TypeInfer.provisional.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_provisional_table_unification_infinite_recursion

#[cfg(test)]
#[test]
fn type_infer_provisional_table_unification_infinite_recursion() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
local tbl = {}

function tbl:f1(state)
    self.someNonExistentvalue2 = state
end

function tbl:f2()
    self.someNonExistentvalue:Dc()
end

function tbl:f3()
    self:f2()
    self:f1(false)
end
return tbl
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
local tbl = require(game.A)
tbl:f3()
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}
