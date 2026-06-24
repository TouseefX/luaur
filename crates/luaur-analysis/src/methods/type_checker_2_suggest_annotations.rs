use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::explicit_function_annotation_recommended::ExplicitFunctionAnnotationRecommended;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::type_arena::TypeArena;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_error_data::IntoTypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::vec_deque::VecDeque;
use luaur_common::LUAU_ASSERT;

impl TypeChecker2 {
    pub fn type_checker_2_suggest_annotations(&mut self, expr: *mut AstExprFunction, ty: TypeId) {
        let inferred_ftv_ptr = unsafe { get_type_id::<FunctionType>(ty) };
        LUAU_ASSERT!(!inferred_ftv_ptr.is_null());
        let inferred_ftv = unsafe { &*inferred_ftv_ptr };

        let mut work_list: VecDeque<TypeId> = VecDeque::new();
        let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(core::ptr::null());

        let mut guesser = TypeFunctionReductionGuesser::type_function_reduction_guesser_type_function_reduction_guesser(
            unsafe { &mut (*self.module).internal_types as *mut TypeArena },
            self.builtin_types,
            &mut self.normalizer as *mut crate::records::normalizer::Normalizer,
        );

        let (ret_head, _ret_tail) = flatten_type_pack_id(inferred_ftv.ret_types);
        for ret_ty in ret_head {
            work_list.push_back(ret_ty);
        }

        while !work_list.empty() {
            let t = unsafe { follow_type_id(*work_list.front()) };
            work_list.pop_front();

            if seen.contains(&t) {
                continue;
            }
            seen.insert(t);

            let ut_ptr = unsafe { get_type_id::<UnionType>(t) };
            if !ut_ptr.is_null() {
                let ut = unsafe { &*ut_ptr };
                for &part in &ut.options {
                    work_list.push_back(part);
                }
                continue;
            }

            let it_ptr = unsafe { get_type_id::<IntersectionType>(t) };
            if !it_ptr.is_null() {
                let it = unsafe { &*it_ptr };
                for &part in &it.parts {
                    work_list.push_back(part);
                }
                continue;
            }

            let tfi_ptr = unsafe { get_type_id::<TypeFunctionInstanceType>(t) };
            if !tfi_ptr.is_null() {
                let result = guesser.guess_type_function_reduction_for_function_expr(
                    unsafe { &*expr },
                    inferred_ftv,
                    t,
                );
                if result.should_recommend_annotation
                    && unsafe { get_type_id::<UnknownType>(result.guessed_return_type) }.is_null()
                {
                    let err = ExplicitFunctionAnnotationRecommended {
                        recommended_args: result.guessed_function_annotations,
                        recommended_return: result.guessed_return_type,
                    };
                    let location = unsafe { (*expr).base.base.location };
                    self.report_error_type_error_data_location(
                        err.into_type_error_data(),
                        &location,
                    );
                }
            }
        }
    }
}
