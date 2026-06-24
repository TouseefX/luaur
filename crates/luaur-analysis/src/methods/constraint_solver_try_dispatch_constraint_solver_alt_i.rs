use crate::enums::polarity::Polarity;
use crate::functions::begin_type_pack::begin_type_pack_id;
use crate::functions::end_type_pack::end_type_pack_id;
use crate::functions::find_blocked_arg_types_in::find_blocked_arg_types_in;
use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_constraint::get_constraint;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::push_type_into::push_type_into;
use crate::functions::unwrap_group::unwrap_group;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::constraint_v::ConstraintV;
use crate::records::dense_hash_map::DenseHashMap;
use crate::records::dense_hash_set::DenseHashSet;
use crate::records::function_check_constraint::FunctionCheckConstraint;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::incomplete_inference::IncompleteInference;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::normalizer::Normalizer;
use crate::records::push_type_constraint::PushTypeConstraint;
use crate::records::push_type_result::PushTypeResult;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::type_arena::TypeArena;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::records::type_pair_hash::TypePairHash;
use crate::records::unifier_2::Unifier2;
use crate::type_aliases::constraint_v::ConstraintV as ConstraintVMember;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::ptr::NonNull;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;
use luaur_common::records::dense_hash_map::DenseHashMap as CommonDenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet as CommonDenseHashSet;

impl ConstraintSolver {
    pub fn try_dispatch_function_check_constraint_not_null_constraint_bool(
        &mut self,
        c: &FunctionCheckConstraint,
        constraint: *const Constraint,
        force: bool,
    ) -> bool {
        let fn_ty = unsafe { follow_type_id(c.fn_type) };
        let args_pack = unsafe { follow_type_pack_id(c.args_pack) };

        if self.is_blocked_type_id(fn_ty) {
            return self.block_type_id_not_null_constraint(fn_ty, constraint);
        }

        if self.is_blocked_type_pack_id(args_pack) {
            return true;
        }

        let blocked_types = find_blocked_arg_types_in(
            c.call_site,
            c.ast_types as *mut CommonDenseHashMap<*const AstExpr, TypeId>,
        );
        for ty in &blocked_types {
            self.block_type_id_not_null_constraint(*ty, constraint);
        }
        if !blocked_types.is_empty() {
            return false;
        }

        let ftv = unsafe { get_type_id::<FunctionType>(fn_ty) };
        if ftv.is_null() {
            return true;
        }

        let mut replacements: DenseHashMap<TypeId, TypeId> =
            DenseHashMap::new(core::ptr::null_mut());
        let mut replacement_packs: DenseHashMap<TypePackId, TypePackId> =
            DenseHashMap::new(core::ptr::null_mut());

        let mut generic_types_and_packs: DenseHashSet<*const c_void> =
            DenseHashSet::new(core::ptr::null_mut());

        let mut u2 = Unifier2::unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter(
            NonNull::new(self.arena).unwrap(),
            NonNull::new(self.builtin_types).unwrap(),
            NonNull::new(constraint_scope(constraint)).unwrap(),
            NonNull::new(&self.ice_reporter as *const InternalErrorReporter as *mut InternalErrorReporter).unwrap(),
        );

        for generic in unsafe { &(*ftv).generics } {
            // We may see non-generic types here, for example when evaluating a
            // recursive function call.
            let gty = unsafe { get_type_id::<GenericType>(follow_type_id(*generic)) };
            if !gty.is_null() {
                let repl_ty = if unsafe { (*gty).polarity } == Polarity::Negative {
                    unsafe { (*self.builtin_types).neverType }
                } else {
                    unsafe { (*self.builtin_types).unknownType }
                };
                replacements.try_insert(*generic, repl_ty);
                generic_types_and_packs.insert_mut(*generic as *const c_void);
            }
        }

        for generic_pack in unsafe { &(*ftv).generic_packs } {
            replacement_packs.try_insert(*generic_pack, unsafe {
                (*self.builtin_types).unknownTypePack
            });
            generic_types_and_packs.insert_mut(*generic_pack as *const c_void);
        }

        let (expected_args, _) = flatten_type_pack_id(unsafe { (*ftv).arg_types });
        let (arg_pack_head, _) = flatten_type_pack_id(args_pack);

        // If this is a self call, the types will have more elements than the AST call.
        // We don't attempt to perform bidirectional inference on the self type.
        let type_offset = if unsafe { (*c.call_site).self_ } {
            1
        } else {
            0
        };

        let mut subtyping = Subtyping::subtyping_owned(
            self.builtin_types,
            self.arena,
            self.normalizer,
            self.type_function_runtime,
            &self.ice_reporter as *const InternalErrorReporter as *mut InternalErrorReporter,
        );

        let call_site_args_size = unsafe { (*c.call_site).args.size };
        let args_data = unsafe { (*c.call_site).args.data };

        for i in 0..call_site_args_size {
            if i as usize + type_offset >= expected_args.len()
                || i as usize + type_offset >= arg_pack_head.len()
            {
                break;
            }

            let expected_arg_ty =
                unsafe { follow_type_id(expected_args[i as usize + type_offset]) };
            let expr = unsafe { unwrap_group(*args_data.add(i as usize)) };

            let result = push_type_into(
                NonNull::new(c.ast_types as *mut CommonDenseHashMap<*const AstExpr, TypeId>)
                    .unwrap(),
                NonNull::new(
                    c.ast_expected_types as *mut CommonDenseHashMap<*const AstExpr, TypeId>,
                )
                .unwrap(),
                NonNull::new(self as *mut ConstraintSolver).unwrap(),
                NonNull::new(constraint as *mut Constraint).unwrap(),
                NonNull::new(&mut generic_types_and_packs as *mut DenseHashSet<*const c_void>)
                    .unwrap(),
                NonNull::new(&mut u2 as *mut Unifier2).unwrap(),
                NonNull::new(&mut subtyping as *mut Subtyping).unwrap(),
                expected_arg_ty,
                expr as *const AstExpr,
            );

            if !force && !result.incomplete_types.is_empty() {
                for incomplete in &result.incomplete_types {
                    let addition = self.push_constraint(
                        NonNull::new(constraint_scope(constraint)).unwrap(),
                        unsafe { (*constraint).location },
                        ConstraintV::PushType(PushTypeConstraint {
                            expectedType: incomplete.expectedType,
                            targetType: incomplete.targetType,
                            astTypes: c.ast_types,
                            astExpectedTypes: c.ast_expected_types,
                            expr: incomplete.expr as *const AstExpr,
                        }),
                    );
                    self.inherit_blocks(constraint, addition.as_ptr());
                }
            }
        }

        let incomplete_subtypes = u2.incomplete_subtypes.clone();
        for c_item in incomplete_subtypes {
            let addition = self.push_constraint(
                NonNull::new(constraint_scope(constraint)).unwrap(),
                unsafe { (*constraint).location },
                c_item,
            );
            self.inherit_blocks(constraint, addition.as_ptr());
        }

        true
    }
}

fn constraint_scope(c: *const Constraint) -> *mut Scope {
    unsafe { (*c).scope }
}
