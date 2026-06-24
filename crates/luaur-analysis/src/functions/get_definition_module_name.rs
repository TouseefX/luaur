use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::table_type::TableType;
use crate::type_aliases::module_name_type::ModuleName;
use crate::type_aliases::type_id::TypeId;

pub fn get_definition_module_name(type_: TypeId) -> Option<ModuleName> {
    let type_ = unsafe { follow_type_id(type_) };

    let ttv = unsafe { get_type_id::<TableType>(type_) };
    if !ttv.is_null() {
        let def_mod_name = unsafe { (*ttv).definition_module_name.clone() };
        if !def_mod_name.is_empty() {
            return Some(def_mod_name);
        }
    }

    let ftv = unsafe { get_type_id::<FunctionType>(type_) };
    if !ftv.is_null() {
        if let Some(def) = unsafe { (*ftv).definition.as_ref() } {
            if let Some(module_name) = def.definition_module_name.clone() {
                return Some(module_name);
            }
        }
    }

    let etv = unsafe { get_type_id::<ExternType>(type_) };
    if !etv.is_null() {
        let def_mod_name = unsafe { (*etv).definition_module_name.clone() };
        if !def_mod_name.is_empty() {
            return Some(def_mod_name);
        }
    }

    None
}
