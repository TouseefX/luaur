use crate::enums::relation::Relation;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::relate_simplify_alt_b::relate_type_id_type_id;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::intersection_type::IntersectionType;
use crate::records::negation_type::NegationType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::type_ids::TypeIds;
use crate::records::type_simplifier::TypeSimplifier;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use alloc::collections::BTreeSet;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeSimplifier {
    pub fn intersect_type_with_negation(&mut self, left: TypeId, right: TypeId) -> TypeId {
        let builtin_types = unsafe { &*self.builtin_types };

        let left_negation = unsafe { get_type_id::<NegationType>(left) };
        LUAU_ASSERT!(!left_negation.is_null());
        let left_negation = unsafe { &*left_negation };

        let negated_ty = unsafe { follow_type_id(left_negation.ty) };

        if negated_ty == right {
            return builtin_types.neverType;
        }

        if let Some(ut) = unsafe { get_type_id::<UnionType>(negated_ty).as_ref() } {
            // ~(A | B) & C
            // (~A & C) & (~B & C)
            let mut changed = false;
            let mut new_parts = TypeIds::type_ids();

            for &part in &ut.options {
                let r = relate_type_id_type_id(part, right);
                match r {
                    // ~(false?) & nil
                    // (~false & nil) & (~nil & nil)
                    // nil & never
                    //
                    // fallthrough
                    Relation::Coincident => {
                        // ~(boolean | string) & true
                        // (~boolean & true) & (~boolean & string)
                        // never & string
                        return builtin_types.neverType;
                    }
                    Relation::Superset => {
                        return builtin_types.neverType;
                    }
                    Relation::Disjoint => {
                        // ~nil & boolean
                        new_parts.insert_type_id(right);
                    }
                    // ~false & boolean
                    // fallthrough
                    Relation::Subset | Relation::Intersects => {
                        // FIXME: The mkNegation here is pretty unfortunate.
                        // Memoizing this will probably be important.
                        changed = true;
                        new_parts.insert_type_id(right);
                        let negation = self.mk_negation(part);
                        new_parts.insert_type_id(negation);
                    }
                }
            }

            if !changed {
                return right;
            }

            return self.intersect_from_parts(new_parts);
        }

        if let Some(right_union) = unsafe { get_type_id::<UnionType>(right).as_ref() } {
            // ~A & (B | C)
            let mut changed = false;
            let mut new_parts: BTreeSet<TypeId> = BTreeSet::new();

            for &part in &right_union.options {
                let r = relate_type_id_type_id(negated_ty, part);
                match r {
                    Relation::Coincident => {
                        changed = true;
                        continue;
                    }
                    Relation::Disjoint => {
                        new_parts.insert(part);
                    }
                    Relation::Superset => {
                        changed = true;
                        continue;
                    }
                    // fallthrough
                    Relation::Subset | Relation::Intersects => {
                        changed = true;
                        let arena = unsafe { &mut *self.arena.cast_mut() };
                        new_parts.insert(arena.add_type(IntersectionType {
                            parts: alloc::vec![left, part],
                        }));
                    }
                }
            }

            if !changed {
                return right;
            } else if new_parts.is_empty() {
                return builtin_types.neverType;
            } else if new_parts.len() == 1 {
                return *new_parts.iter().next().unwrap();
            } else {
                let arena = unsafe { &mut *self.arena.cast_mut() };
                return arena.add_type(UnionType {
                    options: new_parts.into_iter().collect(),
                });
            }
        }

        if let Some(pt) = unsafe { get_type_id::<PrimitiveType>(right).as_ref() } {
            if pt.r#type == PrimitiveType::Boolean {
                if let Some(st) = unsafe { get_type_id::<SingletonType>(negated_ty).as_ref() } {
                    if st.variant
                        == luaur_common::records::variant::Variant2::V0(BooleanSingleton::new(true))
                    {
                        return builtin_types.falseType;
                    } else if st.variant
                        == luaur_common::records::variant::Variant2::V0(BooleanSingleton::new(false))
                    {
                        return builtin_types.trueType;
                    } else {
                        // boolean & ~"hello"
                        return builtin_types.booleanType;
                    }
                }
            }
        }

        let r = relate_type_id_type_id(negated_ty, right);

        match r {
            Relation::Disjoint => {
                // ~boolean & string
                right
            }
            // ~string & string
            // fallthrough
            Relation::Coincident => {
                // ~string & "hello"
                builtin_types.neverType
            }
            Relation::Superset => builtin_types.neverType,
            // ~string & unknown
            // ~"hello" & string
            // fallthrough
            // ~("hello" | boolean) & string
            // fallthrough
            // default
            Relation::Subset | Relation::Intersects => {
                let arena = unsafe { &mut *self.arena.cast_mut() };
                arena.add_type(IntersectionType {
                    parts: alloc::vec![left, right],
                })
            }
        }
    }
}
