use crate::enums::subtyping_variance::SubtypingVariance;
use crate::type_aliases::type_id::TypeId;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubtypeConstraintRecord {
    pub(crate) subTy: TypeId,
    pub(crate) superTy: TypeId,
    pub(crate) variance: SubtypingVariance,
}
