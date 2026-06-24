use crate::records::block_and_ordering::BlockAndOrdering;
use crate::records::block_ordering::BlockOrdering;
use crate::records::idf_visit_marks::IdfVisitMarks;
use alloc::collections::BinaryHeap;
use alloc::vec::Vec;

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct IdfContext {
    pub queue: BinaryHeap<BlockAndOrdering>,
    pub worklist: Vec<u32>,
    pub visits: Vec<IdfVisitMarks>,
    pub idf: Vec<u32>,
}

impl IdfContext {
    #[allow(dead_code)]
    fn block_and_ordering_field_shape(block_idx: u32, ordering: BlockOrdering) -> BlockAndOrdering {
        BlockAndOrdering {
            block_idx: block_idx,
            ordering: ordering,
        }
    }

    #[allow(dead_code)]
    fn idf_visit_marks_field_shape(seen_in_queue: bool, seen_in_worklist: bool) -> IdfVisitMarks {
        IdfVisitMarks {
            seen_in_queue: seen_in_queue,
            seen_in_worklist: seen_in_worklist,
        }
    }
}
