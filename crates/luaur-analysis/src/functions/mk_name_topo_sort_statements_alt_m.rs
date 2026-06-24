use crate::functions::mk_name_topo_sort_statements_alt_h::mk_name_ast_stat_function;
use crate::functions::mk_name_topo_sort_statements_alt_i::mk_name_ast_stat_local_function;
use crate::functions::mk_name_topo_sort_statements_alt_j::mk_name_ast_stat_assign;
use crate::functions::mk_name_topo_sort_statements_alt_k::mk_name_ast_stat_local;
use crate::functions::mk_name_topo_sort_statements_alt_l::mk_name_ast_stat_type_alias;
use crate::records::identifier::Identifier;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_assign::AstStatAssign;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;
use luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias;
use luaur_ast::rtti::AstNodeClass;

pub fn mk_name_ast_stat(el: *mut AstStat) -> Option<Identifier> {
    if el.is_null() {
        return None;
    }

    let node = unsafe { &*el };
    let class_index = node.base.class_index;

    if class_index == AstStatFunction::CLASS_INDEX {
        let function = unsafe { &*(el as *mut AstStatFunction) };
        Some(mk_name_ast_stat_function(function))
    } else if class_index == AstStatLocalFunction::CLASS_INDEX {
        let function = unsafe { &*(el as *mut AstStatLocalFunction) };
        Some(mk_name_ast_stat_local_function(function))
    } else if class_index == AstStatAssign::CLASS_INDEX {
        let assign = unsafe { &*(el as *mut AstStatAssign) };
        mk_name_ast_stat_assign(assign)
    } else if class_index == AstStatLocal::CLASS_INDEX {
        let local = unsafe { &*(el as *mut AstStatLocal) };
        mk_name_ast_stat_local(local)
    } else if class_index == AstStatTypeAlias::CLASS_INDEX {
        let typealias = unsafe { &*(el as *mut AstStatTypeAlias) };
        Some(mk_name_ast_stat_type_alias(typealias))
    } else {
        None
    }
}
