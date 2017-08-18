use lexer::Lexer;
use source::*;

pub fn run(source_dir: &str) {
    let mut source_manager = SourceManager::new();
    let file = &[source_dir, "/hello_world.ja"].join("");
    source_manager.add_source_file(file);

    for file in source_manager.source_files {
        let mut lexer = Lexer::new(file);
        let tokens = lexer.run();
    }
}
