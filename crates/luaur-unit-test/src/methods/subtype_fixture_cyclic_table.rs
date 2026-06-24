use crate::records::subtype_fixture::SubtypeFixture;
use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
use luaur_analysis::records::table_type::TableType;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SubtypeFixture {
    pub fn cyclic_table<F>(&mut self, cb: F) -> TypeId
    where
        F: FnOnce(&mut SubtypeFixture, TypeId, &mut TableType),
    {
        let ty = self.arena.add_type(TableType::table_type());
        let table = unsafe { get_mutable_type_id::<TableType>(ty).as_mut() }
            .expect("expected cyclic table");
        cb(self, ty, table);
        ty
    }
}
