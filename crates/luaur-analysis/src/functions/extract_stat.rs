extern crate alloc;

use crate::functions::is_identifier::is_identifier;
use alloc::vec::Vec;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_error::AstStatError;
use luaur_ast::rtti::AstNodeClass;

pub fn extract_stat<T: AstNodeClass>(ancestry: &Vec<*mut AstNode>) -> *mut T {
    let node = if ancestry.len() >= 1 {
        ancestry[ancestry.len() - 1]
    } else {
        core::ptr::null_mut()
    };

    if node.is_null() {
        return core::ptr::null_mut();
    }

    let t = unsafe { (*node).as_item_mut::<T>() };
    if !t.is_null() {
        return t;
    }

    let parent = if ancestry.len() >= 2 {
        ancestry[ancestry.len() - 2]
    } else {
        core::ptr::null_mut()
    };

    if parent.is_null() {
        return core::ptr::null_mut();
    }

    let grand_parent = if ancestry.len() >= 3 {
        ancestry[ancestry.len() - 3]
    } else {
        core::ptr::null_mut()
    };

    let great_grand_parent = if ancestry.len() >= 4 {
        ancestry[ancestry.len() - 4]
    } else {
        core::ptr::null_mut()
    };

    if grand_parent.is_null() {
        return core::ptr::null_mut();
    }

    let t_parent = unsafe { (*parent).as_item_mut::<T>() };
    if !t_parent.is_null() && unsafe { (*grand_parent).is::<AstStatBlock>() } {
        return t_parent;
    }

    if great_grand_parent.is_null() {
        return core::ptr::null_mut();
    }

    let t_great = unsafe { (*great_grand_parent).as_item_mut::<T>() };
    if !t_great.is_null()
        && unsafe { (*grand_parent).is::<AstStatBlock>() }
        && unsafe { (*parent).is::<AstStatError>() }
        && is_identifier(node)
    {
        return t_great;
    }

    core::ptr::null_mut()
}
