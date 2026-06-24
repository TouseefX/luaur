use crate::records::reducer::Reducer;
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::{c_char, c_void};
use luaur_ast::functions::pretty_print_with_types_pretty_printer::pretty_print_with_types_ast_stat_block_cst_node_map;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use std::fs::File;
use std::io::Write;
use std::process;

impl Reducer {
    pub fn write_temp_script(&mut self, minify: bool) {
        let mut source = pretty_print_with_types_ast_stat_block_cst_node_map(
            unsafe { &mut *self.root },
            self.cst_node_map.clone(),
        );

        if minify {
            let mut pos = 0;
            loop {
                if let Some(found_pos) = source[pos..].find("\n\n") {
                    source.remove(pos + found_pos);
                    pos += found_pos;
                } else {
                    break;
                }
            }
        }

        let file = File::create(&self.script_name);
        let mut f = match file {
            Ok(f) => f,
            Err(_) => {
                println!("Unable to open temp script to {}", self.script_name);
                process::exit(2);
            }
        };

        for comment in &self.parse_result.hotcomments {
            if let Err(_) = writeln!(f, "--!{}", comment.content) {
                println!("Unable to write to temp script {}", self.script_name);
                process::exit(3);
            }
        }

        if let Err(_) = f.write_all(source.as_bytes()) {
            println!("Unable to write to temp script {}", self.script_name);
            process::exit(3);
        }
    }
}
