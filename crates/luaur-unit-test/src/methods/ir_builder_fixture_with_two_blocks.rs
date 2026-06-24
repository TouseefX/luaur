use crate::records::ir_builder_fixture::IrBuilderFixture;
use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
use luaur_code_gen::enums::ir_cmd::IrCmd;
use luaur_code_gen::records::ir_builder::IrBuilder;
use luaur_code_gen::records::ir_op::IrOp;

impl IrBuilderFixture {
    /// Port of C++ `IrBuilderFixture::withTwoBlocks`. Creates `main`, `a`, `b`;
    /// runs the caller's builder closure in `main`, then emits `RETURN 1u` in `a`
    /// and `RETURN 2u` in `b`. The closure receives `&mut IrBuilder` plus the two
    /// branch target block ops `a` and `b`.
    pub fn with_two_blocks<F>(&mut self, f: F)
    where
        F: FnOnce(&mut IrBuilder, IrOp, IrOp),
    {
        let b = &mut self.build;
        let main = b.block(IrBlockKind::Internal);
        let a = b.block(IrBlockKind::Internal);
        let bb = b.block(IrBlockKind::Internal);

        b.begin_block(main);
        f(b, a, bb);

        b.begin_block(a);
        let c1 = b.const_uint(1);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1);

        b.begin_block(bb);
        let c2 = b.const_uint(2);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c2);
    }
}
