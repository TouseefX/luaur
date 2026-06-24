use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::records::type_function_serializer::TypeFunctionSerializer;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeFunctionSerializer {
    pub fn find_type_pack_id(&self, tp: TypePackId) -> Option<TypeFunctionTypePackId> {
        let tp = unsafe { follow_type_pack_id(tp) };
        self.packs.get(&tp).copied()
    }
}
