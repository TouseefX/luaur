//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4720:autocomplete_autocomplete_via_bidirectional_self`
//! Source: `tests/Autocomplete.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Autocomplete.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
//!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Common/include/Luau/StringUtils.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Autocomplete.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - translates_to -> rust_item autocomplete_autocomplete_via_bidirectional_self

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_via_bidirectional_self() {
    use crate::records::ac_builtins_fixture::ACBuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(&String::from(
        r#"
        type IAccount = {
            __index: IAccount,
            new : (string, number) -> Account,
            report: (self: Account) -> (),
        }

        export type Account = setmetatable<{
            name: string,
            balance: number
        }, IAccount>;

        local Account = {} :: IAccount
        Account.__index = Account

        function Account.new(name, balance): Account
            local self = {}
            self.name = name
            self.balance = balance
            return setmetatable(self, Account)
        end

        function Account:report()
            print("My balance is: " .. self.@1)
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(ac.entry_map.contains_key("name"), true);
    assert_eq!(ac.entry_map.contains_key("balance"), true);
}
