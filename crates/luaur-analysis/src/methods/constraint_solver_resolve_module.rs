use crate::enums::type_file_resolver::Type;
use crate::functions::first::first;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::illegal_require::IllegalRequire;
use crate::records::module_info::ModuleInfo;
use crate::records::unknown_require::UnknownRequire;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use alloc::string::{String, ToString};
use luaur_ast::records::location::Location;

impl ConstraintSolver {
    pub fn resolve_module(&mut self, info: &ModuleInfo, location: &Location) -> TypeId {
        if info.name.is_empty() {
            self.report_error_type_error_data_location(
                UnknownRequire::new(String::new()).into(),
                location,
            );
            return unsafe { (*self.builtin_types).errorType };
        }

        for require_cycle in &self.require_cycles {
            if !require_cycle.path.is_empty() && require_cycle.path[0] == info.name {
                return unsafe { (*self.builtin_types).anyType };
            }
        }

        let module = unsafe {
            ((*self.module_resolver).vtable.get_module)(self.module_resolver, &info.name)
        };
        if module.is_none() {
            if unsafe {
                !((*self.module_resolver).vtable.module_exists)(self.module_resolver, &info.name)
            } && !info.optional
            {
                let human_readable_name = unsafe {
                    ((*self.module_resolver)
                        .vtable
                        .get_human_readable_module_name)(
                        self.module_resolver, &info.name
                    )
                };
                self.report_error_type_error_data_location(
                    UnknownRequire::new(human_readable_name).into(),
                    location,
                );
            }
            return unsafe { (*self.builtin_types).errorType };
        }

        let module = module.unwrap();
        if module.r#type != Type::Module {
            self.report_error_type_error_data_location(
                IllegalRequire::new(
                    module.human_readable_name.clone(),
                    "Module is not a ModuleScript. It cannot be required.".to_string(),
                )
                .into(),
                location,
            );
            return unsafe { (*self.builtin_types).errorType };
        }

        if module
            .errors
            .iter()
            .any(|error| matches!(error.data, TypeErrorData::SyntaxError(_)))
        {
            return unsafe { (*self.builtin_types).errorType };
        }

        let module_pack = module.return_type;
        if !unsafe { get_type_pack_id::<ErrorTypePack>(module_pack) }.is_null() {
            return unsafe { (*self.builtin_types).errorType };
        }

        let module_type = first(module_pack, true);
        if module_type.is_none() {
            self.report_error_type_error_data_location(
                IllegalRequire::new(
                    module.human_readable_name.clone(),
                    "Module does not return exactly 1 value. It cannot be required.".to_string(),
                )
                .into(),
                location,
            );
            return unsafe { (*self.builtin_types).errorType };
        }

        module_type.unwrap()
    }
}
