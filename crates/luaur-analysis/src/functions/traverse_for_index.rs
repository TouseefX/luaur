use crate::records::index::Index;
use crate::records::pack_slice::PackSlice;
use crate::records::path::Path;
use crate::type_aliases::component::Component;

pub fn traverse_for_index(path: &Path) -> Option<usize> {
    let mut component_iter = path.components.iter();
    let mut index: usize = 0;

    // Get the last component index
    let components_len = path.components.len();
    if components_len == 0 {
        return None;
    }
    let last_component_index = components_len - 1;

    // Iterate up to but not including the last component
    for i in 0..last_component_index {
        let component = &path.components[i];

        if let Component::PackSlice(pack_slice) = component {
            index += pack_slice.start_index;
        } else {
            return None;
        }
    }

    // Check the last component
    let last_component = &path.components[last_component_index];
    if let Component::Index(index_component) = last_component {
        index += index_component.index;
        return Some(index);
    }

    None
}
