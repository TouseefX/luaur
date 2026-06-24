use crate::functions::follow_type::follow_type_id;
use crate::functions::to_string_to_string_alt_c::to_string_type_id;
use crate::records::constraint_solving_incomplete_error::ConstraintSolvingIncompleteError;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_id::TypeId;
use alloc::format;
use core::ffi::CStr;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::rtti::ast_node_as;
use luaur_common::FFlag;

impl TypeChecker2 {
    pub fn lookup_annotation(&mut self, annotation: *mut AstType) -> TypeId {
        if FFlag::DebugLuauMagicTypes.get() {
            let ref_ty = unsafe { ast_node_as::<AstTypeReference>(annotation as *mut AstNode) };

            if !ref_ty.is_null() {
                let ref_ty = unsafe { &*ref_ty };
                let name = unsafe { CStr::from_ptr(ref_ty.name.value) }.to_string_lossy();

                if name == "_luau_print" && ref_ty.parameters.size > 0 {
                    let param = unsafe { *ref_ty.parameters.data.add(0) };

                    if !param.r#type.is_null() {
                        let arg_ty = self.lookup_annotation(param.r#type);
                        let line = format!(
                            "_luau_print ({}, {}): {}\n",
                            unsafe { (*annotation).base.location.begin.line },
                            unsafe { (*annotation).base.location.begin.column },
                            to_string_type_id(arg_ty)
                        );

                        unsafe {
                            if let Some(print_line) =
                                crate::functions::set_print_line::luauPrintLine
                            {
                                print_line(&line);
                            }
                        }

                        return unsafe { follow_type_id(arg_ty) };
                    }
                } else if name == "_luau_force_constraint_solving_incomplete" {
                    let location = unsafe { (*annotation).base.location };
                    self.report_error_type_error_data_location(
                        ConstraintSolvingIncompleteError::default().into(),
                        &location,
                    );
                    return unsafe { (*self.builtin_types).anyType };
                }
            }
        }

        let ty = unsafe {
            (*self.module)
                .ast_resolved_types
                .find(&(annotation as *const AstType))
        };

        if unsafe { (*self.module).constraint_generation_did_not_complete } && ty.is_none() {
            return unsafe { (*self.builtin_types).anyType };
        }

        let ty = *ty.expect("Type annotation must be resolved");
        self.check_for_type_function_inhabitance(unsafe { follow_type_id(ty) }, unsafe {
            (*annotation).base.location
        })
    }
}
