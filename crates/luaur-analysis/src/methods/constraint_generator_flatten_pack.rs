use crate::functions::first::first;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::inference::Inference;
use crate::records::inference_pack::InferencePack;
use crate::records::unpack_constraint::UnpackConstraint;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use alloc::vec;
use luaur_ast::records::location::Location;

impl ConstraintGenerator {
    pub fn flatten_pack(
        &mut self,
        scope: &ScopePtr,
        location: Location,
        pack: InferencePack,
    ) -> Inference {
        let tp = pack.tp;
        let refinements = pack.refinements;

        let mut refinement = None;
        if !refinements.is_empty() {
            refinement = Some(refinements[0]);
        }

        if let Some(f) = first(tp, true) {
            return Inference::inference_type_id_refinement_id(
                f,
                refinement.unwrap_or(core::ptr::null_mut()),
            );
        }

        // Create a blocked type: arena->addType(BlockedType{})
        let type_result = unsafe { (*self.arena).add_type(BlockedType::default()) };

        // addConstraint(scope, location, UnpackConstraint{{typeResult}, tp})
        let unpack_constraint = UnpackConstraint {
            result_pack: vec![type_result],
            source_pack: tp,
        };
        let constraint_ptr = self.add_constraint_scope_ptr_location_constraint_v(
            scope,
            location,
            ConstraintV::Unpack(unpack_constraint),
        );

        // getMutable<BlockedType>(typeResult)->setOwner(c)
        unsafe {
            let blocked = get_mutable_type_id::<BlockedType>(type_result);
            (*blocked).set_owner(constraint_ptr as *const _);
        }

        Inference::inference_type_id_refinement_id(
            type_result,
            refinement.unwrap_or(core::ptr::null_mut()),
        )
    }
}
