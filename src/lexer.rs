use source::*;
use token::Token;

pub struct Lexer{
    file: SourceFile
}

impl Lexer {
    pub fn new(file: SourceFile) -> Lexer {
        Lexer { file: file }
    }

    pub fn run(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut current = String::new();

        self.skip_whitespace();
        
        while !self.file.eof() {
            current.push(self.file.get_next());

            match current.as_ref() {
                "//" => {
                    self.read_line(true);
                    current.clear();
                    self.skip_whitespace();
                }
                _ => { }
            }
        }

        self.file.throw_error("abstract error", "some type of way");

        println!("{}", current);

        tokens
    }

    fn skip_whitespace(&mut self) {
        while self.file.peek().is_whitespace() {
            self.file.get_next();
        }
    }

    fn read_line(&mut self, skip_chars: bool) -> String {
        let mut token = String::new();
        let mut c = self.file.get_next();

        while c != '\n' {
            if !skip_chars {
                token.push(c);
            }

            c = self.file.get_next();
        }

        token
    }
}