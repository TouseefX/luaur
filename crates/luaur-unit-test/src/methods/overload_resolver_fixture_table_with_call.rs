use crate::records::overload_resolver_fixture::OverloadResolverFixture;
use luaur_analysis::enums::table_state::TableState;
use luaur_analysis::records::metatable_type::MetatableType;
use luaur_analysis::records::property_type::Property;
use luaur_analysis::records::table_type::TableType;
use luaur_analysis::records::type_level::TypeLevel;
use luaur_analysis::type_aliases::props_type::Props;
use luaur_analysis::type_aliases::type_id::TypeId;

impl OverloadResolverFixture {
    pub fn table_with_call(&self, call_mm: TypeId) -> TypeId {
        unsafe {
            let table = (*self.arena).add_type(TableType::table_type_table_state_type_level_scope(
                TableState::Sealed,
                TypeLevel::default(),
                core::ptr::null_mut(),
            ));

            let mut props = Props::new();
            props.insert("__call".to_string(), Property::readonly(call_mm));
            let metatable = (*self.arena).add_type(
                TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
                    &props,
                    None,
                    TypeLevel::default(),
                    core::ptr::null_mut(),
                    TableState::Sealed,
                ),
            );

            (*self.arena).add_type(MetatableType::new(table, metatable))
        }
    }
}
