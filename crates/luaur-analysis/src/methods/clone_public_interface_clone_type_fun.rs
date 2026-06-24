use crate::records::clone_public_interface::ClonePublicInterface;
use crate::records::generic_type_definition::GenericTypeDefinition;
use crate::records::generic_type_pack_definition::GenericTypePackDefinition;
use crate::records::type_fun::TypeFun;
use alloc::vec::Vec;

impl ClonePublicInterface {
    /// `TypeFun ClonePublicInterface::cloneTypeFun(const TypeFun& tf)`.
    /// Reference: `Module.cpp:260-290`.
    pub fn clone_type_fun(&mut self, tf: &TypeFun) -> TypeFun {
        let mut type_params: Vec<GenericTypeDefinition> = Vec::new();
        let mut type_pack_params: Vec<GenericTypePackDefinition> = Vec::new();

        for type_param in tf.type_params.iter().copied() {
            let ty = self.clone_type(type_param.ty);
            let mut default_value = None;

            if let Some(dv) = type_param.defaultValue {
                default_value = Some(self.clone_type(dv));
            }

            type_params.push(GenericTypeDefinition {
                ty,
                defaultValue: default_value,
            });
        }

        for type_pack_param in tf.type_pack_params.iter().copied() {
            let tp = self.clone_type_pack(type_pack_param.tp);
            let mut default_value = None;

            if let Some(dv) = type_pack_param.defaultValue {
                default_value = Some(self.clone_type_pack(dv));
            }

            type_pack_params.push(GenericTypePackDefinition {
                tp,
                defaultValue: default_value,
            });
        }

        let r#type = self.clone_type(tf.r#type);

        TypeFun {
            type_params,
            type_pack_params,
            r#type,
            definition_location: tf.definition_location,
        }
    }
}
