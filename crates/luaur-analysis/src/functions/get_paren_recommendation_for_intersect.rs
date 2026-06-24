use crate::enums::parentheses_recommendation::ParenthesesRecommendation;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_paren_recommendation_for_func::get_paren_recommendation_for_func;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use luaur_ast::records::ast_node::AstNode;

pub fn get_paren_recommendation_for_intersect(
    intersect: &IntersectionType,
    nodes: &alloc::vec::Vec<*mut AstNode>,
) -> ParenthesesRecommendation {
    let mut rec = ParenthesesRecommendation::None;

    for &part_id in intersect.parts.iter() {
        let part_id = unsafe { follow_type_id(part_id) };

        let part_func = unsafe { get_type_id::<FunctionType>(part_id).as_ref() };
        if let Some(part_func) = part_func {
            let other = get_paren_recommendation_for_func(part_func, nodes);
            if other as i32 > rec as i32 {
                rec = other;
            }
        } else {
            return ParenthesesRecommendation::None;
        }
    }

    rec
}
