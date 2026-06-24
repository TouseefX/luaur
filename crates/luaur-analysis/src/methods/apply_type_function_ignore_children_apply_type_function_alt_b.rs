use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::apply_type_function::ApplyTypeFunction;
use crate::records::generic_type_pack::GenericTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

impl ApplyTypeFunction {
    pub fn ignore_children_type_pack_id(&mut self, tp: TypePackId) -> bool {
        let gt = unsafe { get_type_pack_id::<GenericTypePack>(tp) };
        !gt.is_null()
    }
}
