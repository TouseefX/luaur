use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::extern_type::ExternType;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

pub fn generate_documentation_symbols(ty: TypeId, root_name: String) {
    if unsafe { (*ty).persistent } {
        return;
    }

    let mutable_type_ptr = as_mutable_type_id(ty);
    if mutable_type_ptr.is_null() {
        return;
    }

    unsafe {
        (*mutable_type_ptr).documentation_symbol = Some(root_name.clone());
    }

    let table_type_ptr = unsafe { get_mutable_type_id::<TableType>(ty) };
    if !table_type_ptr.is_null() {
        let table_type = unsafe { &mut *table_type_ptr };
        for (name, prop) in &mut table_type.props {
            let mut n = String::with_capacity(root_name.len() + 1 + name.len());
            n.push_str(&root_name);
            n.push('.');
            n.push_str(name);
            prop.documentation_symbol = Some(n);
        }
    } else if unsafe { !get_mutable_type_id::<ExternType>(ty).is_null() } {
        let extern_type_ptr = unsafe { get_mutable_type_id::<ExternType>(ty) };
        let extern_type = unsafe { &mut *extern_type_ptr };
        for (name, prop) in &mut extern_type.props {
            let mut n = String::with_capacity(root_name.len() + 1 + name.len());
            n.push_str(&root_name);
            n.push('.');
            n.push_str(name);
            prop.documentation_symbol = Some(n);
        }
    }
}
