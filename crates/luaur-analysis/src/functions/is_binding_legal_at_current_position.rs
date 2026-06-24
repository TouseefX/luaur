use crate::records::binding::Binding;
use crate::records::symbol::Symbol;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;

pub fn is_binding_legal_at_current_position(
    symbol: &Symbol,
    binding: &Binding,
    pos: Position,
) -> bool {
    if !symbol.local.is_null() {
        return binding.location.end < pos;
    }

    binding.location == Location::default() || !binding.location.containsClosed(pos)
}
