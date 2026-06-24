use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_buffer::is_buffer;
use crate::functions::is_integer::is_integer;
use crate::functions::is_number::is_number;
use crate::functions::is_prim::is_prim;
use crate::functions::is_thread::is_thread;
use crate::records::extern_type::ExternType;
use crate::records::never_type::NeverType;
use crate::records::normalized_type::NormalizedType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::unknown_type::UnknownType;
use luaur_common::FFlag;

impl NormalizedType {
    pub fn is_unknown(&self) -> bool {
        // Check if tops is UnknownType
        let tops_ptr = unsafe { get_type_id::<UnknownType>(self.tops) };
        if !tops_ptr.is_null() {
            return true;
        }

        // Check if we have all primitives
        let has_all_primitives = if FFlag::LuauIntegerType2.get() {
            is_prim(self.booleans, PrimitiveType::Boolean)
                && is_prim(self.nils, PrimitiveType::NilType)
                && is_number(self.numbers)
                && self.strings.is_string()
                && is_thread(self.threads)
                && is_buffer(self.buffers)
                && is_integer(self.integers)
        } else {
            is_prim(self.booleans, PrimitiveType::Boolean)
                && is_prim(self.nils, PrimitiveType::NilType)
                && is_number(self.numbers)
                && self.strings.is_string()
                && is_thread(self.threads)
                && is_buffer(self.buffers)
        };

        // Check extern types: we need at least one ExternType that matches builtinTypes->externType with empty disjunction
        let mut is_top_extern_type = false;
        for (t, disj) in &self.extern_types.extern_types {
            let extern_type_ptr = unsafe { get_type_id::<ExternType>(*t) };
            if !extern_type_ptr.is_null() {
                let builtin_extern_type = unsafe { (*self.builtin_types).externType };
                if *t == builtin_extern_type && disj.empty() {
                    is_top_extern_type = true;
                    break;
                }
            }
        }

        // Check tables: we need at least one PrimitiveType::Table
        let mut is_top_table = false;
        for &t in &self.tables.order {
            if is_prim(t, PrimitiveType::Table) {
                is_top_table = true;
                break;
            }
        }

        // any = unknown or error ==> we need to make sure we have all the unknown components, but not errors
        let errors_ptr = unsafe { get_type_id::<NeverType>(self.errors) };
        !errors_ptr.is_null()
            && has_all_primitives
            && is_top_extern_type
            && is_top_table
            && self.functions.is_top
    }
}
