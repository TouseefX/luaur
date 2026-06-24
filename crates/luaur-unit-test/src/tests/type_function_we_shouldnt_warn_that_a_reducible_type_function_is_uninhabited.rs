//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:871:type_function_we_shouldnt_warn_that_a_reducible_type_function_is_uninhabited`
//! Source: `tests/TypeFunction.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> enum Mode (Ast/include/Luau/ParseOptions.h)
//!   - translates_to -> rust_item type_function_we_shouldnt_warn_that_a_reducible_type_function_is_uninhabited

#[cfg(test)]
#[test]
fn type_function_we_shouldnt_warn_that_a_reducible_type_function_is_uninhabited() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"

local Debounce = false
local Active = false

local function Use(Mode)

	if Mode ~= nil then

		if Mode == false and Active == false then
			return
		else
			Active = not Mode
		end

		Debounce = false
	end
	Active = not Active

end
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
}
