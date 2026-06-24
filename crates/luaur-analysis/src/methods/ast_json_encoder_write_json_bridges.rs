//! Source: `Analysis/src/AstJsonEncoder.cpp:89-1052` (hand-ported)
//! C++ resolves `write(propName, value)` by overload on the static type of
//! `value`; Rust resolves it through `WriteJson`. One impl per C++ overload,
//! each delegating to the inherent method holding that overload's body.
//! Base-node pointers go through `write(AstNode*) = node->visit(this)`
//! (AstJsonEncoder.cpp:271); concrete-node pointers bind their concrete
//! overload exactly as C++ overload resolution does.
use crate::methods::ast_json_encoder_write_primitives::WriteJson;
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_attr::AstAttr;
use luaur_ast::records::ast_declared_extern_type_property::AstDeclaredExternTypeProperty;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_binary::AstExprBinary_Op;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_expr_table::{Item, ItemKind};
use luaur_ast::records::ast_expr_unary::AstExprUnaryOp;
use luaur_ast::records::ast_generic_type::AstGenericType;
use luaur_ast::records::ast_generic_type_pack::AstGenericTypePack;
use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_table_indexer::AstTableIndexer;
use luaur_ast::records::ast_table_prop::AstTableProp;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_list::AstTypeList;
use luaur_ast::records::ast_type_or_pack::AstTypeOrPack;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;
use luaur_ast::type_aliases::ast_argument_name::AstArgumentName;

// write(AstNode*) and the implicit base-pointer conversions: virtual dispatch.
impl WriteJson for *mut AstNode {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_node(*self);
    }
}
impl WriteJson for *mut AstExpr {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_node(*self as *mut AstNode);
    }
}
impl WriteJson for *mut AstStat {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_node(*self as *mut AstNode);
    }
}
impl WriteJson for *mut AstType {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_node(*self as *mut AstNode);
    }
}
impl WriteJson for *mut AstTypePack {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_node(*self as *mut AstNode);
    }
}
// Concrete static types bind their exact overload (no virtual call in C++).
impl WriteJson for *mut AstStatBlock {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_stat_block(*self);
    }
}
impl WriteJson for *mut AstExprFunction {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_expr_function(*self);
    }
}
impl WriteJson for *mut AstLocal {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_local(*self);
    }
}
impl WriteJson for *mut AstAttr {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_attr(*self);
    }
}
impl WriteJson for *mut AstGenericType {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_generic_type(*self);
    }
}
impl WriteJson for *mut AstGenericTypePack {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_generic_type_pack(*self);
    }
}
impl WriteJson for *mut AstTableIndexer {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_table_indexer(*self);
    }
}
impl WriteJson for Location {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_location(self);
    }
}
impl WriteJson for Position {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_position(self);
    }
}
// AstArgumentName = std::pair<AstName, Location>
impl WriteJson for AstArgumentName {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_argument_name(*self);
    }
}
impl WriteJson for AstTypeList {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_type_list(self);
    }
}
impl WriteJson for Item {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_expr_table_item(self);
    }
}
impl WriteJson for ItemKind {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_expr_table_item_kind(*self);
    }
}
impl WriteJson for AstExprUnaryOp {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_expr_unary_op(*self);
    }
}
impl WriteJson for AstExprBinary_Op {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_expr_binary_op(*self);
    }
}
impl WriteJson for AstTypeOrPack {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_type_or_pack(self);
    }
}
impl WriteJson for AstTableProp {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_table_prop(self);
    }
}
impl WriteJson for AstDeclaredExternTypeProperty {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_declared_extern_type_property(self);
    }
}
// NB: no WriteJson for c_char itself -- that would make AstArray<c_char>
// overlap the generic AstArray<T: WriteJson> impl. The one char-typed field
// (AstExprIndexName::op) calls `write_c_char` explicitly instead.
// write(AstArray<char>): the chars as one string (AstJsonEncoder.cpp:387).
impl WriteJson for AstArray<core::ffi::c_char> {
    fn write_json(&self, enc: &mut AstJsonEncoder) {
        enc.write_ast_array_c_char(*self);
    }
}
