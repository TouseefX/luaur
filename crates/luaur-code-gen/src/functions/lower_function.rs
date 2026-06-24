//! @interface-stub
use crate::enums::code_gen_compilation_result::CodeGenCompilationResult;
use crate::enums::ir_block_kind::IrBlockKind;
use crate::enums::ir_cmd::IrCmd;
use crate::functions::compute_cfg_block_edges::compute_cfg_block_edges;
use crate::functions::compute_cfg_info::compute_cfg_info;
use crate::functions::const_prop_in_block_chains::const_prop_in_block_chains;
use crate::functions::create_linear_blocks::create_linear_blocks;
use crate::functions::get_instruction_count_code_gen_lower::get_instruction_count_vector_ir_inst_ir_cmd;
use crate::functions::get_sorted_block_order::get_sorted_block_order;
use crate::functions::jit_rng_seed::jit_rng_seed;
use crate::functions::kill_unused_blocks::kill_unused_blocks;
use crate::functions::lower_ir_code_gen_lower::lower_ir_x_64_assembly_builder_x_64_ir_builder_vector_u32_module_helpers_proto_assembly_options_lowering_stats;
use crate::functions::lower_ir_code_gen_lower_alt_b::lower_ir_a_64_assembly_builder_a_64_ir_builder_vector_u32_module_helpers_proto_assembly_options_lowering_stats;
use crate::functions::mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains;
use crate::functions::update_last_use_locations::update_last_use_locations;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::assembly_options::AssemblyOptions;
use crate::records::ir_builder::IrBuilder;
use crate::records::lowering_stats::LoweringStats;
use crate::records::module_helpers::ModuleHelpers;
use luaur_vm::functions::lua_clock::lua_clock;
use luaur_vm::records::proto::Proto;

pub unsafe fn lower_function_x_64(
    ir: &mut IrBuilder,
    build: &mut AssemblyBuilderX64,
    helpers: &mut ModuleHelpers,
    proto: *mut Proto,
    options: AssemblyOptions,
    stats: *mut LoweringStats,
    code_gen_compilation_result: &mut CodeGenCompilationResult,
) -> bool {
    ir.function.stats = stats;
    ir.function.record_counters = options.compilation_options.record_counters;

    if options.compilation_options.nop_padding && !proto.is_null() {
        ir.function.jit_rng_state = jit_rng_seed(proto as usize);
    }

    kill_unused_blocks(&mut ir.function);

    let mut pre_opt_block_count = 0u32;
    let mut max_block_instructions = 0u32;

    for block in &ir.function.blocks {
        if block.kind != IrBlockKind::Dead {
            pre_opt_block_count += 1;
        }

        let block_instructions = block.finish.wrapping_sub(block.start);
        max_block_instructions = max_block_instructions.max(block_instructions);
    }

    if !stats.is_null() {
        (*stats).blocks_pre_opt += pre_opt_block_count;
        (*stats).max_block_instructions = max_block_instructions;
    }

    if pre_opt_block_count >= luaur_common::FInt::CodegenHeuristicsBlockLimit.get() as u32 {
        *code_gen_compilation_result = CodeGenCompilationResult::CodeGenOverflowBlockLimit;
        return false;
    }

    if max_block_instructions
        >= luaur_common::FInt::CodegenHeuristicsBlockInstructionLimit.get() as u32
    {
        *code_gen_compilation_result =
            CodeGenCompilationResult::CodeGenOverflowBlockInstructionLimit;
        return false;
    }

    compute_cfg_info(&mut ir.function);

    const_prop_in_block_chains(ir);

    if !luaur_common::FFlag::DebugCodegenOptSize.get() {
        let mut start_time = 0.0;
        let mut const_prop_instruction_count = 0u32;

        if !stats.is_null() {
            const_prop_instruction_count = get_instruction_count_vector_ir_inst_ir_cmd(
                &ir.function.instructions,
                IrCmd::SUBSTITUTE,
            );
            start_time = lua_clock();
        }

        create_linear_blocks(ir);

        if !stats.is_null() {
            (*stats).block_linearization_stats.time_seconds += lua_clock() - start_time;
            const_prop_instruction_count = get_instruction_count_vector_ir_inst_ir_cmd(
                &ir.function.instructions,
                IrCmd::SUBSTITUTE,
            )
            .wrapping_sub(const_prop_instruction_count);
            (*stats)
                .block_linearization_stats
                .const_prop_instruction_count += const_prop_instruction_count;
        }
    }

    mark_dead_stores_in_block_chains(ir);

    compute_cfg_block_edges(&mut ir.function);

    let sorted_blocks = get_sorted_block_order(&mut ir.function);

    update_last_use_locations(&mut ir.function, &sorted_blocks);

    if !stats.is_null() {
        for block in &ir.function.blocks {
            if block.kind != IrBlockKind::Dead {
                (*stats).blocks_post_opt += 1;
            }
        }
    }

    let result = lower_ir_x_64_assembly_builder_x_64_ir_builder_vector_u32_module_helpers_proto_assembly_options_lowering_stats(
        build,
        ir,
        &sorted_blocks,
        helpers,
        proto,
        options,
        stats,
    );

    if !result {
        *code_gen_compilation_result = CodeGenCompilationResult::CodeGenLoweringFailure;
    }

    result
}

pub unsafe fn lower_function_a_64(
    ir: &mut IrBuilder,
    build: &mut AssemblyBuilderA64,
    helpers: &mut ModuleHelpers,
    proto: *mut Proto,
    options: AssemblyOptions,
    stats: *mut LoweringStats,
    code_gen_compilation_result: &mut CodeGenCompilationResult,
) -> bool {
    ir.function.stats = stats;
    ir.function.record_counters = options.compilation_options.record_counters;

    if options.compilation_options.nop_padding && !proto.is_null() {
        ir.function.jit_rng_state = jit_rng_seed(proto as usize);
    }

    kill_unused_blocks(&mut ir.function);

    let mut pre_opt_block_count = 0u32;
    let mut max_block_instructions = 0u32;

    for block in &ir.function.blocks {
        if block.kind != IrBlockKind::Dead {
            pre_opt_block_count += 1;
        }

        let block_instructions = block.finish.wrapping_sub(block.start);
        max_block_instructions = max_block_instructions.max(block_instructions);
    }

    if !stats.is_null() {
        (*stats).blocks_pre_opt += pre_opt_block_count;
        (*stats).max_block_instructions = max_block_instructions;
    }

    if pre_opt_block_count >= luaur_common::FInt::CodegenHeuristicsBlockLimit.get() as u32 {
        *code_gen_compilation_result = CodeGenCompilationResult::CodeGenOverflowBlockLimit;
        return false;
    }

    if max_block_instructions
        >= luaur_common::FInt::CodegenHeuristicsBlockInstructionLimit.get() as u32
    {
        *code_gen_compilation_result =
            CodeGenCompilationResult::CodeGenOverflowBlockInstructionLimit;
        return false;
    }

    compute_cfg_info(&mut ir.function);

    const_prop_in_block_chains(ir);

    if !luaur_common::FFlag::DebugCodegenOptSize.get() {
        let mut start_time = 0.0;
        let mut const_prop_instruction_count = 0u32;

        if !stats.is_null() {
            const_prop_instruction_count = get_instruction_count_vector_ir_inst_ir_cmd(
                &ir.function.instructions,
                IrCmd::SUBSTITUTE,
            );
            start_time = lua_clock();
        }

        create_linear_blocks(ir);

        if !stats.is_null() {
            (*stats).block_linearization_stats.time_seconds += lua_clock() - start_time;
            const_prop_instruction_count = get_instruction_count_vector_ir_inst_ir_cmd(
                &ir.function.instructions,
                IrCmd::SUBSTITUTE,
            )
            .wrapping_sub(const_prop_instruction_count);
            (*stats)
                .block_linearization_stats
                .const_prop_instruction_count += const_prop_instruction_count;
        }
    }

    mark_dead_stores_in_block_chains(ir);

    compute_cfg_block_edges(&mut ir.function);

    let sorted_blocks = get_sorted_block_order(&mut ir.function);

    update_last_use_locations(&mut ir.function, &sorted_blocks);

    if !stats.is_null() {
        for block in &ir.function.blocks {
            if block.kind != IrBlockKind::Dead {
                (*stats).blocks_post_opt += 1;
            }
        }
    }

    let result = lower_ir_a_64_assembly_builder_a_64_ir_builder_vector_u32_module_helpers_proto_assembly_options_lowering_stats(
        build,
        ir,
        &sorted_blocks,
        helpers,
        proto,
        options,
        stats,
    );

    if !result {
        *code_gen_compilation_result = CodeGenCompilationResult::CodeGenLoweringFailure;
    }

    result
}
