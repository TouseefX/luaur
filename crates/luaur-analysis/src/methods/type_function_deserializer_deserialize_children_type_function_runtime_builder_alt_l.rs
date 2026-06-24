use crate::records::property_type::Property;
use crate::records::table_indexer::TableIndexer;
use crate::records::table_type::TableType;
use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::records::type_function_table_type::TypeFunctionTableType;

impl TypeFunctionDeserializer {
    pub fn deserialize_children_type_function_table_type_table_type(
        &mut self,
        t2: *mut TypeFunctionTableType,
        t1: *mut TableType,
    ) {
        unsafe {
            for (k, p) in &(*t2).props {
                let read_ty = p
                    .read_ty
                    .map(|ty| self.shallow_deserialize_type_function_type_id(ty));
                let write_ty = p
                    .write_ty
                    .map(|ty| self.shallow_deserialize_type_function_type_id(ty));

                let prop = match (read_ty, write_ty) {
                    (Some(r), Some(w)) => Property::rw_type_id_type_id(r, w),
                    (Some(r), None) => Property::readonly(r),
                    (None, Some(w)) => Property::writeonly(w),
                    (None, None) => Property::property(),
                };
                (*t1).props.insert(k.clone(), prop);
            }

            if let Some(indexer) = &(*t2).indexer {
                let key_ty = self.shallow_deserialize_type_function_type_id(indexer.key_type);
                let val_ty = self.shallow_deserialize_type_function_type_id(indexer.value_type);
                (*t1).indexer = Some(TableIndexer {
                    index_type: key_ty,
                    index_result_type: val_ty,
                    is_read_only: false,
                });
            }
        }
    }
}
