use crate::records::cli_file_resolver::CliFileResolver;
use crate::records::file_navigation_context::FileNavigationContext;
use crate::records::luau_config_interrupt_info::LuauConfigInterruptInfo;
use alloc::boxed::Box;
use alloc::string::String;
use luaur_analysis::records::module_info::ModuleInfo;
use luaur_analysis::records::type_check_limits::TypeCheckLimits;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;
use luaur_require::enums::status_require_navigator::Status;
use luaur_require::records::error_handler::ErrorHandler;
use luaur_require::records::navigator::Navigator;

/// `Luau::Require::ErrorHandler nullErrorHandler{};` — a no-op error handler
/// (`CLI/src/Analyze.cpp:207`).
struct NullErrorHandler;

impl ErrorHandler for NullErrorHandler {
    fn report_error(&mut self, _message: String) {}
}

impl CliFileResolver {
    /// C++ `std::optional<ModuleInfo> resolveModule(const ModuleInfo* context, AstExpr* node, const TypeCheckLimits& limits)`
    /// (`CLI/src/Analyze.cpp:185-221`).
    pub unsafe fn resolve_module(
        &mut self,
        context: *const ModuleInfo,
        expr: *mut AstExpr,
        limits: &TypeCheckLimits,
    ) -> Option<ModuleInfo> {
        // if (AstExprConstantString* expr = node->as<AstExprConstantString>())
        let const_str = ast_node_as::<AstExprConstantString>(expr as *mut AstNode);
        if const_str.is_null() {
            return None;
        }

        // std::string path{expr->value.data, expr->value.size};
        let bytes: &[u8] = {
            let slice = (*const_str).value.as_slice();
            core::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len())
        };
        let path = String::from_utf8_lossy(bytes).into_owned();

        // FileNavigationContext navigationContext{context->name};
        let mut navigation_context = FileNavigationContext::new((*context).name.clone());

        // LuauConfigInterruptInfo info = {limits, path};
        // navigationContext.luauConfigInit / luauConfigInterrupt capture &info.
        navigation_context.interrupt_info = Some(Box::new(LuauConfigInterruptInfo {
            limits: limits.clone(),
            module: path.clone(),
        }));

        let mut null_error_handler = NullErrorHandler;

        // Require::Navigator navigator(navigationContext, nullErrorHandler);
        let mut navigator = Navigator::new(&mut navigation_context, &mut null_error_handler);

        // if (navigator.navigate(std::move(path)) != Status::Success) return std::nullopt;
        if navigator.navigate(path) != Status::Success {
            return None;
        }

        // The navigator borrows navigation_context for its lifetime; drop it before
        // reading back the (now-mutated) context.
        drop(navigator);

        // if (!navigationContext.isModulePresent()) return std::nullopt;
        if !crate::methods::file_navigation_context_is_module_present::file_navigation_context_is_module_present(
            &navigation_context,
        ) {
            return None;
        }

        // if (std::optional<std::string> identifier = navigationContext.getIdentifier())
        //     return {{*identifier}};
        if let Some(identifier) =
            crate::methods::file_navigation_context_get_identifier::file_navigation_context_get_identifier(
                &navigation_context,
            )
        {
            return Some(ModuleInfo {
                name: identifier,
                optional: false,
            });
        }

        None
    }
}
