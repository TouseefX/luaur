use crate::functions::are_equal_structural_type_equality_alt_b::are_equal_seen_set_function_type_function_type;
use crate::functions::are_equal_structural_type_equality_alt_c::are_equal_seen_set_table_type_table_type;
use crate::functions::are_equal_structural_type_equality_alt_d::are_equal_seen_set_metatable_type_metatable_type;
use crate::records::any_type::AnyType;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::generic_type::GenericType;
use crate::records::metatable_type::MetatableType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::r#type::Type;
use crate::records::table_type::TableType;
use crate::type_aliases::bound_type::BoundType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::seen_set_structural_type_equality::SeenSet;
use crate::type_aliases::type_variant::TypeVariantMember;

#[allow(non_snake_case)]
pub fn are_equal_seen_set_type_item_type_item(seen: &mut SeenSet, lhs: &Type, rhs: &Type) -> bool {
    if let Some(bound) = BoundType::get_if(&lhs.ty) {
        return are_equal_seen_set_type_item_type_item(seen, unsafe { &*bound.boundTo }, rhs);
    }

    if let Some(bound) = BoundType::get_if(&rhs.ty) {
        return are_equal_seen_set_type_item_type_item(seen, lhs, unsafe { &*bound.boundTo });
    }

    if lhs.ty.index() != rhs.ty.index() {
        return false;
    }

    {
        let lf = FreeType::get_if(&lhs.ty);
        let rf = FreeType::get_if(&rhs.ty);
        if let (Some(lf), Some(rf)) = (lf, rf) {
            return lf.index == rf.index;
        }
    }

    {
        let lg = GenericType::get_if(&lhs.ty);
        let rg = GenericType::get_if(&rhs.ty);
        if let (Some(lg), Some(rg)) = (lg, rg) {
            return lg.index == rg.index;
        }
    }

    {
        let lp = PrimitiveType::get_if(&lhs.ty);
        let rp = PrimitiveType::get_if(&rhs.ty);
        if let (Some(lp), Some(rp)) = (lp, rp) {
            return lp.r#type == rp.r#type;
        }
    }

    {
        let lg = GenericType::get_if(&lhs.ty);
        let rg = GenericType::get_if(&rhs.ty);
        if let (Some(lg), Some(rg)) = (lg, rg) {
            return lg.index == rg.index;
        }
    }

    {
        let le = ErrorType::get_if(&lhs.ty);
        let re = ErrorType::get_if(&rhs.ty);
        if let (Some(le), Some(re)) = (le, re) {
            return le.index == re.index;
        }
    }

    {
        let lf = FunctionType::get_if(&lhs.ty);
        let rf = FunctionType::get_if(&rhs.ty);
        if let (Some(lf), Some(rf)) = (lf, rf) {
            return are_equal_seen_set_function_type_function_type(seen, lf, rf);
        }
    }

    {
        let lt = TableType::get_if(&lhs.ty);
        let rt = TableType::get_if(&rhs.ty);
        if let (Some(lt), Some(rt)) = (lt, rt) {
            return are_equal_seen_set_table_type_table_type(seen, lt, rt);
        }
    }

    {
        let lmt = MetatableType::get_if(&lhs.ty);
        let rmt = MetatableType::get_if(&rhs.ty);
        if let (Some(lmt), Some(rmt)) = (lmt, rmt) {
            return are_equal_seen_set_metatable_type_metatable_type(seen, lmt, rmt);
        }
    }

    if AnyType::get_if(&lhs.ty).is_some() && AnyType::get_if(&rhs.ty).is_some() {
        return true;
    }

    false
}
