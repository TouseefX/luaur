use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::get_mutable;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_approximately_falsy_type::is_approximately_falsy_type;
use crate::functions::is_approximately_truthy_type::is_approximately_truthy_type;
use crate::functions::is_type_variable::is_type_variable;
use crate::functions::shallow_clone_clone_alt_b::shallow_clone;
use crate::records::any_type::AnyType;
use crate::records::clone_state::CloneState;
use crate::records::error_type::ErrorType;
use crate::records::intersection_type::IntersectionType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::table_type::TableType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_ids::TypeIds;
use crate::records::type_simplifier::TypeSimplifier;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl TypeSimplifier {
    pub fn intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(
        &self,
        target: TypeId,
        discriminant: TypeId,
        seen: &mut DenseHashSet<TypeId>,
    ) -> Option<TypeId> {
        if seen.contains(&target) {
            return None;
        }
        let target = unsafe { follow_type_id(target) };
        let discriminant = unsafe { follow_type_id(discriminant) };
        if seen.contains(&target) {
            return None;
        }

        if let Some(ut) = unsafe { get_type_id::<UnionType>(target).as_ref() } {
            seen.insert(target);
            let mut options = TypeIds::type_ids();
            for &option in &ut.options {
                let res = self
                    .intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(
                        option,
                        discriminant,
                        seen,
                    )?;
                if !unsafe { get_type_id::<UnknownType>(res).is_null() } {
                    return Some(unsafe { (*self.builtin_types).unknownType });
                }
                if !unsafe { get_type_id::<NeverType>(res).is_null() } {
                    continue;
                }
                options.insert_type_id(res);
            }
            if options.empty() {
                return Some(unsafe { (*self.builtin_types).neverType });
            }
            if options.size() == 1 {
                return Some(options.front());
            }
            return Some(unsafe {
                (*self.arena.cast_mut()).add_type(UnionType {
                    options: options.take(),
                })
            });
        }

        if let Some(it) = unsafe { get_type_id::<IntersectionType>(target).as_ref() } {
            seen.insert(target);
            let mut parts = TypeIds::type_ids();
            for &part in &it.parts {
                let res = self
                    .intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(
                        part,
                        discriminant,
                        seen,
                    )?;

                if !unsafe { get_type_id::<NeverType>(res).is_null() } {
                    return Some(unsafe { (*self.builtin_types).neverType });
                }

                if let Some(sub_intersection) =
                    unsafe { get_type_id::<IntersectionType>(res).as_ref() }
                {
                    for &sub_option in &sub_intersection.parts {
                        if !unsafe { get_type_id::<NeverType>(sub_option).is_null() } {
                            return Some(unsafe { (*self.builtin_types).neverType });
                        }
                        if unsafe { get_type_id::<UnknownType>(sub_option).is_null() } {
                            parts.insert_type_id(sub_option);
                        }
                    }
                } else if unsafe { get_type_id::<UnknownType>(res).is_null() } {
                    parts.insert_type_id(res);
                }
            }

            if parts.empty() {
                return Some(unsafe { (*self.builtin_types).unknownType });
            }
            if parts.size() == 1 {
                return Some(parts.front());
            }
            return Some(unsafe {
                (*self.arena.cast_mut()).add_type(IntersectionType {
                    parts: parts.take(),
                })
            });
        }

        if let Some(ttv) = unsafe { get_type_id::<TableType>(target).as_ref() } {
            if let Some(disc_ttv) = unsafe { get_type_id::<TableType>(discriminant).as_ref() } {
                // The precondition of this function is that `discriminant` is
                // simple, so if it's a table it *must* be a sealed table with
                // a single property and no indexer.
                LUAU_ASSERT!(disc_ttv.props.len() == 1 && disc_ttv.indexer.is_none());
                let (disc_prop_name, disc_prop) = disc_ttv.props.iter().next().unwrap();
                if let Some(ty_prop) = ttv.props.get(disc_prop_name) {
                    let property = self.intersect_property(ty_prop, disc_prop, seen)?;
                    if let Some(read_ty) = property.read_ty {
                        if !unsafe { get_type_id::<NeverType>(follow_type_id(read_ty)) }.is_null() {
                            return Some(unsafe { (*self.builtin_types).neverType });
                        }
                    }
                    if let Some(write_ty) = property.write_ty {
                        if !unsafe { get_type_id::<NeverType>(follow_type_id(write_ty)) }.is_null()
                        {
                            return Some(unsafe { (*self.builtin_types).neverType });
                        }
                    }

                    // If the property we get back is pointer identical to the
                    // original property, return the underlying property as an
                    // optimization.
                    if ty_prop.read_ty == property.read_ty && ty_prop.write_ty == property.write_ty
                    {
                        return Some(target);
                    }

                    let mut cs = CloneState::new(unsafe { &mut *self.builtin_types.cast_mut() });
                    let result = shallow_clone(
                        target,
                        unsafe { &mut *self.arena.cast_mut() },
                        &mut cs,
                        /* clonePersistentTypes */ true,
                    );
                    let result_ttv = unsafe { get_mutable::<TableType>(result) };
                    LUAU_ASSERT!(!result_ttv.is_null());
                    unsafe {
                        (*result_ttv).props.insert(disc_prop_name.clone(), property);
                        // Shallow cloning clears out scopes, so let's put back the
                        // scope from the original type.
                        (*result_ttv).scope = ttv.scope;
                    }
                    return Some(result);
                }

                let mut cs = CloneState::new(unsafe { &mut *self.builtin_types.cast_mut() });
                let result = shallow_clone(
                    target,
                    unsafe { &mut *self.arena.cast_mut() },
                    &mut cs,
                    /* clonePersistentTypes */ true,
                );
                let result_ttv = unsafe { get_mutable::<TableType>(result) };
                LUAU_ASSERT!(!result_ttv.is_null());
                unsafe {
                    // C++ `props.emplace` only inserts if the key is absent; the
                    // `ty_prop` lookup above already established it is absent here.
                    (*result_ttv)
                        .props
                        .entry(disc_prop_name.clone())
                        .or_insert_with(|| disc_prop.clone());
                    // Shallow cloning clears out scopes, so let's put back the
                    // scope from the original type.
                    (*result_ttv).scope = ttv.scope;
                }
                return Some(result);
            }

            // At this point, we're doing something like:
            //
            //  { ... } & ~nil
            //
            // Which can be handled via fallthrough.
        }

        if is_type_variable(target)
            || !unsafe { get_type_id::<TypeFunctionInstanceType>(target) }.is_null()
        {
            return None;
        }

        if is_approximately_truthy_type(discriminant) {
            return self.basic_intersect_with_truthy(target);
        }
        if is_approximately_truthy_type(target) {
            return self.basic_intersect_with_truthy(discriminant);
        }
        if is_approximately_falsy_type(discriminant) {
            return self.basic_intersect_with_falsy(target);
        }
        if is_approximately_falsy_type(target) {
            return self.basic_intersect_with_falsy(discriminant);
        }

        if !unsafe { get_type_id::<AnyType>(target) }.is_null() {
            return Some(unsafe {
                (*self.arena.cast_mut()).add_type(UnionType {
                    options: alloc::vec![(*self.builtin_types).errorType, discriminant],
                })
            });
        }
        if !unsafe { get_type_id::<ErrorType>(target) }.is_null() {
            return Some(unsafe { (*self.builtin_types).errorType });
        }
        if let Some(nty) = unsafe { get_type_id::<NegationType>(discriminant).as_ref() } {
            return self.subtract_one(target, nty.ty);
        }

        self.intersect_one(target, discriminant)
    }
}
