use crate::records::ir_builder_fixture::IrBuilderFixture;
use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
use luaur_code_gen::enums::ir_cmd::IrCmd;
use luaur_code_gen::records::ir_builder::IrBuilder;
use luaur_code_gen::records::ir_op::IrOp;

impl IrBuilderFixture {
    /// Port of C++ `IrBuilderFixture::withOneBlock`. Creates a `main` block and a
    /// follow-up block `a`; runs the caller's builder closure in `main`, then
    /// emits `RETURN 1u` in `a`. The closure receives `&mut IrBuilder` (the C++
    /// closure captures `build`) plus the block op `a`.
    pub fn with_one_block<F>(&mut self, f: F)
    where
        F: FnOnce(&mut IrBuilder, IrOp),
    {
        let b = &mut self.build;
        let main = b.block(IrBlockKind::Internal);
        let a = b.block(IrBlockKind::Internal);

        b.begin_block(main);
        f(b, a);

        b.begin_block(a);
        let c1 = b.const_uint(1);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, c1);
    }
}
