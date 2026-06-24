//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Scope.h:33:scope`
//! Source: `Analysis/include/Luau/Scope.h` (Scope.h:33-118, hand-ported; fields only,
//! methods are separate schedule items)

use crate::records::binding::Binding;
use crate::records::def::Def;
use crate::records::symbol::Symbol;
use crate::records::type_fun::TypeFun;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::module_name_type::ModuleName;
use crate::type_aliases::name_type::Name;
use crate::type_aliases::refinement_map::RefinementMap;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scope {
    pub parent: Option<ScopePtr>, // null for the root

    pub children: Vec<*mut Scope>, // NotNull<Scope>
    pub bindings: HashMap<Symbol, Binding>,
    pub return_type: TypePackId,
    pub vararg_pack: Option<TypePackId>,

    pub level: TypeLevel,

    /// the spanning location associated with this scope
    pub location: Location,

    pub exported_type_bindings: HashMap<Name, TypeFun>,
    pub private_type_bindings: HashMap<Name, TypeFun>,
    pub type_alias_locations: HashMap<Name, Location>,
    pub type_alias_name_locations: HashMap<Name, Location>,
    /// Mapping from the name in the require statement to the internal moduleName.
    pub imported_modules: HashMap<Name, ModuleName>,
    pub imported_type_bindings: HashMap<Name, HashMap<Name, TypeFun>>,
    pub builtin_type_names: DenseHashSet<Name>,

    pub private_type_pack_bindings: HashMap<Name, TypePackId>,

    pub refinements: RefinementMap,
    pub lvalue_types: DenseHashMap<*const Def, TypeId>,
    pub rvalue_refinements: DenseHashMap<*const Def, TypeId>,

    pub globals_to_warn: DenseHashSet<String>,

    pub type_alias_type_parameters: HashMap<Name, TypeId>,
    pub type_alias_type_pack_parameters: HashMap<Name, TypePackId>,

    pub interior_free_types: Option<Vec<TypeId>>,
    pub interior_free_type_packs: Option<Vec<TypePackId>>,

    pub invalid_type_aliases: DenseHashMap<String, Location>,
}
