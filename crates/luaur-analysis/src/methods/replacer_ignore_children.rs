use crate::functions::get_type_alt_j::get_type_id;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::replacer::Replacer;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl Replacer {
    pub fn ignore_children(&self, ty: TypeId) -> bool {
        unsafe {
            if !get_type_id::<ExternType>(ty).is_null() {
                return true;
            }

            if let Some(ftv) = get_type_id::<FunctionType>(ty).as_ref() {
                if ftv.has_no_free_or_generic_types {
                    return false;
                }

                for &generic in &ftv.generics {
                    if unsafe { (*self.replacements).find(&generic).is_some() } {
                        return true;
                    }
                }

                for &generic in &ftv.generic_packs {
                    if unsafe { (*self.replacement_packs).find(&generic).is_some() } {
                        return true;
                    }
                }
            }
        }

        false
    }
}
