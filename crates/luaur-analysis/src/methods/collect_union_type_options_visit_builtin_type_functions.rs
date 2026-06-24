use crate::functions::is_pending::is_pending;
use crate::records::collect_union_type_options::CollectUnionTypeOptions;
use crate::type_aliases::type_id::TypeId;

impl CollectUnionTypeOptions {
    pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
        self.options.insert(ty);
        if is_pending(ty, unsafe { self.ctx.as_ref().solver }) {
            self.blocking_types.insert(ty);
        }
        false
    }
}
