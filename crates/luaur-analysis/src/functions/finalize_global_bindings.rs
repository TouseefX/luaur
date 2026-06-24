use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::persist_type::persist;
use crate::functions::to_string_symbol::to_string_symbol;
use crate::records::table_type::TableType;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use alloc::format;

pub fn finalize_global_bindings(scope: ScopePtr) {
    for (symbol, binding) in scope.bindings.iter() {
        persist(binding.type_id);

        let ttv = unsafe { get_mutable_type_id::<TableType>(binding.type_id) };
        if !ttv.is_null() {
            if unsafe { (*ttv).name.is_none() } {
                let name = format!("typeof({})", unsafe { to_string_symbol(symbol) });
                unsafe {
                    (*ttv).name = Some(name);
                }
            }
        }
    }
}
