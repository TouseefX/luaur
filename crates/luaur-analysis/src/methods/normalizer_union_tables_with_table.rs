use crate::functions::get_type_alt_j::get_type_id;
use crate::records::never_type::NeverType;
use crate::records::normalizer::Normalizer;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;

impl Normalizer {
    pub fn union_tables_with_table(&mut self, heres: &mut TypeIds, there: TypeId) {
        // we can always skip `never`
        let never_ptr = unsafe { get_type_id::<NeverType>(there) };
        if !never_ptr.is_null() {
            return;
        }

        heres.insert_type_id(there);
    }
}
