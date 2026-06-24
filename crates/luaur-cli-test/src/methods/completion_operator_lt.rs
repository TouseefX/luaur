use crate::records::completion::Completion;

pub fn completion_operator_lt(a: &Completion, b: &Completion) -> bool {
    a.completion < b.completion || (a.completion == b.completion && a.display < b.display)
}
