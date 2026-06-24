use crate::records::instantiation_queuer::InstantiationQueuer;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::type_alias_expansion_constraint::TypeAliasExpansionConstraint;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::type_id::TypeId;

impl InstantiationQueuer {
    pub fn visit_type_id_pending_expansion_type(
        &mut self,
        ty: TypeId,
        _petv: &PendingExpansionType,
    ) -> bool {
        let solver = unsafe { &mut *self.solver };
        solver.push_constraint(
            self.scope,
            self.location,
            ConstraintV::TypeAliasExpansion(TypeAliasExpansionConstraint { target: ty }),
        );
        false
    }
}
