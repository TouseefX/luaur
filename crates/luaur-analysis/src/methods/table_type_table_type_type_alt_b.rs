use crate::enums::table_state::TableState;
use crate::records::scope::Scope;
use crate::records::table_type::TableType;
use crate::records::type_level::TypeLevel;

impl TableType {
    pub fn table_type_table_state_type_level_scope(
        state: TableState,
        level: TypeLevel,
        scope: *mut Scope,
    ) -> Self {
        TableType {
            state,
            level,
            scope,
            props: Default::default(),
            indexer: None,
            name: None,
            synthetic_name: None,
            instantiated_type_params: Default::default(),
            instantiated_type_pack_params: Default::default(),
            definition_module_name: Default::default(),
            definition_location: Default::default(),
            bound_to: None,
            tags: Default::default(),
            remaining_props: 0,
        }
    }
}
