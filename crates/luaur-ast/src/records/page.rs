// C++ `alignas(8)` on `data`; Rust has no field-level alignment, but aligning
// the whole struct to 8 puts `data` (after the 8-byte `next` pointer) at an
// 8-aligned offset, preserving the intent.
#[repr(C, align(8))]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct Page {
    pub(crate) next: *mut Page,
    pub(crate) data: [u8; 8192],
}
