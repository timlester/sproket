mod sproket;
mod lexer;
mod source;
mod token;

fn main() {
    let test_dir = "./examples";
    let out_dir = "./target/test_build";
    sproket::run(test_dir, out_dir);
}
