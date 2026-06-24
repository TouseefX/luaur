use crate::records::occurs_check_failed::OccursCheckFailed;
use crate::records::type_pack_var::TypePackVar;
use crate::records::unifier::Unifier;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;

impl Unifier {
    pub fn occurs_check_type_pack_id_type_pack_id_bool(
        &mut self,
        needle: TypePackId,
        haystack: TypePackId,
        _reversed: bool,
    ) -> bool {
        let shared_state = unsafe { &mut *self.shared_state };
        shared_state.temp_seen_tp.clear();

        let occurs = self.occurs_check_dense_hash_set_type_pack_id_type_pack_id_type_pack_id(
            &mut shared_state.temp_seen_tp,
            needle,
            haystack,
        );

        if occurs {
            self.report_error_location_type_error_data(
                self.location,
                TypeErrorData::OccursCheckFailed(OccursCheckFailed::default()),
            );
            // C++: log.replace(needle, BoundTypePack{builtinTypes->errorTypePack});
            // The Bound variant stores the bound-to pack id directly.
            let error_tp = unsafe { (*self.builtin_types).errorTypePack };
            let bound = TypePackVar {
                ty: TypePackVariant::Bound(error_tp),
                persistent: false,
                owningArena: core::ptr::null_mut(),
            };
            self.log.replace_type_pack_id_type_pack_var(needle, bound);
        }

        occurs
    }
}
