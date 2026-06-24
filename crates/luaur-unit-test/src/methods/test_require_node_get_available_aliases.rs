impl crate::records::test_require_node::TestRequireNode {
    pub fn get_available_aliases(
        &self,
    ) -> alloc::vec::Vec<luaur_analysis::records::require_alias::RequireAlias> {
        alloc::vec![
            luaur_analysis::records::require_alias::RequireAlias::require_alias_string(
                alloc::string::String::from("defaultalias")
            )
        ]
    }
}
