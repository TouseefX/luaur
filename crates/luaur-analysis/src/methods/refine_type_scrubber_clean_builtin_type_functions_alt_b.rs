//! C++ `RefineTypeScrubber::clean(TypeId ty)`
//! (BuiltinTypeFunctions.cpp:1131-1169). Filters the needle (and `never`/
//! `unknown` short-circuits) out of a union/intersection, collapsing the result.
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::intersection_type::IntersectionType;
use crate::records::never_type::NeverType;
use crate::records::refine_type_scrubber::RefineTypeScrubber;
use crate::records::type_ids::TypeIds;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;

impl RefineTypeScrubber {
    pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
        // NOTE: this feels pretty similar to other places where we try to
        // filter over a set type, may be worth combining those in the future.
        let ctx_ref = unsafe { self.ctx.as_ref() };
        let builtins = unsafe { ctx_ref.builtins.as_ref() };

        if let Some(ut) = unsafe { get_type_id::<UnionType>(ty).as_ref() } {
            let mut new_options = TypeIds::type_ids();
            for &option in &ut.options {
                let followed = unsafe { follow_type_id(option) };
                if followed != self.needle
                    && unsafe { get_type_id::<NeverType>(followed) }.is_null()
                {
                    new_options.insert_type_id(option);
                }
            }
            if new_options.empty() {
                builtins.neverType
            } else if new_options.size() == 1 {
                new_options.front()
            } else {
                unsafe {
                    (*ctx_ref.arena.as_ptr()).add_type(UnionType {
                        options: new_options.take(),
                    })
                }
            }
        } else if let Some(it) = unsafe { get_type_id::<IntersectionType>(ty).as_ref() } {
            let mut new_parts = TypeIds::type_ids();
            for &part in &it.parts {
                let followed = unsafe { follow_type_id(part) };
                if followed != self.needle
                    && unsafe { get_type_id::<UnknownType>(followed) }.is_null()
                {
                    new_parts.insert_type_id(part);
                }
            }
            if new_parts.empty() {
                builtins.unknownType
            } else if new_parts.size() == 1 {
                new_parts.front()
            } else {
                unsafe {
                    (*ctx_ref.arena.as_ptr()).add_type(IntersectionType {
                        parts: new_parts.take(),
                    })
                }
            }
        } else if ty == self.needle {
            builtins.unknownType
        } else {
            ty
        }
    }
}
