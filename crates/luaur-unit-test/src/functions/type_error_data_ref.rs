use luaur_analysis::records::type_error::TypeError;
use luaur_analysis::type_aliases::type_error_data::TypeErrorDataMember;

pub fn type_error_data_ref<T: TypeErrorDataMember>(error: &TypeError) -> Option<&T> {
    T::get_if(&error.data)
}
