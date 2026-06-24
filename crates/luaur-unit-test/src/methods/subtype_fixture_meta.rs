use crate::records::subtype_fixture::SubtypeFixture;
use luaur_analysis::records::metatable_type::MetatableType;
use luaur_analysis::type_aliases::props_type_alt_c::Props;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SubtypeFixture {
    pub fn meta(&mut self, meta_props: Props, table_props: Props) -> TypeId {
        let meta_table = self.tbl(table_props);
        let meta_metatable = self.tbl(meta_props);
        self.meta_table_umbrella(meta_metatable, meta_table)
    }
}

impl SubtypeFixture {
    pub(crate) fn meta_table_umbrella(&mut self, meta: TypeId, table: TypeId) -> TypeId {
        self.arena.add_type(MetatableType::new(table, meta))
    }
}
