//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/Constraint.h:318:constraint_v`
//! Source: `Analysis/include/Luau/Constraint.h:318-341` (hand-ported)
use crate::records::assign_index_constraint::AssignIndexConstraint;
use crate::records::assign_prop_constraint::AssignPropConstraint;
use crate::records::equality_constraint::EqualityConstraint;
use crate::records::function_call_constraint::FunctionCallConstraint;
use crate::records::function_check_constraint::FunctionCheckConstraint;
use crate::records::generalization_constraint::GeneralizationConstraint;
use crate::records::has_indexer_constraint::HasIndexerConstraint;
use crate::records::has_prop_constraint::HasPropConstraint;
use crate::records::iterable_constraint::IterableConstraint;
use crate::records::name_constraint::NameConstraint;
use crate::records::pack_subtype_constraint::PackSubtypeConstraint;
use crate::records::primitive_type_constraint::PrimitiveTypeConstraint;
use crate::records::push_function_type_constraint::PushFunctionTypeConstraint;
use crate::records::push_type_constraint::PushTypeConstraint;
use crate::records::reduce_constraint::ReduceConstraint;
use crate::records::reduce_pack_constraint::ReducePackConstraint;
use crate::records::simplify_constraint::SimplifyConstraint;
use crate::records::subtype_constraint::SubtypeConstraint;
use crate::records::type_alias_expansion_constraint::TypeAliasExpansionConstraint;
use crate::records::type_instantiation_constraint::TypeInstantiationConstraint;
use crate::records::unpack_constraint::UnpackConstraint;

// 21 members exceed Variant7 -> custom enum (TypeVariant precedent).
#[derive(Debug, Clone)]
pub enum ConstraintV {
    Subtype(SubtypeConstraint),
    PackSubtype(PackSubtypeConstraint),
    Generalization(GeneralizationConstraint),
    Iterable(IterableConstraint),
    Name(NameConstraint),
    TypeAliasExpansion(TypeAliasExpansionConstraint),
    FunctionCall(FunctionCallConstraint),
    FunctionCheck(FunctionCheckConstraint),
    PrimitiveType(PrimitiveTypeConstraint),
    HasProp(HasPropConstraint),
    HasIndexer(HasIndexerConstraint),
    AssignProp(AssignPropConstraint),
    AssignIndex(AssignIndexConstraint),
    Unpack(UnpackConstraint),
    Reduce(ReduceConstraint),
    ReducePack(ReducePackConstraint),
    Equality(EqualityConstraint),
    Simplify(SimplifyConstraint),
    PushFunctionType(PushFunctionTypeConstraint),
    PushType(PushTypeConstraint),
    TypeInstantiation(TypeInstantiationConstraint),
}

impl ConstraintV {
    /// C++ `v.index()` — the member's position in the Variant<...> list.
    pub fn index(&self) -> i32 {
        match self {
            ConstraintV::Subtype(_) => 0,
            ConstraintV::PackSubtype(_) => 1,
            ConstraintV::Generalization(_) => 2,
            ConstraintV::Iterable(_) => 3,
            ConstraintV::Name(_) => 4,
            ConstraintV::TypeAliasExpansion(_) => 5,
            ConstraintV::FunctionCall(_) => 6,
            ConstraintV::FunctionCheck(_) => 7,
            ConstraintV::PrimitiveType(_) => 8,
            ConstraintV::HasProp(_) => 9,
            ConstraintV::HasIndexer(_) => 10,
            ConstraintV::AssignProp(_) => 11,
            ConstraintV::AssignIndex(_) => 12,
            ConstraintV::Unpack(_) => 13,
            ConstraintV::Reduce(_) => 14,
            ConstraintV::ReducePack(_) => 15,
            ConstraintV::Equality(_) => 16,
            ConstraintV::Simplify(_) => 17,
            ConstraintV::PushFunctionType(_) => 18,
            ConstraintV::PushType(_) => 19,
            ConstraintV::TypeInstantiation(_) => 20,
        }
    }
}

/// `get_if<T>(&v)` — the Rust shape of C++ overload-on-T over this variant.
pub trait ConstraintVMember: Sized {
    fn get_if(v: &ConstraintV) -> Option<&Self>;
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self>;
}

impl ConstraintVMember for SubtypeConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::Subtype(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::Subtype(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for PackSubtypeConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::PackSubtype(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::PackSubtype(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for GeneralizationConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::Generalization(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::Generalization(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for IterableConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::Iterable(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::Iterable(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for NameConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::Name(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::Name(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for TypeAliasExpansionConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::TypeAliasExpansion(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::TypeAliasExpansion(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for FunctionCallConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::FunctionCall(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::FunctionCall(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for FunctionCheckConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::FunctionCheck(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::FunctionCheck(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for PrimitiveTypeConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::PrimitiveType(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::PrimitiveType(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for HasPropConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::HasProp(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::HasProp(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for HasIndexerConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::HasIndexer(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::HasIndexer(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for AssignPropConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::AssignProp(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::AssignProp(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for AssignIndexConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::AssignIndex(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::AssignIndex(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for UnpackConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::Unpack(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::Unpack(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for ReduceConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::Reduce(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::Reduce(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for ReducePackConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::ReducePack(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::ReducePack(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for EqualityConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::Equality(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::Equality(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for SimplifyConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::Simplify(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::Simplify(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for PushFunctionTypeConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::PushFunctionType(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::PushFunctionType(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for PushTypeConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::PushType(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::PushType(x) => Some(x),
            _ => None,
        }
    }
}

impl ConstraintVMember for TypeInstantiationConstraint {
    fn get_if(v: &ConstraintV) -> Option<&Self> {
        match v {
            ConstraintV::TypeInstantiation(x) => Some(x),
            _ => None,
        }
    }
    fn get_if_mut(v: &mut ConstraintV) -> Option<&mut Self> {
        match v {
            ConstraintV::TypeInstantiation(x) => Some(x),
            _ => None,
        }
    }
}
