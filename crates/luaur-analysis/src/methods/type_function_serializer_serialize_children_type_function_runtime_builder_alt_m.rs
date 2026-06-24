use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::metatable_type::MetatableType;
use crate::records::table_type::TableType;
use crate::records::type_function_serializer::TypeFunctionSerializer;
use crate::records::type_function_table_type::TypeFunctionTableType;

impl TypeFunctionSerializer {
    pub fn serialize_children_metatable_type_type_function_table_type(
        &mut self,
        m1: *const MetatableType,
        m2: *mut TypeFunctionTableType,
    ) {
        unsafe {
            let m1 = &*m1;
            let m2 = &mut *m2;
            let table = follow_type_id(m1.table);

            if let Some(table) = get_type_id::<TableType>(table).as_ref() {
                self.serialize_children_table_type_type_function_table_type(table, m2);
            }

            m2.metatable = Some(self.shallow_serialize_type_id(m1.metatable));
        }
    }
}
