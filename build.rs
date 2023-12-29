use std::path::PathBuf;

fn main() {
    let clj_parser_dir: PathBuf = ["tree-sitter-clojure", "src"].iter().collect();

    println!("cargo:rerun-if-changed=tree-sitter-clojure/src/parser.c");
    cc::Build::new()
        .include(&clj_parser_dir)
        .file(clj_parser_dir.join("parser.c"))
        .compile("tree-sitter-clojure");
}
