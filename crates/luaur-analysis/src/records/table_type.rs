//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Type.h:488:table_type`
//! Source: `Analysis/include/Luau/Type.h` (Type.h:488-524, hand-ported)

use crate::enums::table_state::TableState;
use crate::records::scope::Scope;
use crate::records::table_indexer::TableIndexer;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::module_name_type_fwd::ModuleName;
use crate::type_aliases::props_type::Props;
use crate::type_aliases::tags::Tags;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;

#[derive(Debug, Clone)]
pub struct TableType {
    pub props: Props,
    pub indexer: Option<TableIndexer>,

    pub state: TableState,
    pub level: TypeLevel,
    pub scope: *mut Scope,
    pub name: Option<String>,

    /// Sometimes we throw a type on a name to make for nicer error messages,
    /// but without creating any entry in the type namespace.
    pub synthetic_name: Option<String>,

    pub instantiated_type_params: Vec<TypeId>,
    pub instantiated_type_pack_params: Vec<TypePackId>,
    pub definition_module_name: ModuleName,
    pub definition_location: Location,

    pub bound_to: Option<TypeId>,
    pub tags: Tags,

    /// Number of as-yet-unadded properties on unsealed tables; some
    /// constraints use this to decide whether they can dispatch.
    pub remaining_props: usize,
}
