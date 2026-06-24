use crate::type_aliases::l_value::LValue;
use crate::type_aliases::refinement_map::RefinementMap;
use crate::type_aliases::type_id::TypeId;

pub fn add_refinement(refis: &mut RefinementMap, lvalue: &LValue, ty: TypeId) {
    refis.insert(lvalue.clone(), ty);
}
