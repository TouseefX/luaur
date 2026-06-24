use crate::enums::skip_test_result::SkipTestResult;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::records::type_function_reducer::TypeFunctionReducer;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeFunctionReducer {
    pub fn test_for_skippability_type_pack_id(&self, ty: TypePackId) -> SkipTestResult {
        let ty = unsafe { follow_type_pack_id(ty) };

        if !unsafe { get_type_pack_id::<TypeFunctionInstanceTypePack>(ty) }.is_null() {
            if !self.irreducible.contains(&(ty as *const core::ffi::c_void)) {
                return SkipTestResult::Defer;
            } else {
                return SkipTestResult::Irreducible;
            }
        } else if !unsafe { get_type_pack_id::<GenericTypePack>(ty) }.is_null() {
            return SkipTestResult::Generic;
        }

        SkipTestResult::Okay
    }
}
