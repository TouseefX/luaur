use crate::records::extern_type::ExternType;
use crate::records::type_function_extern_type::TypeFunctionExternType;
use crate::records::type_function_property::TypeFunctionProperty;
use crate::records::type_function_serializer::TypeFunctionSerializer;
use crate::records::type_function_table_indexer::TypeFunctionTableIndexer;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;

impl TypeFunctionSerializer {
    pub fn serialize_children_extern_type_type_function_extern_type(
        &mut self,
        c1: *const ExternType,
        c2: *mut TypeFunctionExternType,
    ) {
        let c1_ref = unsafe { &*c1 };
        let c2_mut = unsafe { &mut *c2 };

        for (k, p) in &c1_ref.props {
            let read_ty = if let Some(read_ty) = &p.read_ty {
                Some(self.shallow_serialize_type_id(*read_ty))
            } else {
                None
            };

            let write_ty = if let Some(write_ty) = &p.write_ty {
                Some(self.shallow_serialize_type_id(*write_ty))
            } else {
                None
            };

            c2_mut
                .props
                .insert(k.clone(), TypeFunctionProperty { read_ty, write_ty });
        }

        if let Some(indexer) = &c1_ref.indexer {
            let key_type = self.shallow_serialize_type_id(indexer.index_type);
            let value_type = self.shallow_serialize_type_id(indexer.index_result_type);
            c2_mut.indexer = Some(TypeFunctionTableIndexer::new(key_type, value_type));
        }

        if let Some(metatable) = &c1_ref.metatable {
            c2_mut.metatable = Some(self.shallow_serialize_type_id(*metatable));
        }

        if let Some(parent) = &c1_ref.parent {
            let parent_ty = self.shallow_serialize_type_id(*parent);
            c2_mut.read_parent = Some(parent_ty);
            c2_mut.write_parent = Some(parent_ty);
        }
    }
}
