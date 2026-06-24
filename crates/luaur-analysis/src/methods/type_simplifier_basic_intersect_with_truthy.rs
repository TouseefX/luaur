use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_approximately_falsy_type::is_approximately_falsy_type;
use crate::functions::is_approximately_truthy_type::is_approximately_truthy_type;
use crate::records::any_type::AnyType;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::error_type::ErrorType;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::metatable_type::MetatableType;
use crate::records::never_type::NeverType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::table_type::TableType;
use crate::records::type_simplifier::TypeSimplifier;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;

impl TypeSimplifier {
    pub fn basic_intersect_with_truthy(&self, target: TypeId) -> Option<TypeId> {
        let builtin_types = unsafe { &*self.builtin_types };
        let target = unsafe { follow_type_id(target) };

        if is_approximately_truthy_type(target) {
            return Some(target);
        }

        if is_approximately_falsy_type(target) {
            return Some(builtin_types.neverType);
        }

        if !unsafe { get_type_id::<UnknownType>(target) }.is_null() {
            return Some(builtin_types.truthyType);
        }

        if !unsafe { get_type_id::<AnyType>(target) }.is_null() {
            // any = *error-type* | unknown, so truthy & any = *error-type* | truthy
            let arena = unsafe { &mut *self.arena.cast_mut() };
            return Some(arena.add_type(UnionType {
                options: alloc::vec![builtin_types.truthyType, builtin_types.errorType],
            }));
        }

        if !unsafe { get_type_id::<NeverType>(target) }.is_null()
            || !unsafe { get_type_id::<ErrorType>(target) }.is_null()
        {
            return Some(target);
        }

        if !unsafe { get_type_id::<FunctionType>(target) }.is_null()
            || !unsafe { get_type_id::<TableType>(target) }.is_null()
            || !unsafe { get_type_id::<MetatableType>(target) }.is_null()
            || !unsafe { get_type_id::<ExternType>(target) }.is_null()
        {
            return Some(target);
        }

        if let Some(pt) = unsafe { get_type_id::<PrimitiveType>(target).as_ref() } {
            return Some(match pt.r#type {
                PrimitiveType::NilType => builtin_types.neverType,
                PrimitiveType::Boolean => builtin_types.trueType,
                _ => target,
            });
        }

        if let Some(st) = unsafe { get_type_id::<SingletonType>(target).as_ref() } {
            return Some(
                if st.variant
                    == luaur_common::records::variant::Variant2::V0(BooleanSingleton::new(false))
                {
                    builtin_types.neverType
                } else {
                    target
                },
            );
        }

        None
    }
}
