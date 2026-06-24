use crate::records::error_snapshot::ErrorSnapshot;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TypeCheckLog {
    pub errors: alloc::vec::Vec<ErrorSnapshot>,
}
