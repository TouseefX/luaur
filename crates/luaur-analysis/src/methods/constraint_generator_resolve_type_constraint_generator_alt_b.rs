use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::intersection_type::IntersectionType;
use crate::records::module::Module;
use crate::records::scope::Scope;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;
use crate::records::union_type::UnionType;
use crate::type_aliases::singleton_variant::SingletonVariant;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use alloc::vec::Vec;
use core::mem::ManuallyDrop;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_error::AstTypeError;
use luaur_ast::records::ast_type_function::AstTypeFunction;
use luaur_ast::records::ast_type_group::AstTypeGroup;
use luaur_ast::records::ast_type_intersection::AstTypeIntersection;
use luaur_ast::records::ast_type_optional::AstTypeOptional;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::records::ast_type_singleton_bool::AstTypeSingletonBool;
use luaur_ast::records::ast_type_singleton_string::AstTypeSingletonString;
use luaur_ast::records::ast_type_table::AstTypeTable;
use luaur_ast::records::ast_type_typeof::AstTypeTypeof;
use luaur_ast::records::ast_type_union::AstTypeUnion;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintGenerator {
    // ConstraintGenerator::resolveType_(const ScopePtr&, AstType*, bool, bool)
    // (ConstraintGenerator.cpp:4578).
    pub fn resolve_type_constraint_generator_alt_b(
        &mut self,
        scope: *mut Scope,
        ty: *mut AstType,
        in_type_arguments: bool,
        replace_error_with_fresh: bool,
    ) -> TypeId {
        // The resolve helpers and `check`/`freshType` want a `const ScopePtr&`; the
        // C++ overload also takes a `const ScopePtr&`. Reconstruct one without
        // taking ownership of the refcount.
        let sp = ManuallyDrop::new(unsafe { alloc::sync::Arc::from_raw(scope as *const Scope) });

        let node = ty as *mut AstNode;
        let mut result: TypeId = core::ptr::null();

        let ref_ = unsafe { (*node).as_item_mut::<AstTypeReference>() };
        let tab = unsafe { (*node).as_item_mut::<AstTypeTable>() };
        let fn_ = unsafe { (*node).as_item_mut::<AstTypeFunction>() };
        let tof = unsafe { (*node).as_item_mut::<AstTypeTypeof>() };
        if !ref_.is_null() {
            result = self.resolve_reference_type(
                &sp,
                ty,
                ref_,
                in_type_arguments,
                replace_error_with_fresh,
            );
        } else {
            if !tab.is_null() {
                result = self.resolve_table_type(
                    scope,
                    ty,
                    tab,
                    in_type_arguments,
                    replace_error_with_fresh,
                );
            } else {
                if !fn_.is_null() {
                    result = self.resolve_function_type(
                        &sp,
                        ty,
                        unsafe { &*fn_ },
                        in_type_arguments,
                        replace_error_with_fresh,
                    );
                } else {
                    if !tof.is_null() {
                        let expr_type = self
                            .check_scope_ptr_ast_expr(&sp, unsafe { (*tof).expr })
                            .ty;
                        result = expr_type;
                    } else if !unsafe { (*node).as_item_mut::<AstTypeOptional>() }.is_null() {
                        result = unsafe { (*self.builtin_types).nilType };
                    } else if !unsafe { (*node).as_item_mut::<AstTypeUnion>() }.is_null() {
                        let union_annotation = unsafe { (*node).as_item_mut::<AstTypeUnion>() };
                        if unsafe { (*union_annotation).types.size } == 1 {
                            result = self.resolve_type_constraint_generator_alt_b(
                                scope,
                                unsafe { *(*union_annotation).types.data },
                                in_type_arguments,
                                false,
                            );
                        } else {
                            let mut parts: Vec<TypeId> = Vec::new();
                            let types = unsafe { (*union_annotation).types };
                            for i in 0..types.size as usize {
                                let part = unsafe { *types.data.add(i) };
                                parts.push(self.resolve_type_constraint_generator_alt_b(
                                    scope,
                                    part,
                                    in_type_arguments,
                                    false,
                                ));
                            }
                            result =
                                unsafe { (*self.arena).add_type(UnionType { options: parts }) };
                        }
                    } else if !unsafe { (*node).as_item_mut::<AstTypeIntersection>() }.is_null() {
                        let intersection_annotation =
                            unsafe { (*node).as_item_mut::<AstTypeIntersection>() };
                        if unsafe { (*intersection_annotation).types.size } == 1 {
                            result = self.resolve_type_constraint_generator_alt_b(
                                scope,
                                unsafe { *(*intersection_annotation).types.data },
                                in_type_arguments,
                                false,
                            );
                        } else {
                            let mut parts: Vec<TypeId> = Vec::new();
                            let types = unsafe { (*intersection_annotation).types };
                            for i in 0..types.size as usize {
                                let part = unsafe { *types.data.add(i) };
                                parts.push(self.resolve_type_constraint_generator_alt_b(
                                    scope,
                                    part,
                                    in_type_arguments,
                                    false,
                                ));
                            }
                            result = unsafe { (*self.arena).add_type(IntersectionType { parts }) };
                        }
                    } else if !unsafe { (*node).as_item_mut::<AstTypeGroup>() }.is_null() {
                        let type_group_annotation =
                            unsafe { (*node).as_item_mut::<AstTypeGroup>() };
                        result = self.resolve_type_constraint_generator_alt_b(
                            scope,
                            unsafe { (*type_group_annotation).type_ },
                            in_type_arguments,
                            false,
                        );
                    } else if !unsafe { (*node).as_item_mut::<AstTypeSingletonBool>() }.is_null() {
                        let bool_annotation =
                            unsafe { (*node).as_item_mut::<AstTypeSingletonBool>() };
                        if unsafe { (*bool_annotation).value } {
                            result = unsafe { (*self.builtin_types).trueType };
                        } else {
                            result = unsafe { (*self.builtin_types).falseType };
                        }
                    } else if !unsafe { (*node).as_item_mut::<AstTypeSingletonString>() }.is_null()
                    {
                        let string_annotation =
                            unsafe { (*node).as_item_mut::<AstTypeSingletonString>() };
                        let s: String = unsafe {
                            let value = (*string_annotation).value;
                            let bytes = core::slice::from_raw_parts(
                                value.data as *const u8,
                                value.size as usize,
                            );
                            String::from(core::str::from_utf8(bytes).unwrap_or(""))
                        };
                        result = unsafe {
                            (*self.arena).add_type(SingletonType::singleton_type(
                                SingletonVariant::V1(StringSingleton::new(s)),
                            ))
                        };
                    } else if !unsafe { (*node).as_item_mut::<AstTypeError>() }.is_null() {
                        result = unsafe { (*self.builtin_types).errorType };
                        if replace_error_with_fresh {
                            result = self.fresh_type(&sp, self.polarity);
                        }
                    } else {
                        LUAU_ASSERT!(false);
                        result = unsafe { (*self.builtin_types).errorType };
                    }
                }
            }
        }

        if let Some(module) = &self.module {
            let module_ptr = alloc::sync::Arc::as_ptr(module) as *mut Module;
            unsafe {
                *(*module_ptr)
                    .ast_resolved_types
                    .get_or_insert(ty as *const AstType) = result;
            }
        }

        result
    }
}
