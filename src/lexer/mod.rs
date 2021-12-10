pub mod token;

/// The Lexer itself
#[derive(Debug)]
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
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect::<Vec<char>>(),
            position: 0,
            read_position: 0,
            ch: '#',
        }
    }

    /// Reads next char, updates the positions
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '#';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    pub fn skip_whitespace(&mut self) {
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
            '+' => token::Token::PLUS(self.ch),
            '-' => token::Token::MINUS(self.ch),
            '*' => token::Token::TIMES(self.ch),
            '/' => token::Token::SLASH(self.ch),
            '>' => token::Token::GT(self.ch),
            '<' => token::Token::LT(self.ch),
            '=' => token::Token::EQUAL(self.ch),
            '!' => token::Token::NOT(self.ch),
            '(' => token::Token::LPAREN(self.ch),
            ')' => token::Token::RPAREN(self.ch),
            ';' => token::Token::SEMICOLON(self.ch),
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
                    let ident: Vec<char> = read_number(self);
                    return token::Token::INT(ident);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let input = "";
        let mut l = Lexer::new(input);
        
        l.read_char();
        
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
        
        l.read_char();
        
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
            token::Token::EQUAL('='),
            token::Token::LT('<'),
            token::Token::GT('>'),
            token::Token::NOT('!'),
            token::Token::SLASH('/'),
            token::Token::TIMES('*'),
            token::Token::PLUS('+'),
        ]);
    }

    #[test]
    fn test_assign_to_ident() {
        let input = "A = 5";
        let mut l = Lexer::new(input);
        
        l.read_char();
        
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
            token::Token::EQUAL('='),
            token::Token::INT(vec!['5'])
        ]);
    }
}