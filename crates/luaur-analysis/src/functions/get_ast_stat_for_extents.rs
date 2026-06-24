use luaur_ast::records::ast_stat_for::AstStatFor;
use luaur_ast::records::location::Location;

pub fn get_ast_stat_for_extents(for_stat: *mut AstStatFor) -> Location {
    let begin = unsafe { (*for_stat).base.base.location.begin };
    let mut end = unsafe { (*for_stat).base.base.location.end };

    let step = unsafe { (*for_stat).step };
    if !step.is_null() {
        end = unsafe { (*step).base.location.end };
    } else {
        let to = unsafe { (*for_stat).to };
        if !to.is_null() {
            end = unsafe { (*to).base.location.end };
        } else {
            let from = unsafe { (*for_stat).from };
            if !from.is_null() {
                end = unsafe { (*from).base.location.end };
            } else {
                let var = unsafe { (*for_stat).var };
                if !var.is_null() {
                    end = unsafe { (*var).location.end };
                }
            }
        }
    }

    Location::new(begin, end)
}
