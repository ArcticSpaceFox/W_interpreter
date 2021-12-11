/// A Token is a string with an assigned and thus identified meaning. It is 
/// structured as a pair consisting of a token name and an optional token value. 
/// 
/// ```rust
/// use winter::lexer::token::Token;
/// 
/// let t = Token::INT(123);
/// ```
#[derive(PartialEq, Debug)]
pub enum Token {
    // =========
    // Types
    INT(u64),
    TRUE,
    FALSE,
    // =========
    // Expressions
    PLUS,
    MINUS,
    TIMES,
    SLASH,
    GT,
    LT,
    EQUAL,
    NOT,
    // =========
    IDENT(Vec<char>),
    SEMICOLON,
    IF,
    ELSE,
    WHILE,
    DO,
    END,
    ABORT,
    LPAREN,
    RPAREN,
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