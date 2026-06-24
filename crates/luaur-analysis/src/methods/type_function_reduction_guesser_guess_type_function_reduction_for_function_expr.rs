use crate::functions::begin_type_pack::begin;
use crate::functions::end_type_pack::end;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait};
use crate::records::instance_collector_2::InstanceCollector2;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::records::type_function_reduction_guess_result::TypeFunctionReductionGuessResult;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::ffi::c_void;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::vec_deque::VecDeque;

// C++ `struct InstanceCollector2 : TypeOnceVisitor` (TypeFunctionReductionGuesser.cpp).
// The virtual `visit(...)`/`cycle(...)` overrides live as the
// `GenericTypeVisitorTrait` impl so `traverse` dispatches into them; the bodies
// delegate to the inherent methods declared on the record / sibling files.
impl GenericTypeVisitorTrait for InstanceCollector2 {
    type Seen = DenseHashSet<*mut c_void>;

    fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
        &mut self.base.base
    }

    fn cycle_type_id(&mut self, ty: TypeId) {
        InstanceCollector2::cycle(self, ty);
    }

    fn visit_type_id_type_function_instance_type(
        &mut self,
        ty: TypeId,
        it: &TypeFunctionInstanceType,
    ) -> bool {
        InstanceCollector2::visit_type_id_type_function_instance_type(self, ty, it)
    }

    fn visit_type_id_extern_type(&mut self, ty: TypeId, et: &ExternType) -> bool {
        InstanceCollector2::visit_type_id_extern_type(self, ty, et)
    }

    fn visit_type_pack_id_type_function_instance_type_pack(
        &mut self,
        tp: TypePackId,
        itp: &TypeFunctionInstanceTypePack,
    ) -> bool {
        InstanceCollector2::visit_type_pack_id_type_function_instance_type_pack(self, tp, itp)
    }
}

impl TypeFunctionReductionGuesser {
    pub fn guess_type_function_reduction_for_function_expr(
        &mut self,
        expr: &AstExprFunction,
        ftv: &FunctionType,
        ret_ty: TypeId,
    ) -> TypeFunctionReductionGuessResult {
        let mut collector = InstanceCollector2::new();
        collector.traverse_type_id(ret_ty);
        self.to_infer = core::mem::replace(&mut collector.tys, VecDeque::new());
        self.cyclic_instances = core::mem::replace(
            &mut collector.cyclic_instance,
            DenseHashSet::new(core::ptr::null_mut()),
        );

        if self.is_function_generics_saturated(ftv, &mut collector.instance_arguments) {
            return TypeFunctionReductionGuessResult {
                guessed_function_annotations: Vec::new(),
                guessed_return_type: core::ptr::null(),
                should_recommend_annotation: false,
            };
        }
        self.infer();

        let mut results: Vec<(String, TypeId)> = Vec::new();
        let mut args: Vec<TypeId> = Vec::new();
        let mut it = begin(ftv.arg_types);
        let end_it = end(ftv.arg_types);
        while it.operator_ne(&end_it) {
            args.push(*it.operator_deref());
            it.operator_inc();
        }

        // Submit a guess for arg types
        for i in 0..expr.args.size {
            let local = unsafe { *expr.args.data.add(i) };
            if i >= args.len() {
                continue;
            }

            let arg_ty = args[i];
            let guessed_type = self.guess_type(arg_ty);
            let guessed_type = match guessed_type {
                Some(g) => g,
                None => continue,
            };
            let guess = unsafe { follow_type_id(guessed_type) };
            if !unsafe { get_type_id::<TypeFunctionInstanceType>(guess) }.is_null() {
                continue;
            }

            let name = unsafe {
                if (*local).name.value.is_null() {
                    String::new()
                } else {
                    core::ffi::CStr::from_ptr((*local).name.value)
                        .to_string_lossy()
                        .to_string()
                }
            };
            results.push((name, guess));
        }

        // Submit a guess for return types
        let recommended_annotation: TypeId;
        let guessed_return_type = self.guess_type(ret_ty);
        match guessed_return_type {
            None => recommended_annotation = unsafe { (*self.builtins).unknownType },
            Some(g) => recommended_annotation = unsafe { follow_type_id(g) },
        }
        let recommended_annotation = if !unsafe {
            get_type_id::<TypeFunctionInstanceType>(recommended_annotation)
        }
        .is_null()
        {
            unsafe { (*self.builtins).unknownType }
        } else {
            recommended_annotation
        };

        self.to_infer.clear();
        self.cyclic_instances.clear();
        self.function_reduces_to.clear();
        self.substitutable.clear();

        TypeFunctionReductionGuessResult {
            guessed_function_annotations: results,
            guessed_return_type: recommended_annotation,
            should_recommend_annotation: true,
        }
    }
}
