use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::ir_function::IrFunction;
use crate::records::lowering_stats::LoweringStats;
use crate::records::register_a_64::RegisterA64;
use crate::records::set::Set;
use crate::records::spill::Spill;
use crate::type_aliases::exit_sync_args_a_64::ExitSyncArgsA64;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrRegAllocA64 {
    pub build: *mut AssemblyBuilderA64,
    pub function: *mut IrFunction,
    pub stats: *mut LoweringStats,

    pub curr_inst_idx: u32,

    pub gpr: Set,
    pub simd: Set,

    pub spills: alloc::vec::Vec<Spill>,

    pub free_spill_slots: u64,

    pub exit_sync_args: DenseHashMap<u32, ExitSyncArgsA64>,

    pub alloc_action_count: u32,

    pub error: bool,
}

impl IrRegAllocA64 {
    pub(crate) const kInvalidInstIdx: u32 = 0xFFFFFFFF;

    #[allow(dead_code)]
    fn set_field_shape(base: u32, free: u32, temp: u32, defs: [u32; 32]) -> Set {
        Set {
            base: base,
            free: free,
            temp: temp,
            defs: defs,
        }
    }

    #[allow(dead_code)]
    fn spill_field_shape(inst: u32, origin: RegisterA64, slot: i8) -> Spill {
        Spill {
            inst: inst,
            origin: origin,
            slot: slot,
        }
    }
}
