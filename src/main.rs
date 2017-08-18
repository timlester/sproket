mod sproket;
mod lexer;
mod source;
mod token;

fn main() {
    let test_dir = "./examples";
    sproket::run(test_dir);
}
