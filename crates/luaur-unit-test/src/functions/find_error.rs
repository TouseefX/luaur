use alloc::vec::Vec;
use luaur_analysis::records::check_result::CheckResult;
use luaur_analysis::records::type_error::TypeError;
use luaur_analysis::type_aliases::type_error_data::TypeErrorDataMember;

pub fn find_error<E: Clone + TypeErrorDataMember>(result: &CheckResult) -> Option<E> {
    for error in &result.errors {
        if let Some(e) = get::<E>(error) {
            return Some(e.clone());
        }
    }
    None
}

fn get<T: Clone + TypeErrorDataMember>(error: &TypeError) -> Option<T> {
    T::get_if(&error.data).cloned()
}
