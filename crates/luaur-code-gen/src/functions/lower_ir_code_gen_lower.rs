//! @interface-stub
use crate::functions::lower_impl::lower_impl_x_64;
use crate::functions::optimize_memory_operands_x_64_optimize_final_x_64_alt_b::optimize_memory_operands_x_64;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::assembly_options::AssemblyOptions;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_lowering_x_64::IrLoweringX64;
use crate::records::lowering_stats::LoweringStats;
use crate::records::module_helpers::ModuleHelpers;
use luaur_vm::records::proto::Proto;

pub fn lower_ir_x_64_assembly_builder_x_64_ir_builder_vector_u32_module_helpers_proto_assembly_options_lowering_stats(
    build: &mut AssemblyBuilderX64,
    ir: &mut IrBuilder,
    sorted_blocks: &[u32],
    helpers: &mut ModuleHelpers,
    proto: *mut Proto,
    options: AssemblyOptions,
    stats: *mut LoweringStats,
) -> bool {
    optimize_memory_operands_x_64(&mut ir.function);

    let mut lowering =
        IrLoweringX64::ir_lowering_x_64_ir_lowering_x_64(build, helpers, &mut ir.function, stats);
    lowering.reset_restore_callback();

    let bytecodeid = unsafe {
        if proto.is_null() {
            0
        } else {
            (*proto).bytecodeid
        }
    };

    unsafe {
        lower_impl_x_64(
            build,
            &mut lowering,
            &mut ir.function,
            sorted_blocks,
            bytecodeid,
            &options,
        )
    }
}
