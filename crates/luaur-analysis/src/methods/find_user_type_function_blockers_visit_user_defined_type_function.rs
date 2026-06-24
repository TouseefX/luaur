use crate::functions::is_pending::is_pending;
use crate::records::find_user_type_function_blockers::FindUserTypeFunctionBlockers;
use crate::type_aliases::type_id::TypeId;

impl FindUserTypeFunctionBlockers {
    pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
        let solver = unsafe { self.ctx.as_ref().solver };
        if is_pending(ty, solver) {
            if !self.blocking_type_map.contains(&ty) {
                self.blocking_type_map.insert(ty);
                self.blocking_types.push(ty);
            }
        }
        true
    }
}
