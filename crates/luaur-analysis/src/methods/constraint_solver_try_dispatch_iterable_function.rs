use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::functions::get_type_id::get_type_id;
use crate::records::blocked_type_pack::BlockedTypePack;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::function_call_constraint::FunctionCallConstraint;
use crate::records::function_type::FunctionType;
use crate::records::iterable_constraint::IterableConstraint;
use crate::records::unpack_constraint::UnpackConstraint;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::type_id::TypeId;
use core::ptr::NonNull;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintSolver {
    pub fn try_dispatch_iterable_function(
        &mut self,
        next_ty: TypeId,
        table_ty: TypeId,
        c: &IterableConstraint,
        constraint: *const Constraint,
    ) -> bool {
        let next_fn = unsafe { get_type_id::<FunctionType>(next_ty) };
        LUAU_ASSERT!(!next_fn.is_null());

        unsafe {
            (*(*c.ast_for_in_next_types)
                .get_or_insert(c.next_ast_fragment as *const crate::records::ast_node::AstNode)) =
                next_ty
        };

        let table_ty_pack =
            unsafe { (*self.arena).add_type_pack_initializer_list_type_id(&[table_ty]) };

        let variables_pack = unsafe {
            let mut btp = BlockedTypePack {
                index: 0,
                owner: core::ptr::null_mut(),
            };
            btp.blocked_type_pack_blocked_type_pack();
            (*self.arena).add_type_pack_t(btp)
        };

        let call_constraint = self.push_constraint(
            NonNull::new(unsafe { (*constraint).scope }).unwrap(),
            unsafe { (*constraint).location },
            ConstraintV::FunctionCall(FunctionCallConstraint {
                fn_type: next_ty,
                args_pack: table_ty_pack,
                result: variables_pack,
                call_site: core::ptr::null_mut(),
                discriminant_types: alloc::vec::Vec::new(),
                type_arguments: alloc::vec::Vec::new(),
                type_pack_arguments: alloc::vec::Vec::new(),
                ast_overload_resolved_types: core::ptr::null_mut(),
            }),
        );

        unsafe {
            (*get_mutable_type_pack_id::<BlockedTypePack>(variables_pack)).owner =
                call_constraint.as_ptr()
        };

        let unpack_constraint = self.unpack_and_assign(
            c.variables.clone(),
            variables_pack,
            NonNull::new(constraint as *mut Constraint).unwrap(),
        );

        self.inherit_blocks(constraint, call_constraint.as_ptr());
        self.inherit_blocks(unpack_constraint.as_ptr(), call_constraint.as_ptr());

        true
    }
}
