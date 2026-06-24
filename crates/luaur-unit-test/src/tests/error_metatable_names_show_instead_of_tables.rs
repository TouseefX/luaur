//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Error.test.cpp:19:error_metatable_names_show_instead_of_tables`
//! Source: `tests/Error.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Error.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/Error.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item error_metatable_names_show_instead_of_tables

#[cfg(test)]
#[test]
fn error_metatable_names_show_instead_of_tables() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend().options.retain_full_type_graphs = false;

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
local Account = {}
Account.__index = Account
function Account.deposit(self: Account, x: number)
	self.balance += x
end
type Account = typeof(setmetatable({} :: { balance: number }, Account))
local x: Account = 5
"#,
        ),
        None,
    );

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
        "Expected this to be 'Account', but got 'number'",
        to_string_type_error(&result.errors[0])
    );
}
