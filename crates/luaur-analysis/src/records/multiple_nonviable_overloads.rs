#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MultipleNonviableOverloads {
    pub(crate) attempted_arg_count: usize,
}
