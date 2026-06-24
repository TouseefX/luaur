use crate::enums::ir_block_kind::IrBlockKind;
use crate::enums::ir_op_kind::IrOpKind;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_block::IrBlock;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;

impl IrBuilder {
    pub fn fallback_block(&mut self, pcpos: u32) -> IrOp {
        let index = self.function.blocks.len() as u32;
        self.function.blocks.push(IrBlock {
            kind: IrBlockKind::Fallback,
            flags: 0,
            use_count: 0,
            start: !0u32,
            finish: !0u32,
            sortkey: 0,
            chainkey: 0,
            expected_next_block: !0u32,
            startpc: pcpos,
            label: crate::records::label::Label::default(),
        });
        CODEGEN_ASSERT!(index != 0);
        IrOp::ir_op_ir_op_kind_u32(IrOpKind::Block, index)
    }
}
