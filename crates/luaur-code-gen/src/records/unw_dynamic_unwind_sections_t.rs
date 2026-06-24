#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct unw_dynamic_unwind_sections_t {
    pub dso_base: usize,
    pub dwarf_section: usize,
    pub dwarf_section_length: usize,
    pub compact_unwind_section: usize,
    pub compact_unwind_section_length: usize,
}

#[allow(non_upper_case_globals)]
impl unw_dynamic_unwind_sections_t {
    pub const dso_base: usize = 0;
    pub const dwarf_section: usize = 0;
    pub const dwarf_section_length: usize = 0;
    pub const compact_unwind_section: usize = 0;
    pub const compact_unwind_section_length: usize = 0;
}

pub type UnwDynamicUnwindSectionsT = unw_dynamic_unwind_sections_t;
