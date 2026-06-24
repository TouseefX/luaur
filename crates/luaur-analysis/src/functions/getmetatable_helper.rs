use crate::enums::reduction::Reduction;
use crate::functions::find_metatable_entry::find_metatable_entry;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::extern_type::ExternType;
use crate::records::metatable_type::MetatableType;
use crate::records::primitive_type::{PrimitiveType, Type as PrimitiveKind};
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::table_type::TableType;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::records::union_type::UnionType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::singleton_variant::SingletonVariantMember;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;

pub fn getmetatable_helper(
    target_ty: TypeId,
    location: &Location,
    ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    let ctx_ref = unsafe { &*ctx };
    let target_ty = unsafe { follow_type_id(target_ty) };

    let mut result: Option<TypeId> = None;
    let mut erroneous = true;

    if unsafe { get_type_id::<TableType>(target_ty).as_ref().is_some() } {
        erroneous = false;
    }

    if let Some(mt) = unsafe { get_type_id::<MetatableType>(target_ty).as_ref() } {
        result = Some(mt.metatable());
        erroneous = false;
    }

    if let Some(class_type) = unsafe { get_type_id::<ExternType>(target_ty).as_ref() } {
        result = class_type.metatable;
        erroneous = false;
    }

    if let Some(primitive) = unsafe { get_type_id::<PrimitiveType>(target_ty).as_ref() } {
        if primitive.r#type == PrimitiveKind::Table {
            result = Some(unsafe {
                (*ctx_ref.arena.as_ptr()).add_type(UnionType {
                    options: Vec::from([
                        (*ctx_ref.builtins.as_ptr()).tableType,
                        (*ctx_ref.builtins.as_ptr()).nilType,
                    ]),
                })
            });
        } else {
            result = primitive.metatable;
        }
        erroneous = false;
    }

    if let Some(singleton) = unsafe { get_type_id::<SingletonType>(target_ty).as_ref() } {
        if StringSingleton::get_if(&singleton.variant).is_some() {
            let primitive_string = unsafe {
                get_type_id::<PrimitiveType>((*ctx_ref.builtins.as_ptr()).stringType).as_ref()
            };
            if let Some(primitive_string) = primitive_string {
                result = primitive_string.metatable;
            }
        }
        erroneous = false;
    }

    if unsafe { get_type_id::<AnyType>(target_ty).as_ref().is_some() } {
        result = Some(target_ty);
        erroneous = false;
    }

    if unsafe { get_type_id::<ErrorType>(target_ty).as_ref().is_some() } {
        result = Some(target_ty);
        erroneous = false;
    }

    if erroneous {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::Erroneous,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    let mut dummy = Vec::new();
    let metatable_metamethod = find_metatable_entry(
        ctx_ref.builtins.as_ptr(),
        &mut dummy,
        target_ty,
        "__metatable",
        *location,
    );

    if let Some(metatable_metamethod) = metatable_metamethod {
        return TypeFunctionReductionResult {
            result: Some(metatable_metamethod),
            reduction_status: Reduction::MaybeOk,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    TypeFunctionReductionResult {
        result: Some(result.unwrap_or(unsafe { (*ctx_ref.builtins.as_ptr()).nilType })),
        reduction_status: Reduction::MaybeOk,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
    }
}
