use crate::functions::fail::fail;
use crate::type_aliases::error::Error;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name_table::AstNameTable;
use luaur_ast::records::lexeme::Type;
use luaur_ast::records::lexer::Lexer;
use luaur_ast::records::position::Position;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

fn next(lexer: &mut Lexer) {
    lexer.next();

    while lexer.current().r#type == Type::FloorDiv {
        lexer.nextline();
    }
}

pub(crate) fn parse_json<Action>(contents: &str, mut action: Action) -> Error
where
    Action: FnMut(&Vec<String>, String) -> Error,
{
    let mut allocator = Allocator::allocator();
    let mut names = AstNameTable::new(&mut allocator);
    let mut lexer = Lexer::new(
        contents.as_ptr() as *const core::ffi::c_char,
        contents.len(),
        &mut names,
        Position::new(0, 0),
    );
    next(&mut lexer);

    let mut keys: Vec<String> = Vec::new();
    let mut array_top = false;

    if lexer.current().r#type != Type('{' as i32) {
        return fail(&lexer, "'{'");
    }
    next(&mut lexer);

    loop {
        if array_top {
            if lexer.current().r#type == Type(']' as i32) {
                next(&mut lexer);
                array_top = false;

                LUAU_ASSERT!(!keys.is_empty());
                keys.pop();

                if lexer.current().r#type == Type(',' as i32) {
                    next(&mut lexer);
                } else if lexer.current().r#type != Type('}' as i32) {
                    return fail(&lexer, "',' or '}'");
                }
            } else if lexer.current().r#type == Type::QuotedString {
                let lex = lexer.current();
                let value = unsafe {
                    let p = lex.data.data as *const u8;
                    String::from_utf8_lossy(core::slice::from_raw_parts(
                        p,
                        lex.get_length() as usize,
                    ))
                    .into_owned()
                };
                next(&mut lexer);

                if let Some(err) = action(&keys, value) {
                    return Some(err);
                }

                if lexer.current().r#type == Type(',' as i32) {
                    next(&mut lexer);
                } else if lexer.current().r#type != Type(']' as i32) {
                    return fail(&lexer, "',' or ']'");
                }
            } else {
                return fail(&lexer, "array element or ']'");
            }
        } else {
            if lexer.current().r#type == Type('}' as i32) {
                next(&mut lexer);

                if keys.is_empty() {
                    if lexer.current().r#type != Type::Eof {
                        return fail(&lexer, "end of file");
                    }
                    return None;
                } else {
                    keys.pop();
                }

                if lexer.current().r#type == Type(',' as i32) {
                    next(&mut lexer);
                } else if lexer.current().r#type != Type('}' as i32) {
                    return fail(&lexer, "',' or '}'");
                }
            } else if lexer.current().r#type == Type::QuotedString {
                let lex = lexer.current();
                let key = unsafe {
                    let p = lex.data.data as *const u8;
                    String::from_utf8_lossy(core::slice::from_raw_parts(
                        p,
                        lex.get_length() as usize,
                    ))
                    .into_owned()
                };
                next(&mut lexer);

                keys.push(key);

                if lexer.current().r#type != Type(':' as i32) {
                    return fail(&lexer, "':'");
                }
                next(&mut lexer);

                if lexer.current().r#type == Type('{' as i32)
                    || lexer.current().r#type == Type('[' as i32)
                {
                    array_top = lexer.current().r#type == Type('[' as i32);
                    next(&mut lexer);
                } else if lexer.current().r#type == Type::QuotedString
                    || lexer.current().r#type == Type::ReservedTrue
                    || lexer.current().r#type == Type::ReservedFalse
                {
                    let lex = lexer.current();
                    let value = if lex.r#type == Type::QuotedString {
                        unsafe {
                            let p = lex.data.data as *const u8;
                            String::from_utf8_lossy(core::slice::from_raw_parts(
                                p,
                                lex.get_length() as usize,
                            ))
                            .into_owned()
                        }
                    } else if lex.r#type == Type::ReservedTrue {
                        "true".to_string()
                    } else {
                        "false".to_string()
                    };
                    next(&mut lexer);

                    if let Some(err) = action(&keys, value) {
                        return Some(err);
                    }

                    keys.pop();

                    if lexer.current().r#type == Type(',' as i32) {
                        next(&mut lexer);
                    } else if lexer.current().r#type != Type('}' as i32) {
                        return fail(&lexer, "',' or '}'");
                    }
                } else {
                    return fail(&lexer, "field value");
                }
            } else {
                return fail(&lexer, "field key");
            }
        }
    }
}
