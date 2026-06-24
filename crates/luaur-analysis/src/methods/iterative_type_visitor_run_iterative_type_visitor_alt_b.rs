use crate::records::iterative_type_visitor::IterativeTypeVisitor;

impl IterativeTypeVisitor {
    pub fn iterative_type_visitor_run(&mut self) {
        self.parent_cursor = -1;
        self.work_cursor = 0;
        self.work_queue.clear();
        self.iterative_type_visitor_traverse();
        self.iterative_type_visitor_process_work_queue();
    }
}
