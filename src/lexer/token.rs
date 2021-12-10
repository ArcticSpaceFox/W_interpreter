/// A Token is a string with an assigned and thus identified meaning. It is 
/// structured as a pair consisting of a token name and an optional token value. 
#[derive(PartialEq, Debug)]
pub enum Token {
    // =========
    // Types
    INT(Vec<char>),
    TRUE,
    FALSE,
    // =========
    // Expressions
    PLUS(char),
    MINUS(char),
    TIMES(char),
    SLASH(char),
    GT(char),
    LT(char),
    EQUAL(char),
    NOT(char),
    // =========
    IDENT(Vec<char>),
    SEMICOLON(char),
    IF,
    ELSE,
    WHILE,
    DO,
    END,
    ABORT,
    LPAREN(char),
    RPAREN(char),
    // =========
    ILLEGAL,
    EOF,
}

pub fn get_keyword_token(ident: &Vec<char>) -> Result<Token, String> {
    let identifier: String = ident.into_iter().collect();
    match &identifier[..] {
        "true" => Ok(Token::TRUE),
        "false" => Ok(Token::FALSE),
        "if" => Ok(Token::IF),
        "else" => Ok(Token::ELSE),
        "while" => Ok(Token::WHILE),
        "do" => Ok(Token::DO),
        "end" => Ok(Token::END),
        "abort" => Ok(Token::ABORT),
        _ => Err(String::from("Not a keyword"))
    }
}