
pub struct Token {
    token_type: TokenType,
    data: String,
    sub_tokens: Vec<Token>
}

impl Token {
    pub fn new(token_type: TokenType, data: String) -> Token {
        Token { token_type: token_type, data: data, sub_tokens: vec![] }
    }

    pub fn add_sub_token(&mut self, token: Token) {
        self.sub_tokens.push(token);
    }
}

pub enum TokenType {
    Expression,
    Function,
    Operator,
    Program,
    Statement,
    Variable
}
