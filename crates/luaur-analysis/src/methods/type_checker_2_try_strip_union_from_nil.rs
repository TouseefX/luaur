use crate::records::r#type::Type;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;

impl TypeChecker2 {
    pub fn try_strip_union_from_nil(&self, ty: TypeId) -> Option<TypeId> {
        unsafe {
            let utv_ptr = crate::functions::get_type_alt_j::get_type_id::<UnionType>(ty);
            if utv_ptr.is_null() {
                return None;
            }

            let utv: &UnionType = &*utv_ptr;

            // if (!std::any_of(begin(utv), end(utv), isNil)) return ty;
            if !utv
                .options
                .iter()
                .any(|&option| crate::functions::is_nil::is_nil(option))
            {
                return Some(ty);
            }

            let mut result: alloc::vec::Vec<TypeId> = alloc::vec::Vec::new();
            for &option in utv.options.iter() {
                if !crate::functions::is_nil::is_nil(option) {
                    result.push(option);
                }
            }

            if result.is_empty() {
                return None;
            }

            if result.len() == 1 {
                return Some(result[0]);
            }

            Some(
                (*self.module)
                    .internal_types
                    .add_type(Type::new(TypeVariant::Union(UnionType { options: result }))),
            )
        }
    }
}
