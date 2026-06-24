use crate::records::usage_finder::UsageFinder;
use crate::type_aliases::name_type::Name;
use luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl UsageFinder {
    pub fn visit_ast_stat_type_alias(&mut self, alias: *mut AstStatTypeAlias) -> bool {
        let alias_ref = unsafe { &*alias };
        let name_str = unsafe {
            core::ffi::CStr::from_ptr(alias_ref.name.value)
                .to_string_lossy()
                .into_owned()
        };
        self.declared_aliases.insert(Name::from(name_str));
        true
    }
}
