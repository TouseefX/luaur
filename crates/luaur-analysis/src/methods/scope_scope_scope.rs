//! C++ `Scope::Scope(TypePackId returnType)` (`Analysis/src/Scope.cpp:10`):
//! a root scope with no parent, the given return type, and a default
//! `TypeLevel{}`. Every other member uses its in-class initializer (the
//! `DenseHash*` empty-key sentinels from `Analysis/include/Luau/Scope.h`).
use crate::records::def::Def;
use crate::records::scope::Scope;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;
use std::collections::HashMap;

impl Scope {
    pub fn scope_type_pack_id(return_type: TypePackId) -> Self {
        Scope {
            parent: None,
            children: Vec::new(),
            bindings: HashMap::new(),
            return_type,
            vararg_pack: None,
            level: TypeLevel::default(),
            location: Location::default(),
            exported_type_bindings: HashMap::new(),
            private_type_bindings: HashMap::new(),
            type_alias_locations: HashMap::new(),
            type_alias_name_locations: HashMap::new(),
            imported_modules: HashMap::new(),
            imported_type_bindings: HashMap::new(),
            builtin_type_names: DenseHashSet::default(),
            private_type_pack_bindings: HashMap::new(),
            refinements: HashMap::new(),
            lvalue_types: DenseHashMap::new(core::ptr::null::<Def>()),
            rvalue_refinements: DenseHashMap::new(core::ptr::null::<Def>()),
            globals_to_warn: DenseHashSet::default(),
            type_alias_type_parameters: HashMap::new(),
            type_alias_type_pack_parameters: HashMap::new(),
            interior_free_types: None,
            interior_free_type_packs: None,
            invalid_type_aliases: DenseHashMap::new(String::new()),
        }
    }
}
