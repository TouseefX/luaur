//! Source: `Analysis/src/OverloadResolver.cpp:226-287` (hand-ported)
//!
//! Figuring out which argument a particular path points at can be kind of tricky
//! due to generic pack substitutions.
use crate::enums::pack_field::PackField;
use crate::functions::begin_type_pack::begin;
use crate::functions::end_type_pack::end;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::function_type::FunctionType;
use crate::records::path::Path;
use crate::type_aliases::component::Component;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_pack::TypeOrPack;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::variant::Variant2;

pub fn get_argument_index(path: &Path, fn_ty: TypeId) -> Option<usize> {
    let mut iter = path.components.iter();

    let first = match iter.next() {
        Some(c) => c,
        None => return None,
    };

    if let Component::PackField(args) = first {
        if *args != PackField::Arguments {
            return None;
        }
    } else {
        return None;
    }

    let ft = unsafe { get_type_id::<FunctionType>(fn_ty) };
    LUAU_ASSERT!(!fn_ty.is_null());
    if ft.is_null() {
        return None;
    }
    let ft = unsafe { &*ft };

    let mut result: usize = 0;
    let mut ty: TypeOrPack = Variant2::V1(ft.arg_types);

    for component in iter {
        match component {
            Component::Index(index) => return Some(result + index.index),
            Component::GenericPackMapping(subst) => {
                ty = Variant2::V1(subst.mappedType);
            }
            Component::PackSlice(slice) => {
                result += slice.start_index;
            }
            Component::PackField(pack_field) if *pack_field == PackField::Tail => {
                // If the path component points at the tail of the pack, we need to
                // advance the count by the length of the current pack.
                let tp: Option<&TypePackId> = ty.get_if_1();
                let tp = match tp {
                    Some(tp) => *tp,
                    None => {
                        LUAU_ASSERT!(false);
                        return None;
                    }
                };

                // Subtyping flattens out chains of concrete packs when it generates
                // these TypePaths, so we need to do the same here.
                let mut pack_iter = begin(tp);
                let pack_end_iter = end(tp);
                while pack_iter.operator_ne(&pack_end_iter) {
                    result += 1;
                    pack_iter.operator_inc();
                }

                match pack_iter.tail() {
                    Some(tail) => ty = Variant2::V1(tail),
                    None => return None,
                }

                continue;
            }
            _ => return None,
        }
    }

    None
}
