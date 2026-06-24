use crate::records::type_checker::TypeChecker;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl TypeChecker {
    pub fn try_strip_union_from_nil(&mut self, ty: TypeId) -> Option<TypeId> {
        unsafe {
            let utv_ptr = crate::functions::get_type_alt_j::get_type_id::<UnionType>(ty);
            if utv_ptr.is_null() {
                return None;
            }

            let utv: &UnionType = &*utv_ptr;

            let mut has_nil = false;
            let mut result: alloc::vec::Vec<TypeId> = alloc::vec::Vec::new();

            for &option in utv.options.iter() {
                if option == self.nil_type {
                    has_nil = true;
                    continue;
                }
                result.push(option);
            }

            if !has_nil {
                return Some(ty);
            }

            if result.is_empty() {
                return None;
            }

            if result.len() == 1 {
                return Some(result[0]);
            }

            Some(self.add_type(&UnionType { options: result }))
        }
    }
}
