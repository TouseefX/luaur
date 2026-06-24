use crate::functions::emit_warning::emit_warning;
use crate::records::lint_format_string::LintFormatString;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_constant_bool::AstExprConstantBool;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti;
use luaur_config::enums::code::Code;
use luaur_config::records::lint_warning::LintWarning;

impl LintFormatString {
    pub fn match_string_call(
        &mut self,
        name: AstName,
        self_expr: *mut AstExpr,
        args: AstArray<*mut AstExpr>,
    ) {
        let is_format = name.operator_eq_c_char(c"format".as_ptr());
        let is_pack_packsize_unpack = name.operator_eq_c_char(c"pack".as_ptr())
            || name.operator_eq_c_char(c"packsize".as_ptr())
            || name.operator_eq_c_char(c"unpack".as_ptr());
        let is_match_gmatch = name.operator_eq_c_char(c"match".as_ptr())
            || name.operator_eq_c_char(c"gmatch".as_ptr());
        let is_find = name.operator_eq_c_char(c"find".as_ptr());
        let is_gsub = name.operator_eq_c_char(c"gsub".as_ptr());

        if is_format {
            let fmt_node =
                unsafe { rtti::ast_node_as::<AstExprConstantString>(self_expr as *mut AstNode) };
            if !fmt_node.is_null() {
                let fmt = unsafe { &*fmt_node };
                let error = self.check_string_format(fmt.value.data, fmt.value.size);
                if !error.is_null() {
                    emit_warning(
                        unsafe { &mut *self.context },
                        Code::Code_FormatString,
                        fmt.base.base.location,
                        format_args!("Invalid format string: {}", unsafe {
                            core::ffi::CStr::from_ptr(error).to_string_lossy()
                        }),
                    );
                }
            }
        } else if is_pack_packsize_unpack {
            let fmt_node =
                unsafe { rtti::ast_node_as::<AstExprConstantString>(self_expr as *mut AstNode) };
            if !fmt_node.is_null() {
                let fmt = unsafe { &*fmt_node };
                let is_packsize = name.operator_eq_c_char(c"packsize".as_ptr());
                let error = self.check_string_pack(fmt.value.data, fmt.value.size, is_packsize);
                if !error.is_null() {
                    emit_warning(
                        unsafe { &mut *self.context },
                        Code::Code_FormatString,
                        fmt.base.base.location,
                        format_args!("Invalid pack format: {}", unsafe {
                            core::ffi::CStr::from_ptr(error).to_string_lossy()
                        }),
                    );
                }
            }
        } else if is_match_gmatch && args.size > 0 {
            let first_arg = unsafe { *args.data.add(0) };
            let pat_node =
                unsafe { rtti::ast_node_as::<AstExprConstantString>(first_arg as *mut AstNode) };
            if !pat_node.is_null() {
                let pat = unsafe { &*pat_node };
                let error =
                    self.check_string_match(pat.value.data, pat.value.size, core::ptr::null_mut());
                if !error.is_null() {
                    emit_warning(
                        unsafe { &mut *self.context },
                        Code::Code_FormatString,
                        pat.base.base.location,
                        format_args!("Invalid match pattern: {}", unsafe {
                            core::ffi::CStr::from_ptr(error).to_string_lossy()
                        }),
                    );
                }
            }
        } else if is_find && args.size > 0 && args.size <= 2 {
            let first_arg = unsafe { *args.data.add(0) };
            let pat_node =
                unsafe { rtti::ast_node_as::<AstExprConstantString>(first_arg as *mut AstNode) };
            if !pat_node.is_null() {
                let pat = unsafe { &*pat_node };
                let error =
                    self.check_string_match(pat.value.data, pat.value.size, core::ptr::null_mut());
                if !error.is_null() {
                    emit_warning(
                        unsafe { &mut *self.context },
                        Code::Code_FormatString,
                        pat.base.base.location,
                        format_args!("Invalid match pattern: {}", unsafe {
                            core::ffi::CStr::from_ptr(error).to_string_lossy()
                        }),
                    );
                }
            }
        } else if is_find && args.size >= 3 {
            let third_arg = unsafe { *args.data.add(2) };
            let mode =
                unsafe { rtti::ast_node_as::<AstExprConstantBool>(third_arg as *mut AstNode) };
            if !mode.is_null() {
                let mode_val = unsafe { &*mode };
                if !mode_val.value {
                    let first_arg = unsafe { *args.data.add(0) };
                    let pat_node = unsafe {
                        rtti::ast_node_as::<AstExprConstantString>(first_arg as *mut AstNode)
                    };
                    if !pat_node.is_null() {
                        let pat = unsafe { &*pat_node };
                        let error = self.check_string_match(
                            pat.value.data,
                            pat.value.size,
                            core::ptr::null_mut(),
                        );
                        if !error.is_null() {
                            emit_warning(
                                unsafe { &mut *self.context },
                                Code::Code_FormatString,
                                pat.base.base.location,
                                format_args!("Invalid match pattern: {}", unsafe {
                                    core::ffi::CStr::from_ptr(error).to_string_lossy()
                                }),
                            );
                        }
                    }
                }
            }
        } else if is_gsub && args.size > 1 {
            let mut captures = -1;

            let first_arg = unsafe { *args.data.add(0) };
            let pat_node =
                unsafe { rtti::ast_node_as::<AstExprConstantString>(first_arg as *mut AstNode) };
            if !pat_node.is_null() {
                let pat = unsafe { &*pat_node };
                let error = self.check_string_match(pat.value.data, pat.value.size, &mut captures);
                if !error.is_null() {
                    emit_warning(
                        unsafe { &mut *self.context },
                        Code::Code_FormatString,
                        pat.base.base.location,
                        format_args!("Invalid match pattern: {}", unsafe {
                            core::ffi::CStr::from_ptr(error).to_string_lossy()
                        }),
                    );
                }
            }

            let second_arg = unsafe { *args.data.add(1) };
            let rep_node =
                unsafe { rtti::ast_node_as::<AstExprConstantString>(second_arg as *mut AstNode) };
            if !rep_node.is_null() {
                let rep = unsafe { &*rep_node };
                let error = self.check_string_replace(rep.value.data, rep.value.size, captures);
                if !error.is_null() {
                    emit_warning(
                        unsafe { &mut *self.context },
                        Code::Code_FormatString,
                        rep.base.base.location,
                        format_args!("Invalid match replacement: {}", unsafe {
                            core::ffi::CStr::from_ptr(error).to_string_lossy()
                        }),
                    );
                }
            }
        }
    }
}
