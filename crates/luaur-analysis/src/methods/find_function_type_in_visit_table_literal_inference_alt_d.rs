use crate::records::find_function_type_in::FindFunctionTypeIn;
use crate::records::function_type::FunctionType;
use crate::type_aliases::type_id::TypeId;

impl FindFunctionTypeIn {
    pub fn visit_type_id_function_type(&mut self, ty: TypeId, ftv: &FunctionType) -> bool {
        // This logic is a little clowny.
        //
        // For bidirectional inference we're trying to _guess_ what the user
        // is intending so that we can give decent results. For functions, we
        // will error if the user doesn't provide exactly the correct number of
        // arguments.
        //
        // The original C++ implementation attempts to prefer candidates with
        // an arg count closest to the lambda parameter count.
        let candidate = self.candidate;

        unsafe {
            if candidate.is_null()
                || (candidate_arg_count(candidate) as i32 - self.number_of_lambda_parameters).abs()
                    > (ftv_arg_count(ftv) as i32 - self.number_of_lambda_parameters).abs()
            {
                self.candidate = crate::functions::get_type_alt_j::get_type_id::<FunctionType>(ty);
                return false;
            }
        }

        false
    }
}

unsafe fn candidate_arg_count(candidate: *const FunctionType) -> usize {
    let c = &*candidate;
    type_pack_len(c.arg_types)
}

unsafe fn ftv_arg_count(ftv: &FunctionType) -> usize {
    type_pack_len(ftv.arg_types)
}

fn type_pack_len(arg_types: crate::type_aliases::type_pack_id::TypePackId) -> usize {
    // C++ uses `size(argTypes)` (TypePack.cpp:308) — count the bound head types
    // and follow the tail. The default-log overload passes `log = nullptr`.
    crate::functions::size_type_pack::size(arg_types, core::ptr::null_mut())
}
