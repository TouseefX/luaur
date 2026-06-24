use crate::records::type_level::TypeLevel;

pub fn subsumes_ty_a_ty_b<TY_A, TY_B>(left: *mut TY_A, right: *mut TY_B) -> bool
where
    TY_A: HasLevel,
    TY_B: HasLevel,
{
    if left.is_null() {
        return true;
    }
    if right.is_null() {
        return false;
    }
    let left_level = unsafe { &*left }.level();
    let right_level = unsafe { &*right }.level();
    left_level.subsumes(&right_level)
}

pub trait HasLevel {
    fn level(&self) -> TypeLevel;
}
