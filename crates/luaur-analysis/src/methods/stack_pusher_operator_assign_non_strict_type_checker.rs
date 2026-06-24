use crate::records::stack_pusher_non_strict_type_checker::StackPusher;

impl StackPusher {
    pub fn operator_assign_mut(&mut self, _other: &StackPusher) -> &mut Self {
        unimplemented!("StackPusher copy assignment is deleted in C++")
    }
}
