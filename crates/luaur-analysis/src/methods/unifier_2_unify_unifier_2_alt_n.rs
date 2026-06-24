use crate::enums::unify_result::UnifyResult;
use crate::records::any_type::AnyType;
use crate::records::table_type::TableType;
use crate::records::unifier_2::Unifier2;

impl Unifier2 {
    pub fn unify_table_type_any_type(
        &mut self,
        sub_table: &TableType,
        _super_any: &AnyType,
    ) -> UnifyResult {
        for (_prop_name, prop) in &sub_table.props {
            if let Some(read_ty) = prop.read_ty {
                let _ = self.unify_type_id_type_id(read_ty, unsafe {
                    (*self.builtin_types.as_ptr()).anyType
                });
            }

            if let Some(write_ty) = prop.write_ty {
                let _ = self.unify_type_id_type_id(
                    unsafe { (*self.builtin_types.as_ptr()).anyType },
                    write_ty,
                );
            }
        }

        if let Some(indexer) = &sub_table.indexer {
            let _ = self.unify_type_id_type_id(indexer.index_type, unsafe {
                (*self.builtin_types.as_ptr()).anyType
            });
            let _ = self.unify_type_id_type_id(indexer.index_result_type, unsafe {
                (*self.builtin_types.as_ptr()).anyType
            });
        }

        UnifyResult::Ok
    }
}
