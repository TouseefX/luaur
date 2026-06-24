use crate::enums::pack_field::PackField;
use crate::records::generic_pack_mapping::GenericPackMapping;
use crate::records::pack_slice::PackSlice;
use crate::records::path::Path;
use crate::type_aliases::component::Component;

pub fn is_path_on_argument_list(path: &Path) -> bool {
    let mut iter = path.components.iter();

    let first = match iter.next() {
        Some(c) => c,
        None => return false,
    };

    if let Component::PackField(args) = first {
        if *args != PackField::Arguments {
            return false;
        }
    } else {
        return false;
    }

    for component in iter {
        match component {
            Component::PackSlice(_) | Component::GenericPackMapping(_) => continue,
            Component::PackField(pack_field) => {
                if *pack_field != PackField::Tail {
                    return false;
                }
            }
            _ => return false,
        }
    }

    true
}
