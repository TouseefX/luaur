//! `TypeFunctionReducer::replace<T>` (TypeFunction.cpp:324-344).
//!
//! C++ is a single template specialized on `TypeId`/`TypePackId`. The result
//! struct in this crate is monomorphized on `TypeId`, so the two
//! specializations are rendered as the concrete `replace_type_id` and
//! `replace_type_pack_id` methods. `asMutable(subject)->ty.emplace<Bound<T>>`
//! becomes assigning the `Bound` variant of the type/type-pack variant.

use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::as_mutable_type_pack::as_mutable_type_pack_id;
use crate::records::internal_error::InternalError;
use crate::records::type_error::TypeError;
use crate::records::type_function_reducer::TypeFunctionReducer;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use crate::type_aliases::type_variant::TypeVariant;

impl TypeFunctionReducer {
    pub fn replace_type_id(&mut self, subject: TypeId, replacement: TypeId) {
        unsafe {
            if (*subject).owning_arena != (*self.ctx.as_ptr()).arena.as_ptr() {
                self.result
                    .errors
                    .push(TypeError::type_error_location_type_error_data(
                        self.location,
                        TypeErrorData::InternalError(InternalError::new(
                            alloc::string::String::from(
                                "Attempting to modify a type function instance from another arena",
                            ),
                        )),
                    ));
                return;
            }

            // asMutable(subject)->ty.emplace<Unifiable::Bound<TypeId>>(replacement);
            (*as_mutable_type_id(subject)).ty = TypeVariant::Bound(replacement);
        }

        self.result.reduced_types.insert(subject);
    }

    pub fn replace_type_pack_id(&mut self, subject: TypePackId, replacement: TypePackId) {
        unsafe {
            if (*subject).owningArena != (*self.ctx.as_ptr()).arena.as_ptr() {
                self.result
                    .errors
                    .push(TypeError::type_error_location_type_error_data(
                        self.location,
                        TypeErrorData::InternalError(InternalError::new(
                            alloc::string::String::from(
                                "Attempting to modify a type function instance from another arena",
                            ),
                        )),
                    ));
                return;
            }

            // asMutable(subject)->ty.emplace<Unifiable::Bound<TypePackId>>(replacement);
            (*as_mutable_type_pack_id(subject)).ty = TypePackVariant::Bound(replacement);
        }

        self.result.reduced_packs.insert(subject);
    }
}
