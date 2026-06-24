use crate::records::subtype_fixture::SubtypeFixture;
use luaur_analysis::enums::table_state::TableState;
use luaur_analysis::records::table_indexer::TableIndexer;
use luaur_analysis::records::table_type::TableType;
use luaur_analysis::records::type_level::TypeLevel;
use luaur_analysis::type_aliases::props_type_alt_c::Props;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SubtypeFixture {
    pub fn tbl_with_indexer(&mut self, props: Props, key_ty: TypeId, value_ty: TypeId) -> TypeId {
        self.arena.add_type(
            TableType::table_type_props_optional_table_indexer_type_level_table_state(
                &props,
                Some(TableIndexer {
                    index_type: key_ty,
                    index_result_type: value_ty,
                    is_read_only: false,
                }),
                TypeLevel::default(),
                TableState::Sealed,
            ),
        )
    }
}
