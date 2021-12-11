use self::token::Token;

pub mod token;

/// The Lexer will parse a String into Tokens
/// 
/// Can be used as follows:
/// ```rust
/// let mut l = winter::lexer::Lexer::new("123");
/// println!("{:?}", l.next())
/// ```
#[derive(Debug, Clone)]
pub struct Lexer {
    /// Source Code
    input: Vec<char>,
    /// Reading position
    pub position: usize,
    /// Current moving reading position
    pub read_position: usize,
    /// Current read char
    pub ch: char,
}

#[allow(unused)]
impl Lexer {
    /// Initializes a new Lexer instance with given input
    /// 
    /// ```rust
    /// let l = winter::lexer::Lexer::new("1");
    /// ```
    pub fn new(input: &str) -> Self {
        let mut new = Self {
            input: input.chars().collect::<Vec<char>>(),
            position: 0,
            read_position: 0,
            ch: '#',
        };
        new.read_char();
        new
    }

    /// Reads next char, updates the positions
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '#';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    /// Skips unneeded whitespace, we just want Tokens
    fn skip_whitespace(&mut self) {
        let ch = self.ch;
        if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            self.read_char();
        }
    }

    /// Match the read character and assign approproate type
    pub fn next_token(&mut self) -> token::Token {
        let read_identifier = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && l.ch.is_alphabetic() {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let read_number = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && l.ch.is_numeric() {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        self.skip_whitespace();
        let tok = match self.ch {
            '+' => token::Token::PLUS,
            '-' => token::Token::MINUS,
            '*' => token::Token::TIMES,
            '/' => token::Token::SLASH,
            '>' => token::Token::GT,
            '<' => token::Token::LT,
            '=' => token::Token::EQUAL,
            '!' => token::Token::NOT,
            '(' => token::Token::LPAREN,
            ')' => token::Token::RPAREN,
            ';' => token::Token::SEMICOLON,
            '#' => token::Token::EOF,
            _ => {
                if self.ch.is_alphabetic() {
                    let ident: Vec<char> = read_identifier(self);
                    match token::get_keyword_token(&ident) {
                        Ok(keywork_token) => {
                            return keywork_token;
                        },
                        Err(_err) => {
                            return token::Token::IDENT(ident);
                        }
                    }
                } else if self.ch.is_numeric() {
                    let ident = read_number(self).iter()
                        .collect::<String>();
                    return token::Token::INT(u64::from_str_radix(
                        ident.as_str(), 
                        10
                    ).expect("Was not a number"));
                } 
                else {
                    return token::Token::ILLEGAL
                }
            }
        };
        self.read_char();
        
        tok
    }
}

/// Implement the Iterator trait
/// 
/// ```rust
/// let mut l = winter::lexer::Lexer::new("A = 1");
/// 
/// assert_eq!(Some(winter::lexer::token::Token::IDENT(vec!['A'])), l.next());
/// ```
impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next_token();
        if next == token::Token::EOF {
            return None;
        }
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let input = "";
        let mut l = Lexer::new(input);
        
        let mut tokens = vec![];
        loop {
            let token = l.next_token();
            if token == token::Token::EOF {
                break
            }
            println!("{:?}", token);
            tokens.push(token);
        }

        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_operators() {
        let input = "=<>!/*+";
        let mut l = Lexer::new(input);
        
        let mut tokens = vec![];
        loop {
            let token = l.next_token();
            if token == token::Token::EOF {
                break
            }
            println!("{:?}", token);
            tokens.push(token);
        }

        assert_eq!(tokens, vec![
            token::Token::EQUAL,
            token::Token::LT,
            token::Token::GT,
            token::Token::NOT,
            token::Token::SLASH,
            token::Token::TIMES,
            token::Token::PLUS,
        ]);
    }

    #[test]
    fn test_assign_to_ident() {
        let input = "A = 5";
        let mut l = Lexer::new(input);
        
        let mut tokens = vec![];
        loop {
            let token = l.next_token();
            if token == token::Token::EOF {
                break
            }
            println!("{:?}", token);
            tokens.push(token);
        }

        assert_eq!(tokens, vec![
            token::Token::IDENT(vec!['A']),
            token::Token::EQUAL,
            token::Token::INT(5)
        ]);
    }

    #[test]
    fn test_big_number() {
        let input = "66424";
        let mut l = Lexer::new(input);
        
        assert_eq!(l.next_token(), 
            token::Token::INT(66424)
        );
    }

    #[test]
    fn test_invalid_number() {
        let input = "123_456";
        let l = Lexer::new(input);
        
        assert_eq!(l.take(3).collect::<Vec<Token>>(), vec![
            token::Token::INT(123),
            token::Token::ILLEGAL,
            token::Token::ILLEGAL
        ]);
    }

    #[test]
    fn test_iterator() {
        let input = "B = 1";
        let l = Lexer::new(input);
     
        let res = l.take(3).collect::<Vec<Token>>();

        assert_eq!(
            res, 
            vec![
                token::Token::IDENT(vec!['B']),
                token::Token::EQUAL,
                token::Token::INT(1)
            ],
            "The Lexer should be working when used via the Iterator concepts"
        );
    }

    #[test]
    fn test_if_none_when_empty() {
        let mut l = Lexer::new("");
        assert_eq!(
            None,
            l.next(),
            "Tests if Iterator returns None when nothing is left"
        )
    }

    #[test]
    fn test_long_identifier() {
        let mut l = Lexer::new("Test = 1");

        assert_eq!(
            Some(token::Token::IDENT(vec!['T', 'e', 's', 't'])), 
            l.next(),
            "Long identifiers should still be working"
        );
    }
}