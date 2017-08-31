
pub struct Token {
    pub data: String,
    pub child_tokens: Vec<Token>
}

impl Token {
    pub fn new(data: String) -> Token {
        Token { data: data, child_tokens: vec![] }
    }
}

/*pub trait Token {
    fn get_type(&self) -> &TokenType;
    fn get_data(&self) -> &String;
    fn add_child(&mut self, token: Box<Token>);
}

pub struct Program {
    token_type: TokenType,
    data: String,
    child_tokens: Vec<Box<Token>>
}

impl Program {
    pub fn new(token_type: TokenType, data: String) -> Program {
        Program { token_type: token_type, data: data, child_tokens: vec![] }
    }
}

impl Token for Program { 
    fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    fn get_data(&self) -> &String {
        &self.data
    }

    fn add_child(&mut self, token: Box<Token>) {
        self.child_tokens.push(token);
    }
}

pub struct Call {
    token_type: TokenType,
    data: String,
    args: Vec<String>
}

impl Call {
   pub fn new(token_type: TokenType, data: String, args: Vec<String>) -> Call {
        Call { token_type: token_type, data: data, args: args }
    }
}

impl Token for Call {
    fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    fn get_data(&self) -> &String {
        &self.data
    }

    fn add_child(&mut self, token: Box<Token>) { unimplemented!(); }
}

pub enum TokenType {
    Call,
    Expression,
    Function,
    Program,
    Statement,
    Variable
}*/
