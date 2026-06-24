use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::lint_table_operations::LintTableOperations;
use crate::type_aliases::type_id::TypeId;

impl LintTableOperations {
    pub fn get_return_count(&mut self, ty: TypeId) -> usize {
        let ty = unsafe { follow_type_id(ty) };

        if !unsafe { get_type_id::<FunctionType>(ty).is_null() } {
            let ftv = unsafe { &*get_type_id::<FunctionType>(ty) };
            return crate::functions::size_type_pack::size(ftv.ret_types, core::ptr::null_mut());
        }

        if !unsafe { get_type_id::<IntersectionType>(ty).is_null() } {
            let itv = unsafe { &*get_type_id::<IntersectionType>(ty) };
            let mut result = 0;

            for &part in itv.parts.iter() {
                let followed_part = unsafe { follow_type_id(part) };
                if !unsafe { get_type_id::<FunctionType>(followed_part).is_null() } {
                    let ftv = unsafe { &*get_type_id::<FunctionType>(followed_part) };
                    let count = crate::functions::size_type_pack::size(
                        ftv.ret_types,
                        core::ptr::null_mut(),
                    );
                    result = std::cmp::max(result, count);
                }
            }

            return result;
        }

        0
    }
}
