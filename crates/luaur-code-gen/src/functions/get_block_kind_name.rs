use crate::enums::ir_block_kind::IrBlockKind;
use luaur_common::macros::luau_unreachable::LUAU_UNREACHABLE;

pub fn get_block_kind_name(kind: IrBlockKind) -> &'static str {
    match kind {
        IrBlockKind::Bytecode => "bb_bytecode",
        IrBlockKind::Fallback => "bb_fallback",
        IrBlockKind::Internal => "bb",
        IrBlockKind::Linearized => "bb_linear",
        IrBlockKind::ExitSync => "bb_exit",
        IrBlockKind::Dead => "dead",
        _ => LUAU_UNREACHABLE!(),
    }
}
