use crate::functions::follow_type::follow_type_id;
use crate::functions::get_paren_recommendation_for_func::get_paren_recommendation_for_func;
use crate::functions::get_paren_recommendation_for_intersect::get_paren_recommendation_for_intersect;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_node::AstNode;

use crate::enums::parentheses_recommendation::ParenthesesRecommendation;
use crate::enums::type_correct_kind::TypeCorrectKind;

pub fn get_paren_recommendation(
    id: TypeId,
    nodes: &alloc::vec::Vec<*mut AstNode>,
    type_correct: TypeCorrectKind,
) -> ParenthesesRecommendation {
    if type_correct == TypeCorrectKind::Correct {
        return ParenthesesRecommendation::None;
    }

    let id = unsafe { follow_type_id(id) };

    unsafe {
        let func = get_type_id::<FunctionType>(id);
        if !func.is_null() {
            return get_paren_recommendation_for_func(&*func, nodes);
        }

        let intersect = get_type_id::<IntersectionType>(id);
        if !intersect.is_null() {
            return get_paren_recommendation_for_intersect(&*intersect, nodes);
        }
    }

    ParenthesesRecommendation::None
}
