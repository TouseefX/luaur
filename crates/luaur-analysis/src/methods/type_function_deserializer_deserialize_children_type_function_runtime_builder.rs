//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:866-898`
//!
//! Dispatch for `void deserializeChildren(TypeFunctionTypeId tfti, TypeId ty)`.
//! Each arm pairs the source `Type` variant (`getMutable<T>(ty)`) with the runtime
//! variant (`getMutable<TypeFunctionT>(tfti)`) and forwards to the leaf overload
//! `deserializeChildren(tfX, X)`. Table vs metatable share the runtime
//! `TypeFunctionTableType`, disambiguated by `metatable.has_value()`.
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_mutable_type_function_runtime_alt_g::get_mutable_type_function_type_id;
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
use crate::records::type_function_any_type::TypeFunctionAnyType;
use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::records::type_function_extern_type::TypeFunctionExternType;
use crate::records::type_function_function_type::TypeFunctionFunctionType;
use crate::records::type_function_generic_type::TypeFunctionGenericType;
use crate::records::type_function_intersection_type::TypeFunctionIntersectionType;
use crate::records::type_function_negation_type::TypeFunctionNegationType;
use crate::records::type_function_never_type::TypeFunctionNeverType;
use crate::records::type_function_primitive_type::TypeFunctionPrimitiveType;
use crate::records::type_function_singleton_type::TypeFunctionSingletonType;
use crate::records::type_function_table_type::TypeFunctionTableType;
use crate::records::type_function_union_type::TypeFunctionUnionType;
use crate::records::type_function_unknown_type::TypeFunctionUnknownType;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_id::TypeId;

impl TypeFunctionDeserializer {
    pub fn deserialize_children_type_function_type_id_type_id(
        &mut self,
        tfti: TypeFunctionTypeId,
        ty: TypeId,
    ) {
        unsafe {
            let p1 = get_mutable_type_id::<PrimitiveType>(ty);
            let p2 = get_mutable_type_function_type_id::<TypeFunctionPrimitiveType>(tfti);
            if !p1.is_null() && !p2.is_null() {
                self.deserialize_children_type_function_primitive_type_primitive_type(p2, p1);
                return;
            }

            let u1 = get_mutable_type_id::<UnknownType>(ty);
            let u2 = get_mutable_type_function_type_id::<TypeFunctionUnknownType>(tfti);
            if !u1.is_null() && !u2.is_null() {
                self.deserialize_children_type_function_unknown_type_unknown_type(u2, u1);
                return;
            }

            let n1 = get_mutable_type_id::<NeverType>(ty);
            let n2 = get_mutable_type_function_type_id::<TypeFunctionNeverType>(tfti);
            if !n1.is_null() && !n2.is_null() {
                self.deserialize_children_type_function_never_type_never_type(n2, n1);
                return;
            }

            let a1 = get_mutable_type_id::<AnyType>(ty);
            let a2 = get_mutable_type_function_type_id::<TypeFunctionAnyType>(tfti);
            if !a1.is_null() && !a2.is_null() {
                self.deserialize_children_type_function_any_type_any_type(a2, a1);
                return;
            }

            let s1 = get_mutable_type_id::<SingletonType>(ty);
            let s2 = get_mutable_type_function_type_id::<TypeFunctionSingletonType>(tfti);
            if !s1.is_null() && !s2.is_null() {
                self.deserialize_children_type_function_singleton_type_singleton_type(s2, s1);
                return;
            }

            let u1 = get_mutable_type_id::<UnionType>(ty);
            let u2 = get_mutable_type_function_type_id::<TypeFunctionUnionType>(tfti);
            if !u1.is_null() && !u2.is_null() {
                self.deserialize_children_type_function_union_type_union_type(u2, u1);
                return;
            }

            let i1 = get_mutable_type_id::<IntersectionType>(ty);
            let i2 = get_mutable_type_function_type_id::<TypeFunctionIntersectionType>(tfti);
            if !i1.is_null() && !i2.is_null() {
                self.deserialize_children_type_function_intersection_type_intersection_type(i2, i1);
                return;
            }

            let n1 = get_mutable_type_id::<NegationType>(ty);
            let n2 = get_mutable_type_function_type_id::<TypeFunctionNegationType>(tfti);
            if !n1.is_null() && !n2.is_null() {
                self.deserialize_children_type_function_negation_type_negation_type(n2, n1);
                return;
            }

            let t1 = get_mutable_type_id::<TableType>(ty);
            let t2 = get_mutable_type_function_type_id::<TypeFunctionTableType>(tfti);
            if !t1.is_null() && !t2.is_null() && (*t2).metatable.is_none() {
                self.deserialize_children_type_function_table_type_table_type(t2, t1);
                return;
            }

            let m1 = get_mutable_type_id::<MetatableType>(ty);
            let m2 = get_mutable_type_function_type_id::<TypeFunctionTableType>(tfti);
            if !m1.is_null() && !m2.is_null() && (*m2).metatable.is_some() {
                self.deserialize_children_type_function_table_type_metatable_type(m2, m1);
                return;
            }

            let f1 = get_mutable_type_id::<FunctionType>(ty);
            let f2 = get_mutable_type_function_type_id::<TypeFunctionFunctionType>(tfti);
            if !f1.is_null() && !f2.is_null() {
                self.deserialize_children_type_function_function_type_function_type(f2, f1);
                return;
            }

            let c1 = get_mutable_type_id::<ExternType>(ty);
            let c2 = get_mutable_type_function_type_id::<TypeFunctionExternType>(tfti);
            if !c1.is_null() && !c2.is_null() {
                self.deserialize_children_type_function_extern_type_extern_type(c2, c1);
                return;
            }

            let g1 = get_mutable_type_id::<GenericType>(ty);
            let g2 = get_mutable_type_function_type_id::<TypeFunctionGenericType>(tfti);
            if !g1.is_null() && !g2.is_null() {
                self.deserialize_children_type_function_generic_type_generic_type(g2, g1);
                return;
            }

            (*(*self.state).ctx)
                .ice
                .as_ref()
                .ice_string("Deserializing user defined type function arguments: mysterious type is being deserialized");
        }
    }
}
