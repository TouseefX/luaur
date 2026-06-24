use crate::records::instantiation_queuer_deprecated::InstantiationQueuerDeprecated;
use crate::records::reduce_constraint::ReduceConstraint;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::type_id::TypeId;

impl InstantiationQueuerDeprecated {
    pub fn visit_type_id_type_function_instance_type(
        &mut self,
        ty: TypeId,
        _tfit: &TypeFunctionInstanceType,
    ) -> bool {
        let solver = unsafe { &mut *self.solver };
        solver.push_constraint(
            self.scope,
            self.location,
            ConstraintV::Reduce(ReduceConstraint { ty }),
        );
        true
    }
}
