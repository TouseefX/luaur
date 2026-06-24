use crate::enums::ir_const_kind::IrConstKind;

#[derive(Debug, Clone, Copy, Hash)]
#[repr(C)]
pub struct ConstantKey {
    pub(crate) kind: IrConstKind,
    pub(crate) value: u64,
}

impl PartialEq for ConstantKey {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.value == other.value
    }
}

impl Eq for ConstantKey {}

impl Default for ConstantKey {
    fn default() -> Self {
        Self {
            kind: IrConstKind::Int,
            value: 0,
        }
    }
}
