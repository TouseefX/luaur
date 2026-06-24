use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::optional_value_access::OptionalValueAccess;
use crate::records::type_checker::TypeChecker;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn strip_from_nil_and_report(&mut self, ty: TypeId, location: &Location) -> TypeId {
        let ty = unsafe { follow_type_id(ty) };

        if let Some(utv) = unsafe { get_type_id::<UnionType>(ty).as_ref() } {
            if !utv
                .options
                .iter()
                .any(|&t| unsafe { crate::functions::is_nil::is_nil(t) })
            {
                return ty;
            }
        }

        if let Some(stripped_union) = self.try_strip_union_from_nil(ty) {
            self.report_error_location_type_error_data(
                location,
                crate::type_aliases::type_error_data::TypeErrorData::OptionalValueAccess(
                    OptionalValueAccess { optional: ty },
                ),
            );
            return unsafe { follow_type_id(stripped_union) };
        }

        ty
    }
}
