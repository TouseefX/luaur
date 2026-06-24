use crate::records::block_and_ordering::BlockAndOrdering;

impl BlockAndOrdering {
    #[inline]
    pub fn idf_context_block_and_ordering_operator_lt(&self, rhs: &BlockAndOrdering) -> bool {
        if self.ordering.depth != rhs.ordering.depth {
            return self.ordering.depth < rhs.ordering.depth;
        }

        self.ordering.preOrder < rhs.ordering.preOrder
    }
}

impl PartialOrd for BlockAndOrdering {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BlockAndOrdering {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if self.ordering.depth != other.ordering.depth {
            self.ordering.depth.cmp(&other.ordering.depth)
        } else {
            self.ordering.preOrder.cmp(&other.ordering.preOrder)
        }
    }
}
