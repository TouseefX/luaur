use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_nil::is_nil;
use crate::functions::is_prim::is_prim;
use crate::records::any_type::AnyType;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::free_type::FreeType;
use crate::records::primitive_type::Type as PrimType;
use crate::records::singleton_type::SingletonType;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_id_predicate::TypeIdPredicate;

impl TypeChecker {
    /// `TypeIdPredicate TypeChecker::mkTruthyPredicate(bool sense, TypeId emptySetTy)`.
    /// Reference: `Analysis/src/TypeInfer.cpp:5544`.
    pub fn mk_truthy_predicate(&mut self, sense: bool, empty_set_ty: TypeId) -> TypeIdPredicate {
        // C++ captures `this` and calls `singletonType(sense)`; those are the builtin
        // true/false types, so we capture them directly to keep the closure `Fn`.
        let true_type = unsafe { (*self.builtin_types).trueType };
        let false_type = unsafe { (*self.builtin_types).falseType };

        alloc::boxed::Box::new(move |ty: TypeId| -> Option<TypeId> {
            // any/error/free gets a special pass unconditionally because they can't be decided.
            if unsafe {
                !get_type_id::<AnyType>(ty).is_null()
                    || !get_type_id::<ErrorType>(ty).is_null()
                    || !get_type_id::<FreeType>(ty).is_null()
            } {
                return Some(ty);
            }

            // maps boolean primitive to the corresponding singleton equal to sense
            if is_prim(ty, PrimType::Boolean) {
                return Some(if sense { true_type } else { false_type });
            }

            // if we have boolean singleton, eliminate it if the sense doesn't match with that singleton
            let stv = unsafe { get_type_id::<SingletonType>(ty) };
            if !stv.is_null() {
                if let Some(boolean) = unsafe { (*stv).variant.get_if::<BooleanSingleton>() } {
                    return if boolean.value == sense { Some(ty) } else { None };
                }
            }

            // if we have nil, eliminate it if sense is true, otherwise take it
            if is_nil(ty) {
                return if sense { None } else { Some(ty) };
            }

            // at this point, anything else is kept if sense is true, or replaced by emptySetTy
            if sense {
                Some(ty)
            } else {
                Some(empty_set_ty)
            }
        })
    }
}
