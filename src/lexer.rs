use source::*;
use token::*;

pub struct Lexer{
    file: SourceFile
}

impl Lexer {
    pub fn new(file: SourceFile) -> Lexer {
        Lexer { file: file }
    }

    pub fn run(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        let mut current = String::new();

        self.skip_whitespace();
        
        while !self.file.eof() {
            current.push(self.file.get_next());

            println!("{}", current);
            match current.as_ref() {
                "//" => {
                    self.read_line(true, false);
                    current.clear();
                    self.skip_whitespace();
                },
                _ => { 
                    if !self.file.eof() {
                        let mut token_complete = self.file.peek() == ' ';

                        match self.file.peek() {
                            '(' => {
                                self.file.get_next();
                                let new_call = self.create_function_call(current.as_str());
                                tokens.push(new_call);
                                self.read_line(true, true);
                                token_complete = true;
                            },
                            _ => { 
                                if token_complete {
                                    let msg = format!("Current token is \'{}\'", current);
                                    self.file.throw_error("Unexpected token found", msg.as_ref());
                                }
                            }
                        }

                        if token_complete {
                            current = String::new();
                        }
                    } else {
                        println!("Found token [{}]", current);
                        self.file.throw_error("Unexpected end of file", "");
                    }
                }
            }
        }

        // development debugging
        let debugging = true;
        if debugging {
            for t in &tokens {
                println!("{}", t.data);
            }
        }
        
        tokens
    }

    fn skip_chars(&mut self, count: usize) {
        let mut i = 0;

        while i < count {
            self.file.get_next();
            i += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.file.eof() && self.file.peek().is_whitespace() {
            self.file.get_next();
        }
    }

    fn read_line(&mut self, skip_chars: bool, error_on_find: bool) -> String {
        let mut data = String::new();

        loop {
            if !self.file.eof() {
                let c = self.file.get_next();

                if c == '\n' {
                    break;
                } else if error_on_find && !c.is_whitespace() {
                    self.file.throw_error("Found unexpected character", "");
                }

                if !skip_chars {
                    data.push(c);
                }
            }
        }

        data
    }

    fn read_until(&mut self, term_char: char, consume_term: bool) -> String {
        let mut data = String::new();
        let mut c: char;

        while self.file.peek() != term_char {
            c = self.file.get_next();
            data.push(c);
        }

        if consume_term { 
            self.file.get_next(); 
        }

        data
    }

    fn skip_until(&mut self, term_char: char, consume_term: bool) {
        self.read_until(term_char, consume_term);
    }

    fn create_function_call(&mut self, func: &str) -> Token{
        let mut params = Vec::new();
        let mut string_started = false;
        let mut current = String::new();

        while !self.file.eof() && self.file.peek() != ')' {
            match self.file.peek() {
                ',' => {
                    if string_started {
                        self.file.throw_error("Expected end of string", "");
                    }
                    params.push(current);
                    current = String::new();
                },
                '\"' => {
                    string_started = !string_started;
                    current.push(self.file.get_next());
                },
                _ => {
                    current.push(self.file.get_next());
                }
            }
        }

        if !self.file.eof() {
            self.file.get_next();

            if current != "" && string_started {
                self.file.throw_error("Expected end of string", "");
            } else {
                params.push(current);
            }
        } else {
            self.file.throw_error("Unexpected end of file", "");
        }

        let name = func.trim().to_string();
        //let new_call = Call::new(TokenType::Call, name, params);
        //Box::new(new_call)
        Token::new([ "CALL", func, &params.join(",") ].join(":"))
    }
}