use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::type_function_serializer::TypeFunctionSerializer;
use crate::records::type_function_type_pack_var::TypeFunctionTypePackVar;
use crate::records::type_pack::TypePack;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_function_type_pack_variant::TypeFunctionTypePackVariant;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeFunctionSerializer {
    pub fn serialize_children_type_pack_id_type_function_type_pack_id(
        &mut self,
        tp: TypePackId,
        tftp: TypeFunctionTypePackId,
    ) {
        if tftp.is_null() {
            return;
        }

        let tp = unsafe { follow_type_pack_id(tp) };

        unsafe {
            let target = &mut (*(tftp as *mut TypeFunctionTypePackVar)).type_variant;

            if let Some(source) = get_type_pack_id::<TypePack>(tp).as_ref() {
                if let TypeFunctionTypePackVariant::V0(target) = target {
                    self.serialize_children_type_pack_type_function_type_pack(source, target);
                }
            } else if let Some(source) = get_type_pack_id::<VariadicTypePack>(tp).as_ref() {
                if let TypeFunctionTypePackVariant::V1(target) = target {
                    self.serialize_children_variadic_type_pack_type_function_variadic_type_pack(
                        source, target,
                    );
                }
            } else if let Some(source) = get_type_pack_id::<GenericTypePack>(tp).as_ref() {
                if let TypeFunctionTypePackVariant::V2(target) = target {
                    self.serialize_children_generic_type_pack_type_function_generic_type_pack(
                        source, target,
                    );
                }
            }
        }
    }
}
