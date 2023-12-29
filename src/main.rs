//! Cool stuff
//! normalize imports across project, make them all prefixed, avoid refers with long names or something like that

use std::error::Error;

use once_cell::sync::Lazy;
use parking_lot::Mutex;
use tree_sitter::{Language, Parser};

use crate::lsp_server::LspBackend;

pub mod capabilities;
mod db;
mod ide;
mod ir;
mod language;
mod lsp_server;
mod source_edit;
mod syntax;
mod text_edit;

#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

extern "C" {
    fn tree_sitter_clojure() -> Language;
}

pub static LANGUAGE: Lazy<Language> = Lazy::new(|| unsafe { tree_sitter_clojure() });

pub static PARSER: Lazy<Mutex<Parser>> = Lazy::new(|| {
    let mut parser = Parser::new();
    parser.set_language(*LANGUAGE).unwrap();
    Mutex::new(parser)
});

#[tokio::main]
async fn main() -> Result<(), &'static dyn Error> {
    // other();

    LspBackend::listen().await;
    Ok(())
}

fn other() {
    let stmt = r"(ns myorg.required.namespace
                         (:require [my.org.other.namespace]))

                         (defn sum [x y] (+ x y))";
    let tree = PARSER.lock().parse(stmt, None).unwrap();
    println!("{:?}", tree);
    let mut tree_cursor = tree.walk();

    let abort = true;
    while abort {
        let node = tree_cursor.node();
        println!("{:?}", node.kind());
        println!("{}", node.utf8_text(stmt.as_ref()).unwrap());

        // let succ = tree_cursor.goto_next_sibling();
        let succ = tree_cursor.goto_first_child();
        if !succ {
            // let res = tree_cursor.goto_first_child();
            let res = tree_cursor.goto_next_sibling();

            if !res {
                break;
            }
        }
    }
}
