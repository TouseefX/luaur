use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::table_type::TableType;
use crate::records::type_function_serializer::TypeFunctionSerializer;
use crate::records::type_function_type::TypeFunctionType;
use crate::records::type_function_type_pack_var::TypeFunctionTypePackVar;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant;
use crate::type_aliases::type_id::TypeId;

impl TypeFunctionSerializer {
    pub fn serialize_children_type_id_type_function_type_id(
        &mut self,
        ty: TypeId,
        tfti: TypeFunctionTypeId,
    ) {
        if tfti.is_null() {
            return;
        }

        let ty = unsafe { follow_type_id(ty) };

        unsafe {
            let target = &mut (*(tfti as *mut TypeFunctionType)).type_variant;

            if let Some(source) = get_type_id::<PrimitiveType>(ty).as_ref() {
                if let TypeFunctionTypeVariant::Primitive(target) = target {
                    self.serialize_children_primitive_type_type_function_primitive_type(
                        source, target,
                    );
                }
            } else if let Some(source) = get_type_id::<UnknownType>(ty).as_ref() {
                if let TypeFunctionTypeVariant::Unknown(target) = target {
                    self.serialize_children_unknown_type_type_function_unknown_type(source, target);
                }
            } else if let Some(source) = get_type_id::<NeverType>(ty).as_ref() {
                if let TypeFunctionTypeVariant::Never(target) = target {
                    self.serialize_children_never_type_type_function_never_type(source, target);
                }
            } else if let Some(source) = get_type_id::<AnyType>(ty).as_ref() {
                if let TypeFunctionTypeVariant::Any(target) = target {
                    self.serialize_children_any_type_type_function_any_type(source, target);
                }
            } else if let Some(source) = get_type_id::<SingletonType>(ty).as_ref() {
                if let TypeFunctionTypeVariant::Singleton(target) = target {
                    self.serialize_children_singleton_type_type_function_singleton_type(
                        source, target,
                    );
                }
            } else if let Some(source) = get_type_id::<UnionType>(ty).as_ref() {
                if let TypeFunctionTypeVariant::Union(target) = target {
                    self.serialize_children_union_type_type_function_union_type(source, target);
                }
            } else if let Some(source) = get_type_id::<IntersectionType>(ty).as_ref() {
                if let TypeFunctionTypeVariant::Intersection(target) = target {
                    self.serialize_children_intersection_type_type_function_intersection_type(
                        source, target,
                    );
                }
            } else if let Some(source) = get_type_id::<NegationType>(ty).as_ref() {
                if let TypeFunctionTypeVariant::Negation(target) = target {
                    self.serialize_children_negation_type_type_function_negation_type(
                        source, target,
                    );
                }
            } else if let Some(source) = get_type_id::<TableType>(ty).as_ref() {
                if let TypeFunctionTypeVariant::Table(target) = target {
                    self.serialize_children_table_type_type_function_table_type(source, target);
                }
            } else if let Some(source) = get_type_id::<MetatableType>(ty).as_ref() {
                if let TypeFunctionTypeVariant::Table(target) = target {
                    self.serialize_children_metatable_type_type_function_table_type(source, target);
                }
            } else if let Some(source) = get_type_id::<FunctionType>(ty).as_ref() {
                if let TypeFunctionTypeVariant::Function(target) = target {
                    self.serialize_children_function_type_type_function_function_type(
                        source, target,
                    );
                }
            } else if let Some(source) = get_type_id::<ExternType>(ty).as_ref() {
                if let TypeFunctionTypeVariant::Extern(target) = target {
                    self.serialize_children_extern_type_type_function_extern_type(source, target);
                }
            } else if let Some(source) = get_type_id::<GenericType>(ty).as_ref() {
                if let TypeFunctionTypeVariant::Generic(target) = target {
                    self.serialize_children_generic_type_type_function_generic_type(source, target);
                }
            }
        }
    }
}
