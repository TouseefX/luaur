use crate::records::refinement_arena_control_flow_graph::RefinementArena;

pub fn refinement_arena_freeze(arena: &mut RefinementArena) {
    arena.allocator.freeze();
}
