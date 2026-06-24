use crate::functions::get_mutable_type_function_runtime_alt_g::get_mutable_type_function_type_id;
use crate::records::type_function_cloner::TypeFunctionCloner;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeFunctionCloner {
    pub fn clone_children_type_function_type_id_type_function_type_id(
        &mut self,
        ty: TypeFunctionTypeId,
        tfti: TypeFunctionTypeId,
    ) {
        unsafe {
            if let Some((p1, p2)) = {
                let p1 = get_mutable_type_function_type_id::<
                    crate::records::type_function_primitive_type::TypeFunctionPrimitiveType,
                >(ty);
                let p2 = get_mutable_type_function_type_id::<
                    crate::records::type_function_primitive_type::TypeFunctionPrimitiveType,
                >(tfti);
                if !p1.is_null() && !p2.is_null() {
                    Some((p1, p2))
                } else {
                    None
                }
            } {
                self.clone_children_type_function_primitive_type_type_function_primitive_type(
                    p1, p2,
                );
            } else if let Some((u1, u2)) = {
                let u1 = get_mutable_type_function_type_id::<
                    crate::records::type_function_unknown_type::TypeFunctionUnknownType,
                >(ty);
                let u2 = get_mutable_type_function_type_id::<
                    crate::records::type_function_unknown_type::TypeFunctionUnknownType,
                >(tfti);
                if !u1.is_null() && !u2.is_null() {
                    Some((u1, u2))
                } else {
                    None
                }
            } {
                self.clone_children_type_function_unknown_type_type_function_unknown_type(u1, u2);
            } else if let Some((n1, n2)) = {
                let n1 = get_mutable_type_function_type_id::<
                    crate::records::type_function_never_type::TypeFunctionNeverType,
                >(ty);
                let n2 = get_mutable_type_function_type_id::<
                    crate::records::type_function_never_type::TypeFunctionNeverType,
                >(tfti);
                if !n1.is_null() && !n2.is_null() {
                    Some((n1, n2))
                } else {
                    None
                }
            } {
                self.clone_children_type_function_never_type_type_function_never_type(n1, n2);
            } else if let Some((a1, a2)) = {
                let a1 = get_mutable_type_function_type_id::<
                    crate::records::type_function_any_type::TypeFunctionAnyType,
                >(ty);
                let a2 = get_mutable_type_function_type_id::<
                    crate::records::type_function_any_type::TypeFunctionAnyType,
                >(tfti);
                if !a1.is_null() && !a2.is_null() {
                    Some((a1, a2))
                } else {
                    None
                }
            } {
                self.clone_children_type_function_any_type_type_function_any_type(a1, a2);
            } else if let Some((s1, s2)) = {
                let s1 = get_mutable_type_function_type_id::<
                    crate::records::type_function_singleton_type::TypeFunctionSingletonType,
                >(ty);
                let s2 = get_mutable_type_function_type_id::<
                    crate::records::type_function_singleton_type::TypeFunctionSingletonType,
                >(tfti);
                if !s1.is_null() && !s2.is_null() {
                    Some((s1, s2))
                } else {
                    None
                }
            } {
                self.clone_children_type_function_singleton_type_type_function_singleton_type(
                    s1, s2,
                );
            } else if let Some((u1, u2)) = {
                let u1 = get_mutable_type_function_type_id::<
                    crate::records::type_function_union_type::TypeFunctionUnionType,
                >(ty);
                let u2 = get_mutable_type_function_type_id::<
                    crate::records::type_function_union_type::TypeFunctionUnionType,
                >(tfti);
                if !u1.is_null() && !u2.is_null() {
                    Some((u1, u2))
                } else {
                    None
                }
            } {
                self.clone_children_type_function_union_type_type_function_union_type(u1, u2);
            } else if let Some((i1, i2)) = {
                let i1 = get_mutable_type_function_type_id::<
                    crate::records::type_function_intersection_type::TypeFunctionIntersectionType,
                >(ty);
                let i2 = get_mutable_type_function_type_id::<
                    crate::records::type_function_intersection_type::TypeFunctionIntersectionType,
                >(tfti);
                if !i1.is_null() && !i2.is_null() {
                    Some((i1, i2))
                } else {
                    None
                }
            } {
                self.clone_children_type_function_intersection_type_type_function_intersection_type(i1, i2);
            } else if let Some((n1, n2)) = {
                let n1 = get_mutable_type_function_type_id::<
                    crate::records::type_function_negation_type::TypeFunctionNegationType,
                >(ty);
                let n2 = get_mutable_type_function_type_id::<
                    crate::records::type_function_negation_type::TypeFunctionNegationType,
                >(tfti);
                if !n1.is_null() && !n2.is_null() {
                    Some((n1, n2))
                } else {
                    None
                }
            } {
                self.clone_children_type_function_negation_type_type_function_negation_type(n1, n2);
            } else if let Some((t1, t2)) = {
                let t1 = get_mutable_type_function_type_id::<
                    crate::records::type_function_table_type::TypeFunctionTableType,
                >(ty);
                let t2 = get_mutable_type_function_type_id::<
                    crate::records::type_function_table_type::TypeFunctionTableType,
                >(tfti);
                if !t1.is_null() && !t2.is_null() {
                    Some((t1, t2))
                } else {
                    None
                }
            } {
                self.clone_children_type_function_table_type_type_function_table_type(t1, t2);
            } else if let Some((f1, f2)) = {
                let f1 = get_mutable_type_function_type_id::<
                    crate::records::type_function_function_type::TypeFunctionFunctionType,
                >(ty);
                let f2 = get_mutable_type_function_type_id::<
                    crate::records::type_function_function_type::TypeFunctionFunctionType,
                >(tfti);
                if !f1.is_null() && !f2.is_null() {
                    Some((f1, f2))
                } else {
                    None
                }
            } {
                self.clone_children_type_function_function_type_type_function_function_type(f1, f2);
            } else if let Some((c1, c2)) = {
                let c1 = get_mutable_type_function_type_id::<
                    crate::records::type_function_extern_type::TypeFunctionExternType,
                >(ty);
                let c2 = get_mutable_type_function_type_id::<
                    crate::records::type_function_extern_type::TypeFunctionExternType,
                >(tfti);
                if !c1.is_null() && !c2.is_null() {
                    Some((c1, c2))
                } else {
                    None
                }
            } {
                self.clone_children_type_function_extern_type_type_function_extern_type(c1, c2);
            } else if let Some((g1, g2)) = {
                let g1 = get_mutable_type_function_type_id::<
                    crate::records::type_function_generic_type::TypeFunctionGenericType,
                >(ty);
                let g2 = get_mutable_type_function_type_id::<
                    crate::records::type_function_generic_type::TypeFunctionGenericType,
                >(tfti);
                if !g1.is_null() && !g2.is_null() {
                    Some((g1, g2))
                } else {
                    None
                }
            } {
                self.clone_children_type_function_generic_type_type_function_generic_type(g1, g2);
            } else {
                LUAU_ASSERT!(false);
            }
        }
    }
}
