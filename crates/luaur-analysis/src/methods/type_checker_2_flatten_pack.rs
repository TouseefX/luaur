//! Node: `cxx:Method:Luau.Analysis:Analysis/src/TypeChecker2.cpp:2723:type_checker_2_flatten_pack`
//! Source: `Analysis/src/TypeChecker2.cpp:2723-2749` (hand-ported)

use crate::enums::polarity::Polarity;
use crate::functions::as_mutable_type_pack::as_mutable;
use crate::functions::finite::finite;
use crate::functions::first::first;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::fresh_index::fresh_index;
use crate::functions::get_type_pack::get;
use crate::functions::size_type_pack::size;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::internal_error::InternalError;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_level::TypeLevel;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::type_error_data::IntoTypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use luaur_ast::records::location::Location;

impl TypeChecker2 {
    /// C++ `TypeId TypeChecker2::flattenPack(TypePackId pack)`.
    pub fn flatten_pack(&mut self, pack: TypePackId) -> TypeId {
        let pack = unsafe { follow_type_pack_id(pack) };

        if let Some(fst) = first(pack, /*ignoreHiddenVariadics*/ false) {
            return fst;
        }

        let ftp = unsafe { get::<FreeTypePack>(pack) };
        if !ftp.is_null() {
            let scope = unsafe { (*ftp).scope };
            let result = unsafe {
                (*self.module)
                    .internal_types
                    .fresh_type_not_null_builtin_types_scope(&*self.builtin_types, scope)
            };
            let free_tail = unsafe {
                (*self.module)
                    .internal_types
                    .add_type_pack_type_pack_var(TypePackVar {
                        ty: TypePackVariant::Free(FreeTypePack {
                            index: fresh_index(),
                            level: TypeLevel::default(),
                            scope,
                            polarity: Polarity::Unknown,
                        }),
                        persistent: false,
                        owningArena: core::ptr::null_mut(),
                    })
            };

            let result_pack = unsafe { &mut *as_mutable(pack) };
            result_pack.ty = TypePackVariant::TypePack(TypePack {
                head: alloc::vec![result],
                tail: Some(free_tail),
            });

            return result;
        }

        if !unsafe { get::<ErrorTypePack>(pack) }.is_null() {
            return unsafe { (*self.builtin_types).errorType };
        }

        if finite(pack, core::ptr::null_mut()) && size(pack, core::ptr::null_mut()) == 0 {
            // `(f())` where `f()` returns no values is coerced into `nil`
            return unsafe { (*self.builtin_types).nilType };
        }

        let err = InternalError::new(alloc::string::String::from("flattenPack got a weird pack!"));
        self.report_error_type_error_data_location(
            err.into_type_error_data(),
            &Location::default(),
        );
        unsafe { (*self.builtin_types).errorType }
    }
}
