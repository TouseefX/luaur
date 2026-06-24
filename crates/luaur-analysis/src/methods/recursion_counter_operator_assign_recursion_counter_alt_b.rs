use crate::records::recursion_counter::RecursionCounter;

impl RecursionCounter {
    /// @delete
    #[allow(dead_code)]
    pub fn operator_assign_mut(&mut self, _other: RecursionCounter) -> &mut RecursionCounter {
        unimplemented!("RecursionCounter move assignment is deleted in C++")
    }
}
