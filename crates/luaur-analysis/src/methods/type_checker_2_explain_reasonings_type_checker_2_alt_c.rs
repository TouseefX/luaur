use crate::records::reasonings::Reasonings;
use crate::records::subtyping_result::SubtypingResult;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::location::Location;

impl TypeChecker2 {
    // C++ `Reasonings TypeChecker2::explainReasonings(TypePackId, TypePackId,
    // Location, const SubtypingResult&)` — forwards to the templated
    // `explainReasonings_`.
    pub fn explain_reasonings_type_pack_id_type_pack_id_location_subtyping_result(
        &mut self,
        sub_tp: TypePackId,
        super_tp: TypePackId,
        location: Location,
        r: &SubtypingResult,
    ) -> Reasonings {
        self.explain_reasonings_generic(sub_tp, super_tp, location, r)
    }
}
