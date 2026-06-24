use luaur_analysis::records::check_result::CheckResult;
use luaur_analysis::records::type_error::TypeError;
use luaur_analysis::type_aliases::type_error_data::TypeErrorDataMember;

pub fn has_error<T: TypeErrorDataMember>(result: &CheckResult) -> bool {
    result.errors.iter().any(|error| get::<T>(error).is_some())
}

fn get<T: TypeErrorDataMember>(error: &TypeError) -> Option<&T> {
    T::get_if(&error.data)
}
