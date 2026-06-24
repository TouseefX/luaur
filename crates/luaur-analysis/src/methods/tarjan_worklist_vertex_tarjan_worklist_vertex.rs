use crate::records::tarjan_worklist_vertex::TarjanWorklistVertex;

impl TarjanWorklistVertex {
    pub fn tarjan_worklist_vertex_tarjan_worklist_vertex(
        &mut self,
        index: i32,
        curr_edge: i32,
        last_edge: i32,
    ) {
        self.index = index;
        self.curr_edge = curr_edge;
        self.last_edge = last_edge;
    }
}
