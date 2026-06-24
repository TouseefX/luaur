use crate::enums::table_state::TableState;
use crate::records::anyification::Anyification;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Anyification {
    pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
        LUAU_ASSERT!(self.is_dirty_type_id(ty));

        let log = self.base.base.log;
        let ttv = unsafe { (*log).txn_log_get_mutable::<TableType, TypeId>(ty) };
        if !ttv.is_null() {
            let ttv = unsafe { &*ttv };
            let mut clone =
                TableType::table_type_props_optional_table_indexer_type_level_table_state(
                    &ttv.props,
                    ttv.indexer.clone(),
                    ttv.level,
                    TableState::Sealed,
                );
            clone.definition_module_name = ttv.definition_module_name.clone();
            clone.definition_location = ttv.definition_location.clone();
            clone.name = ttv.name.clone();
            clone.synthetic_name = ttv.synthetic_name.clone();
            clone.tags = ttv.tags.clone();

            return self.base.add_type(clone);
        }

        self.any_type
    }
}
