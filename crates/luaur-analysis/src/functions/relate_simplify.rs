use crate::enums::relation::Relation;
use crate::functions::flip::flip;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_subclass_normalize::is_subclass_type_id_type_id;
use crate::functions::is_type_variable::is_type_variable;
use crate::functions::relate_table_to_extern_type::relate_table_to_extern_type;
use crate::functions::relate_tables::relate_tables;
use crate::records::any_type::AnyType;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::negation_type::NegationType;
use crate::records::never_type::NeverType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::table_type::TableType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::simplifier_seen_set::SimplifierSeenSet;
use crate::type_aliases::type_id::TypeId;

// A cheap and approximate subtype test
pub fn relate(left: TypeId, right: TypeId, seen: &mut SimplifierSeenSet) -> Relation {
    // TODO nice to have: Relate functions of equal argument and return arity

    let left = unsafe { follow_type_id(left) };
    let right = unsafe { follow_type_id(right) };

    if left == right {
        return Relation::Coincident;
    }

    let type_pair = (left, right);
    if !seen.try_insert(type_pair, true).1 {
        // TODO: is this right at all?
        // The thinking here is that this is a cycle if we get here, and therefore its coincident.
        return Relation::Coincident;
    }

    unsafe {
        if !get_type_id::<UnknownType>(left).is_null() {
            if !get_type_id::<AnyType>(right).is_null() {
                return Relation::Subset;
            }

            if !get_type_id::<UnknownType>(right).is_null() {
                return Relation::Coincident;
            }

            if !get_type_id::<ErrorType>(right).is_null() {
                return Relation::Disjoint;
            }

            return Relation::Superset;
        }

        if !get_type_id::<UnknownType>(right).is_null() {
            return flip(relate(right, left, seen));
        }

        if !get_type_id::<AnyType>(left).is_null() {
            if !get_type_id::<AnyType>(right).is_null() {
                return Relation::Coincident;
            }

            return Relation::Superset;
        }

        if !get_type_id::<AnyType>(right).is_null() {
            return flip(relate(right, left, seen));
        }

        // Type variables
        // * FreeType
        // * GenericType
        // * BlockedType
        // * PendingExpansionType

        // Tops and bottoms
        // * ErrorType
        // * AnyType
        // * NeverType
        // * UnknownType

        // Concrete
        // * PrimitiveType
        // * SingletonType
        // * FunctionType
        // * TableType
        // * MetatableType
        // * ExternType
        // * UnionType
        // * IntersectionType
        // * NegationType

        if is_type_variable(left) || is_type_variable(right) {
            return Relation::Intersects;
        }

        // if either type is a type function, we cannot know if they'll be related.
        if !get_type_id::<TypeFunctionInstanceType>(left).is_null()
            || !get_type_id::<TypeFunctionInstanceType>(right).is_null()
        {
            return Relation::Intersects;
        }

        if !get_type_id::<ErrorType>(left).is_null() {
            if !get_type_id::<ErrorType>(right).is_null() {
                return Relation::Coincident;
            } else if !get_type_id::<AnyType>(right).is_null() {
                return Relation::Subset;
            }

            return Relation::Disjoint;
        } else if !get_type_id::<ErrorType>(right).is_null() {
            return flip(relate(right, left, seen));
        }

        if !get_type_id::<NeverType>(left).is_null() {
            if !get_type_id::<NeverType>(right).is_null() {
                return Relation::Coincident;
            }

            return Relation::Subset;
        } else if !get_type_id::<NeverType>(right).is_null() {
            return flip(relate(right, left, seen));
        }

        if !get_type_id::<IntersectionType>(left).is_null() {
            return Relation::Intersects;
        } else if !get_type_id::<IntersectionType>(right).is_null() {
            return Relation::Intersects;
        }

        if let Some(ut) = get_type_id::<UnionType>(left).as_ref() {
            for &part in &ut.options {
                let r = relate(part, right, seen);
                if r == Relation::Superset || r == Relation::Coincident {
                    return Relation::Superset;
                }
            }
            return Relation::Intersects;
        } else if let Some(ut) = get_type_id::<UnionType>(right).as_ref() {
            for &part in &ut.options {
                let r = relate(left, part, seen);
                if r == Relation::Subset || r == Relation::Coincident {
                    return Relation::Subset;
                }
            }
            return Relation::Intersects;
        }

        if let Some(rnt) = get_type_id::<NegationType>(right).as_ref() {
            let a = relate(left, rnt.ty, seen);
            match a {
                Relation::Coincident => {
                    // number & ~number
                    return Relation::Disjoint;
                }
                Relation::Disjoint => {
                    if !get_type_id::<NegationType>(left).is_null() {
                        // ~number & ~string
                        return Relation::Intersects;
                    } else {
                        // number & ~string
                        return Relation::Subset;
                    }
                }
                Relation::Intersects => {
                    // ~(false?) & ~boolean
                    return Relation::Intersects;
                }
                Relation::Subset => {
                    // "hello" & ~string
                    return Relation::Disjoint;
                }
                Relation::Superset => {
                    // ~function & ~(false?)  -> ~function
                    // boolean & ~(false?)    -> true
                    // string & ~"hello"      -> string & ~"hello"
                    return Relation::Intersects;
                }
            }
        } else if !get_type_id::<NegationType>(left).is_null() {
            return flip(relate(right, left, seen));
        }

        if let Some(lp) = get_type_id::<PrimitiveType>(left).as_ref() {
            if let Some(rp) = get_type_id::<PrimitiveType>(right).as_ref() {
                if lp.r#type == rp.r#type {
                    return Relation::Coincident;
                }

                return Relation::Disjoint;
            }

            if let Some(rs) = get_type_id::<SingletonType>(right).as_ref() {
                if lp.r#type == PrimitiveType::String
                    && rs.variant.get_if::<StringSingleton>().is_some()
                {
                    return Relation::Superset;
                }

                if lp.r#type == PrimitiveType::Boolean
                    && rs.variant.get_if::<BooleanSingleton>().is_some()
                {
                    return Relation::Superset;
                }

                return Relation::Disjoint;
            }

            if lp.r#type == PrimitiveType::Function {
                if !get_type_id::<FunctionType>(right).is_null() {
                    return Relation::Superset;
                }

                return Relation::Disjoint;
            }
            if lp.r#type == PrimitiveType::Table {
                if !get_type_id::<TableType>(right).is_null() {
                    return Relation::Superset;
                }

                return Relation::Disjoint;
            }

            if !get_type_id::<FunctionType>(right).is_null()
                || !get_type_id::<TableType>(right).is_null()
                || !get_type_id::<MetatableType>(right).is_null()
                || !get_type_id::<ExternType>(right).is_null()
            {
                return Relation::Disjoint;
            }
        }

        if let Some(ls) = get_type_id::<SingletonType>(left).as_ref() {
            if !get_type_id::<FunctionType>(right).is_null()
                || !get_type_id::<TableType>(right).is_null()
                || !get_type_id::<MetatableType>(right).is_null()
                || !get_type_id::<ExternType>(right).is_null()
            {
                return Relation::Disjoint;
            }

            if !get_type_id::<PrimitiveType>(right).is_null() {
                return flip(relate(right, left, seen));
            }

            if let Some(rs) = get_type_id::<SingletonType>(right).as_ref() {
                if ls.variant == rs.variant {
                    return Relation::Coincident;
                }

                return Relation::Disjoint;
            }
        }

        if !get_type_id::<FunctionType>(left).is_null() {
            if let Some(rp) = get_type_id::<PrimitiveType>(right).as_ref() {
                if rp.r#type == PrimitiveType::Function {
                    return Relation::Subset;
                }

                return Relation::Disjoint;
            }

            return Relation::Intersects;
        }

        if let Some(lt) = get_type_id::<TableType>(left).as_ref() {
            if let Some(rp) = get_type_id::<PrimitiveType>(right).as_ref() {
                if rp.r#type == PrimitiveType::Table {
                    return Relation::Subset;
                }

                return Relation::Disjoint;
            }

            if let Some(rt) = get_type_id::<TableType>(right).as_ref() {
                return relate_tables(lt, rt, seen);
            }

            if let Some(re) = get_type_id::<ExternType>(right).as_ref() {
                return relate_table_to_extern_type(lt, re, seen);
            }

            // TODO metatables

            return Relation::Disjoint;
        }

        if let Some(ct) = get_type_id::<ExternType>(left).as_ref() {
            if get_type_id::<ExternType>(right).as_ref().is_some() {
                if is_subclass_type_id_type_id(left, right) {
                    return Relation::Subset;
                }

                if is_subclass_type_id_type_id(right, left) {
                    return Relation::Superset;
                }

                return Relation::Disjoint;
            }

            if let Some(tbl) = get_type_id::<TableType>(right).as_ref() {
                return flip(relate_table_to_extern_type(tbl, ct, seen));
            }

            return Relation::Disjoint;
        }
    }

    Relation::Intersects
}
