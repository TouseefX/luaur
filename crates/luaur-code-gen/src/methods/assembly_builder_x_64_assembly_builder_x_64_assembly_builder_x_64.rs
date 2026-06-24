use crate::enums::abix_64::ABIX64;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use alloc::string::String;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl AssemblyBuilderX64 {
    pub fn assembly_builder_x_64_bool_abix_64_i32(
        log_text: bool,
        abi: ABIX64,
        features: u32,
    ) -> Self {
        let mut builder = Self {
            data: Vec::new(),
            code: Vec::new(),
            text: String::new(),
            log_text,
            abi,
            features,
            next_label: 1,
            pending_labels: Vec::new(),
            label_locations: Vec::new(),
            const_cache_32: DenseHashMap::new(!0u32),
            const_cache_64: DenseHashMap::new(!0u64),
            finalized: false,
            data_pos: 0,
            code_pos: core::ptr::null_mut(),
            code_end: core::ptr::null_mut(),
            instruction_count: 0,
        };

        builder.data.resize(4096, 0);
        builder.data_pos = builder.data.len();

        builder.code.resize(4096, 0);
        builder.code_pos = builder.code.as_mut_ptr();
        builder.code_end = unsafe { builder.code_pos.add(builder.code.len()) };

        builder
    }
}
