//! Node: `cxx:Method:Luau.Analysis:Analysis/src/NonStrictTypeChecker.cpp:208:non_strict_type_checker_flatten_pack`
//! Source: `Analysis/src/NonStrictTypeChecker.cpp:208-231` (hand-ported)

use crate::enums::polarity::Polarity;
use crate::functions::as_mutable_type_pack::as_mutable;
use crate::functions::emplace_type_pack::emplace_type_pack;
use crate::functions::finite::finite;
use crate::functions::first::first;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::fresh_index::fresh_index;
use crate::functions::get_type_pack::get;
use crate::functions::size_type_pack::size;
use crate::records::free_type::FreeType;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::records::type_level::TypeLevel;
use crate::records::type_pack::TypePack;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;

impl NonStrictTypeChecker {
    /// C++ `TypeId NonStrictTypeChecker::flattenPack(TypePackId pack)`.
    pub fn flatten_pack(&mut self, pack: TypePackId) -> TypeId {
        let pack = unsafe { follow_type_pack_id(pack) };

        if let Some(fst) = first(pack, /*ignoreHiddenVariadics*/ false) {
            return fst;
        }

        let ftp = unsafe { get::<FreeTypePack>(pack) };
        if !ftp.is_null() {
            let scope = unsafe { (*ftp).scope };
            let result = unsafe {
                (*self.arena).add_type(FreeType {
                    index: fresh_index(),
                    level: TypeLevel::default(),
                    scope,
                    forwarded_type_alias: false,
                    lower_bound: core::ptr::null(),
                    upper_bound: core::ptr::null(),
                    polarity: Polarity::Unknown,
                })
            };

            let free_tail = unsafe {
                (*self.arena).add_type_pack_t(FreeTypePack {
                    index: fresh_index(),
                    level: TypeLevel::default(),
                    scope,
                    polarity: Polarity::Unknown,
                })
            };

            let result_pack = unsafe {
                emplace_type_pack(
                    as_mutable(pack),
                    TypePackVariant::TypePack(TypePack {
                        head: alloc::vec![result],
                        tail: Some(free_tail),
                    }),
                )
            };
            let _ = result_pack;

            return result;
        }

        let error_pack = unsafe { get::<ErrorTypePack>(pack) };
        if !error_pack.is_null() {
            return unsafe { (*self.builtin_types).errorType };
        }

        if finite(pack, core::ptr::null_mut()) && size(pack, core::ptr::null_mut()) == 0 {
            return unsafe { (*self.builtin_types).nilType };
        }

        unsafe {
            (*self.ice).ice_string("flattenPack got a weird pack!");
        }
        unsafe { (*self.builtin_types).errorType }
    }
}
