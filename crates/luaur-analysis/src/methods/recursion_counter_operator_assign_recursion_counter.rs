use crate::records::recursion_counter::RecursionCounter;

impl RecursionCounter {
    /// In C++, this method is deleted to prevent copying.
    /// In Rust, `RecursionCounter` does not implement `Clone` or `Copy`,
    /// so an explicit assignment operator is not provided.
    #[allow(dead_code)]
    fn operator_assign(&mut self, _other: &RecursionCounter) -> &mut RecursionCounter {
        panic!("RecursionCounter is not assignable");
    }
}
