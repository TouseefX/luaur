use crate::records::type_fun::TypeFun;

impl TypeFun {
    pub fn type_fun() -> Self {
        Self {
            type_params: Vec::new(),
            type_pack_params: Vec::new(),
            r#type: core::ptr::null(),
            definition_location: None,
        }
    }
}
