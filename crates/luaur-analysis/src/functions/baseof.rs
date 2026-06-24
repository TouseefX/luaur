use crate::records::field::Field;
use crate::records::symbol::Symbol;
use crate::type_aliases::l_value::{LValue, LValueMember};
use alloc::sync::Arc;

pub fn baseof(lvalue: &LValue) -> *const LValue {
    if let Some(field) = <Field as LValueMember>::get_if(lvalue) {
        return match &field.parent {
            Some(parent) => Arc::as_ptr(parent),
            None => core::ptr::null(),
        };
    }

    let symbol = <Symbol as LValueMember>::get_if(lvalue);
    debug_assert!(symbol.is_some());
    core::ptr::null() // Base of root is null.
}
