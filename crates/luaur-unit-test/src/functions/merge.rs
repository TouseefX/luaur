use luaur_analysis::functions::follow_type::follow_type_id;
use luaur_analysis::functions::get_type_alt_j::get_type_id;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::records::union_type::UnionType;
use luaur_analysis::type_aliases::refinement_map::RefinementMap;
use luaur_analysis::type_aliases::type_id::TypeId;
use std::collections::HashSet;

fn add_union_options(options: &mut Vec<TypeId>, seen: &mut HashSet<TypeId>, ty: TypeId) {
    let followed = unsafe { follow_type_id(ty) };
    let union = unsafe { get_type_id::<UnionType>(followed).as_ref() };

    if let Some(union) = union {
        for option in &union.options {
            if seen.insert(*option) {
                options.push(*option);
            }
        }
    } else if seen.insert(ty) {
        options.push(ty);
    }
}

pub fn merge(arena: &mut TypeArena, l: &mut RefinementMap, r: &RefinementMap) {
    let arena = arena as *mut TypeArena;

    luaur_analysis::functions::merge::merge(l, r, &|a, b| {
        let mut options = Vec::new();
        let mut seen = HashSet::new();

        add_union_options(&mut options, &mut seen, a);
        add_union_options(&mut options, &mut seen, b);

        if options.len() == 1 {
            options[0]
        } else {
            unsafe { (*arena).add_type(UnionType { options }) }
        }
    })
}
