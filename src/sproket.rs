use lexer::Lexer;
use source::*;
use std::fs::*;
use std::path::Path;
use token::*;

pub fn run(source_dir: &str, out_dir: &str) {
    let mut source_manager = SourceManager::new();
    let file = &[source_dir, "/hello_world.spr"].join("");
    source_manager.add_source_file(file);

    for file in source_manager.source_files {
        let mut lexer = Lexer::new(file);
        let tokens = lexer.run();
    }

    let out_path = Path::new(out_dir);
    create_dir(out_path);
}
