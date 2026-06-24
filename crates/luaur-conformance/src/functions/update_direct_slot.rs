use crate::functions::get_or_create_atom::direct_slot_for_atom;

#[allow(non_snake_case)]
pub fn updateDirectSlot(atom: i32, cachedslot: *mut u16) {
    if let Some(slot) = direct_slot_for_atom(atom) {
        unsafe {
            *cachedslot = slot as u16;
        }
    }
}
