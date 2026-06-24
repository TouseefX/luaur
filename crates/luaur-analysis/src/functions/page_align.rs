pub(crate) fn page_align(size: usize) -> usize {
    const K_PAGE_SIZE: usize = 4096;
    (size + K_PAGE_SIZE - 1) & !(K_PAGE_SIZE - 1)
}
