use crate::functions::check_overloaded_documentation_symbol::check_overloaded_documentation_symbol;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::module::Module;
use crate::records::table_type::TableType;
use crate::type_aliases::documentation_symbol::DocumentationSymbol;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_name::AstName;

pub fn get_metatable_documentation(
    module: &Module,
    parent_expr: *const AstExpr,
    mtable: &TableType,
    index: &AstName,
) -> Option<DocumentationSymbol> {
    // C++: auto indexIt = mtable->props.find("__index");
    let index_prop = mtable.props.get("__index")?;

    let followed = if let Some(read_ty) = index_prop.read_ty {
        unsafe { follow_type_id(read_ty) }
    } else if let Some(write_ty) = index_prop.write_ty {
        unsafe { follow_type_id(write_ty) }
    } else {
        return None;
    };

    let ttv_ptr = unsafe { get_type_id::<TableType>(followed) };
    if ttv_ptr.is_null() {
        return None;
    }

    let ttv = unsafe { &*ttv_ptr };
    // C++: auto propIt = ttv->props.find(index.value); — props keyed by std::string.
    let index_key = unsafe {
        core::ffi::CStr::from_ptr(index.value)
            .to_string_lossy()
            .into_owned()
    };
    let prop = ttv.props.get(&index_key)?;

    if let Some(ty) = prop.read_ty {
        return check_overloaded_documentation_symbol(
            module,
            ty,
            parent_expr,
            prop.documentation_symbol.clone(),
        );
    }

    None
}
