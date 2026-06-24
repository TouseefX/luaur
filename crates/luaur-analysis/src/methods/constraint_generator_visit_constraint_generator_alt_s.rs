use crate::enums::control_flow::ControlFlow;
use crate::functions::add_all_as_dependencies_and_chain_returns::add_all_as_dependencies_and_chain_returns;
use crate::functions::checkpoint::checkpoint;
use crate::functions::follow_type::follow_type_id;
use crate::functions::for_each_constraint::for_each_constraint;
use crate::functions::get_mutable_type::getMutable;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::propagate_deprecated_attribute_to_constraint::propagate_deprecated_attribute_to_constraint;
use crate::records::blocked_type::BlockedType;
use crate::records::class_decl_record::ClassDeclRecord;
use crate::records::constraint::Constraint;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::extern_type::ExternType;
use crate::records::generalization_constraint::GeneralizationConstraint;
use crate::records::property_type::Property;
use crate::records::scope::Scope;
use crate::records::table_type::TableType;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::CStr;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_stat_class::AstStatClass;
use luaur_common::FFlag;
use luaur_common::LUAU_ASSERT;

fn ast_name_to_string(name: AstName) -> String {
    if name.value.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(name.value).to_string_lossy().into_owned() }
    }
}

impl ConstraintGenerator {
    pub fn visit_scope_ptr_ast_stat_class(
        &mut self,
        scope: &ScopePtr,
        stat_class: *mut AstStatClass,
    ) -> ControlFlow {
        let stat_class_ref = unsafe { &*stat_class };
        LUAU_ASSERT!(FFlag::DebugLuauUserDefinedClasses.get());

        let class_decl_record = self.class_decl_records.find(&stat_class_ref.name).copied();
        let Some(mut class_decl_record) = class_decl_record else {
            // C++: this is unpopulated in fragment autocomplete.
            return ControlFlow::None;
        };

        let mut method_names: Vec<AstName> = Vec::new();

        for i in 0..stat_class_ref.members.size as usize {
            let member = unsafe { &*stat_class_ref.members.data.add(i) };
            let Some(method) = member.get_if_1() else {
                continue;
            };

            if method_names.contains(&method.function_name) {
                continue;
            }
            method_names.push(method.function_name);

            let method_name = ast_name_to_string(method.function_name);
            let function_type = unsafe {
                let class_ty = follow_type_id(class_decl_record.ty);
                let class_ = get_type_id::<ExternType>(class_ty);
                LUAU_ASSERT!(!class_.is_null());
                LUAU_ASSERT!((*class_).metatable.is_some());

                let metatable_ty = follow_type_id((*class_).metatable.unwrap());
                let metatable = get_type_id::<TableType>(metatable_ty);
                LUAU_ASSERT!(!metatable.is_null());

                let maybe_function_prop: Property =
                    if let Some(instance_prop) = (*class_).props.get(&method_name) {
                        instance_prop.clone()
                    } else if let Some(meta_instance_prop) = (*metatable).props.get(&method_name) {
                        meta_instance_prop.clone()
                    } else {
                        LUAU_ASSERT!(false);
                        Property::default()
                    };

                LUAU_ASSERT!(maybe_function_prop.read_ty.is_some());
                LUAU_ASSERT!(maybe_function_prop.write_ty.is_none());
                maybe_function_prop.read_ty.unwrap()
            };

            let sig = self.check_function_signature(
                scope,
                &mut class_decl_record as *mut ClassDeclRecord,
                method.function,
                None,
                Some(unsafe { (*method.function).base.base.location }),
            );

            let start = checkpoint(self as *const _);
            self.check_function_body(&sig.body_scope, unsafe { &*method.function });
            let end = checkpoint(self as *const _);

            let constraint_scope: &ScopePtr = &sig.signature_scope;
            let c: *mut Constraint = self.add_constraint_scope_ptr_location_constraint_v(
                constraint_scope,
                unsafe { (*method.function).base.base.location },
                ConstraintV::Generalization(GeneralizationConstraint {
                    generalized_type: function_type,
                    source_type: sig.signature,
                    interior_types: Vec::new(),
                    has_deprecated_attribute: false,
                    deprecated_info: Default::default(),
                    no_generics: false,
                }),
            );

            propagate_deprecated_attribute_to_constraint(unsafe { &mut (*c).c }, method.function);

            if FFlag::LuauConstraintGraph.get() {
                add_all_as_dependencies_and_chain_returns(start, end, self, c);
            } else {
                let mut previous: *mut Constraint = core::ptr::null_mut();
                for_each_constraint(start, end, self, |constraint: *mut Constraint| {
                    unsafe { (*c).deprecated_dependencies.push(constraint) };
                    if let ConstraintV::PackSubtype(psc) = unsafe { &(*constraint).c } {
                        if psc.returns {
                            if !previous.is_null() {
                                unsafe { (*constraint).deprecated_dependencies.push(previous) };
                            }
                            previous = constraint;
                        }
                    }
                });
            }

            unsafe {
                let blocked = getMutable::<BlockedType>(function_type);
                LUAU_ASSERT!(!blocked.is_null());
                (*blocked).set_owner(c as *const _);
            }
        }

        ControlFlow::None
    }
}
