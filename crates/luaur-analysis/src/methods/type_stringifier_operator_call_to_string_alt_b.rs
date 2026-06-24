use crate::records::type_stringifier::TypeStringifier;
use crate::type_aliases::bound_type::BoundType;
use crate::type_aliases::type_id::TypeId;

impl TypeStringifier {
    pub fn operator_call_10(&mut self, _ty: TypeId, btv: &BoundType) {
        self.stringify_type_id(btv.boundTo);
    }
}
