use crate::records::completion::Completion;
use crate::records::repl_fixture::ReplFixture;
use crate::type_aliases::completion_set::CompletionSet;
use alloc::string::{String, ToString};

pub fn repl_fixture_check_completion(
    fixture: &ReplFixture,
    completions: &CompletionSet,
    prefix: &str,
    expected: &str,
) -> bool {
    let expected_display = if let Some(pos) = expected.find('(') {
        expected[..pos].to_string()
    } else {
        expected.to_string()
    };

    let expected_completion = Completion {
        completion: format!("{}{}", prefix, expected),
        display: expected_display,
    };

    completions.contains(&expected_completion)
}
