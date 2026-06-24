use crate::records::extern_type::ExternType;
use crate::records::type_stringifier::TypeStringifier;
use crate::type_aliases::type_id::TypeId;

impl TypeStringifier {
    pub fn operator_call(&mut self, ty: TypeId, etv: &ExternType) {
        let state = unsafe { &mut *self.state };
        state.emit_and_record_span(&etv.name, ty);
    }
}
