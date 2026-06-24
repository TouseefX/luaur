use crate::enums::unify_result::UnifyResult;
use crate::records::any_type::AnyType;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::table_type::TableType;
use crate::records::unifier_2::Unifier2;
use crate::type_aliases::type_id::TypeId;
use core::ptr::NonNull;

impl Unifier2 {
    pub fn unify_any_type_table_type(
        &mut self,
        _sub_any: &AnyType,
        super_table: &TableType,
    ) -> UnifyResult {
        let builtin_types_ptr: NonNull<BuiltinTypes> = self.builtin_types;
        let builtin_types_ref: &BuiltinTypes = unsafe { builtin_types_ptr.as_ref() };
        let any_type_id: TypeId = builtin_types_ref.anyType;

        for (_prop_name, prop) in &super_table.props {
            if let Some(read_ty) = prop.read_ty {
                let _ = self.unify_type_id_type_id(any_type_id, read_ty);
            }

            if let Some(write_ty) = prop.write_ty {
                let _ = self.unify_type_id_type_id(write_ty, any_type_id);
            }
        }

        if let Some(indexer) = &super_table.indexer {
            let _ = self.unify_type_id_type_id(any_type_id, indexer.index_type);
            let _ = self.unify_type_id_type_id(any_type_id, indexer.index_result_type);
        }

        UnifyResult::Ok
    }
}
