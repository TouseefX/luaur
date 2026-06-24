//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4088:fragment_autocomplete_self_with_fancy_metatable_setting_new_solver`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_self_with_fancy_metatable_setting_new_solver() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
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
            print("My balance is: " .. )
        end
"#,
    );

    let dest = String::from(
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
            print("My balance is: " .. self.@1 )
        end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
        &source,
        &dest,
        '1',
        Box::new(|result: &mut FragmentAutocompleteStatusResult| {
            let ac = &result.result.as_ref().unwrap().ac_results;
            assert!(!ac.entry_map.is_empty());
            assert!(ac.entry_map.contains_key("new"));
            assert!(ac.entry_map.contains_key("report"));
        }),
        None,
    );
}
