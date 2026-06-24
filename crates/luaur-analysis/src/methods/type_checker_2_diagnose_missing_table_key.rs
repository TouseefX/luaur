use crate::functions::follow_type::follow_type_id;
use crate::functions::get_table_type::get_table_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::extern_type::ExternType;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp;
use crate::records::unknown_property::UnknownProperty;
use crate::type_aliases::type_error_data::TypeErrorData;
use alloc::collections::BTreeSet;
use alloc::string::String;
use luaur_common::functions::equals_lower::equalsLower;

impl TypeChecker2 {
    pub fn diagnose_missing_table_key(&self, utk: &UnknownProperty, data: &mut TypeErrorData) {
        let sv = utk.key();
        let mut candidates: BTreeSet<String> = BTreeSet::new();

        let mut accumulate = |props: &crate::type_aliases::props_type::Props| {
            for (name, _) in props {
                if sv != name.as_str() && equalsLower(sv.as_bytes(), name.as_str().as_bytes()) {
                    candidates.insert(name.clone());
                }
            }
        };

        if let Some(ttv) = get_table_type(utk.table()) {
            accumulate(&ttv.props);
        } else if let Some(etv) =
            unsafe { get_type_id::<ExternType>(follow_type_id(utk.table())).as_ref() }
        {
            let mut current = Some(etv);
            while let Some(et) = current {
                accumulate(&et.props);

                if et.parent.is_none() {
                    break;
                }

                current = unsafe {
                    get_type_id::<ExternType>(follow_type_id(et.parent.unwrap())).as_ref()
                };
            }
        }

        if !candidates.is_empty() {
            *data = TypeErrorData::UnknownPropButFoundLikeProp(UnknownPropButFoundLikeProp {
                table: utk.table(),
                key: utk.key().to_string(),
                candidates,
            });
        }
    }
}
