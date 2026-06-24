use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::ir_function::IrFunction;
use crate::records::ir_spill_x_64::IrSpillX64;
use crate::records::register_x_64::RegisterX64;
use crate::type_aliases::exit_sync_args_x_64::ExitSyncArgsX64;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrRegAllocX64 {
    pub build: *mut AssemblyBuilderX64,
    pub function: *mut IrFunction,
    pub stats: *mut crate::records::lowering_stats::LoweringStats,

    pub curr_inst_idx: u32,

    pub free_gpr_map: [bool; 16],
    pub gpr_inst_users: [u32; 16],
    pub free_xmm_map: [bool; 16],
    pub xmm_inst_users: [u32; 16],
    pub usable_xmm_reg_count: u8,

    pub used_spill_slot_halfs: [u64; 8], // std::bitset<512> mapped to fixed-size array
    pub max_used_slot: u32,

    pub next_spill_id: u32,
    pub spills: alloc::vec::Vec<IrSpillX64>,

    pub exit_sync_args: DenseHashMap<u32, ExitSyncArgsX64>,

    pub alloc_action_count: u32,
}

impl IrRegAllocX64 {
    /// `static const RegisterX64 kGprAllocOrder[] = {rax, rdx, rcx, rbx, rsi, rdi, r8, r9, r10, r11};`
    /// (IrRegAllocX64.cpp:23)
    pub const K_GPR_ALLOC_ORDER: [crate::records::register_x_64::RegisterX64; 10] = [
        crate::records::register_x_64::RegisterX64::rax,
        crate::records::register_x_64::RegisterX64::rdx,
        crate::records::register_x_64::RegisterX64::rcx,
        crate::records::register_x_64::RegisterX64::rbx,
        crate::records::register_x_64::RegisterX64::rsi,
        crate::records::register_x_64::RegisterX64::rdi,
        crate::records::register_x_64::RegisterX64::r8,
        crate::records::register_x_64::RegisterX64::r9,
        crate::records::register_x_64::RegisterX64::r10,
        crate::records::register_x_64::RegisterX64::r11,
    ];
}
