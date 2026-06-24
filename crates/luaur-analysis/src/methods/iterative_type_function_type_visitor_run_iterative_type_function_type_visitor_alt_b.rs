use crate::records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;

impl IterativeTypeFunctionTypeVisitor {
    pub fn run_type_function_type_pack_id(&mut self, root_tp: TypeFunctionTypePackId) {
        self.parent_cursor = -1;
        self.work_cursor = 0;
        self.work_queue.clear();
        self.traverse_type_function_type_pack_id(root_tp);
        self.process_work_queue();
    }
}
