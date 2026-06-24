use crate::records::stack_pusher_type_checker_2::StackPusher;

impl StackPusher {
    #[allow(dead_code)]
    pub fn operator_assign(&mut self, _other: &StackPusher) -> &mut Self {
        panic!("StackPusher copy assignment is deleted in C++")
    }
}
