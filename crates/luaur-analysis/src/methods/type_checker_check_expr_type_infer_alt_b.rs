use crate::functions::allows_no_return_values::allows_no_return_values;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeChecker2 {
    pub fn allows_no_return_values(&self, tp: TypePackId) -> bool {
        allows_no_return_values(tp)
    }
}
