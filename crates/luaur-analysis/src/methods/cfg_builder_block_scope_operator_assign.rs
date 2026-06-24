use crate::records::block_scope::BlockScope;

impl BlockScope {
    /// In C++, this method is deleted to prevent copying.
    /// In Rust, `BlockScope` does not implement `Clone` or `Copy`,
    /// so an explicit assignment operator is not provided.
    #[allow(dead_code)]
    fn operator_assign(&mut self, _other: &BlockScope) -> &mut BlockScope {
        panic!("BlockScope is not assignable");
    }
}
