//! @interface-stub
use crate::records::ir_builder_fixture::IrBuilderFixture;
use alloc::vec::Vec;
use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
use luaur_code_gen::functions::compute_cfg_dominance_tree_children::compute_cfg_dominance_tree_children;
use luaur_code_gen::functions::compute_cfg_immediate_dominators::compute_cfg_immediate_dominators;
use luaur_code_gen::functions::successors::successors;

impl IrBuilderFixture {
    pub fn define_cfg_tree(&mut self, successor_sets: &Vec<Vec<u32>>) {
        for successor_set in successor_sets {
            let block = self.build.block(IrBlockKind::Internal);
            self.build.begin_block(block);

            self.build
                .function
                .cfg
                .successors_offsets
                .push(self.build.function.cfg.successors.len() as u32);
            self.build
                .function
                .cfg
                .successors
                .extend(successor_set.iter().copied());
        }

        let block_count = self.build.function.blocks.len();

        for i in 0..block_count {
            self.build
                .function
                .cfg
                .predecessors_offsets
                .push(self.build.function.cfg.predecessors.len() as u32);

            for k in 0..block_count {
                for succ_idx in successors(&self.build.function.cfg, k as u32) {
                    if succ_idx == i as u32 {
                        self.build.function.cfg.predecessors.push(k as u32);
                    }
                }
            }
        }

        compute_cfg_immediate_dominators(&mut self.build.function);
        compute_cfg_dominance_tree_children(&mut self.build.function);
    }
}
