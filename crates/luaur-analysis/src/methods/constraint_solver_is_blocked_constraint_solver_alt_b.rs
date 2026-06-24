use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

impl ConstraintSolver {
    pub fn is_blocked_type_pack_id(&self, tp: TypePackId) -> bool {
        let tp = unsafe { follow_type_pack_id(tp) };

        if !unsafe { get_type_pack_id::<TypeFunctionInstanceTypePack>(tp) }.is_null() {
            return !self
                .uninhabited_type_functions
                .contains(&(tp as *const core::ffi::c_void));
        }

        !unsafe { get_type_pack_id::<BlockedTypePack>(tp) }.is_null()
    }
}
