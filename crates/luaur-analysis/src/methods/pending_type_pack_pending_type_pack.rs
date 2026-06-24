use crate::records::pending_type_pack::PendingTypePack;
use crate::records::type_pack_var::TypePackVar;

impl PendingTypePack {
    pub fn pending_type_pack(&mut self, _state: TypePackVar) {
        self.pending = _state;
    }
}
