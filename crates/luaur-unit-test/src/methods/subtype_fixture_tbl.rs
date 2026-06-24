//! Node: `cxx:Method:Luau.UnitTest:tests/Subtyping.test.cpp:117:subtype_fixture_tbl`
//! Source: `tests/Subtyping.test.cpp:117-120`
use crate::records::subtype_fixture::SubtypeFixture;
use luaur_analysis::enums::table_state::TableState;
use luaur_analysis::records::table_type::TableType;
use luaur_analysis::records::type_level::TypeLevel;
use luaur_analysis::type_aliases::props_type_alt_c::Props;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SubtypeFixture {
    /// C++ `TypeId tbl(TableType::Props&& props)`.
    pub fn tbl(&mut self, props: Props) -> TypeId {
        self.arena.add_type(
            TableType::table_type_props_optional_table_indexer_type_level_table_state(
                &props,
                None,
                TypeLevel::default(),
                TableState::Sealed,
            ),
        )
    }
}
