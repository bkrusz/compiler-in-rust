use core::fmt;
use std::str::FromStr;
pub use crate::input_buffer::InputBuffer;
#[derive(Debug, Clone, PartialEq())]
enum TokenType {
    END_OF_FILE, MAIN, PROC, ENDPROC,
    INPUT, OUTPUT, DO, EQUAL, NUM, ID, 
    SEMICOLON, PLUS, MINUS, MULT, DIV, 
    ERROR
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for TokenType {
    type Err = ();

    fn from_str(s: &str) -> Result<TokenType, Self::Err> {
        match s {
            "MAIN" => Ok(TokenType::MAIN),
            "PROC" => Ok(TokenType::PROC),
            "ENDPROC" => Ok(TokenType::ENDPROC),
            "INPUT" => Ok(TokenType::INPUT),
            "OUTPUT" => Ok(TokenType::OUTPUT),
            "DO" => Ok(TokenType::DO),
            "EQUAL" => Ok(TokenType::EQUAL),
            _ => Err(())
        }
    }
}

const KEYWORDS_COUNT: u8 = 7;

#[derive(Clone)]
struct Token {
    lexeme: String,
    token_type: TokenType,
    line_number: u8,
}

impl Token {
    fn print(&mut self) {
        println!("{{{0}, {1}, {2}}}", self.lexeme, self.token_type, self.line_number);
    }
}

pub struct Lexer {
    token_list: Vec<Token>,
    line_number: u8,
    index: u8,
    input_buffer: InputBuffer
}

impl Lexer {
    fn get_token_main(&mut self) -> Token {
        let mut c: Option<char>;
        let mut temp_token: Token = Token {
            lexeme: String::from(""),
            line_number: self.line_number,
            token_type: TokenType::END_OF_FILE
        };

        self.skip_space();
        temp_token.lexeme = String::from("");
        temp_token.line_number = self.line_number;
        temp_token.token_type = TokenType::END_OF_FILE;

        if !self.input_buffer.end_of_input() {
            c = self.input_buffer.get_char();
        } else {
            return temp_token;
        }

        match c.unwrap_or_default() {
            ';' => temp_token.token_type = TokenType::SEMICOLON,
            '*' => temp_token.token_type = TokenType::MULT,
            '/' => temp_token.token_type = TokenType::DIV,
            '+' => temp_token.token_type = TokenType::PLUS,
            '-' => temp_token.token_type = TokenType::MINUS,
            '=' => temp_token.token_type = TokenType::EQUAL,
            _ => if c.unwrap_or_default().is_ascii_digit() {
                  self.input_buffer.unget_char(c.unwrap_or_default());
                    return self.scan_number();
                } else if c.unwrap_or_default().is_ascii_alphabetic() {
                    return self.scan_id_or_keyword()
                } else if self.input_buffer.end_of_input() {
                    temp_token.token_type = TokenType::END_OF_FILE;
                } else {
                    temp_token.token_type = TokenType::ERROR;
                }
        }

        return temp_token;
    }

    fn scan_number(&mut self) -> Token {
        let mut temp_token = Token {
            lexeme: String::from(""),
            line_number: self.line_number,
            token_type: TokenType::ERROR
        };
        let mut c = self.input_buffer.get_char();
        if c.unwrap_or_default().is_ascii_digit() {
            if c.unwrap() == '0' {
                temp_token.lexeme = String::from("0");
            } else {
                temp_token.lexeme = String::from("");
                while !self.input_buffer.end_of_input() && c.unwrap().is_ascii_digit() {
                    temp_token.lexeme += &String::from(c.unwrap());
                    c = self.input_buffer.get_char();
                }
                if !self.input_buffer.end_of_input() {
                    self.input_buffer.unget_char(c.unwrap());
                }
            }

            temp_token.token_type = TokenType::NUM;
            return temp_token;
        } else {
            if !self.input_buffer.end_of_input() {
                self.input_buffer.unget_char(c.unwrap());
            }

            return temp_token;
        }
    }

    fn scan_id_or_keyword(&mut self) -> Token {
        let mut temp_token = Token {
            lexeme: String::from(""),
            line_number: self.line_number,
            token_type: TokenType::ERROR
        };
        let c = self.input_buffer.get_char();

        if c.unwrap_or_default().is_ascii_alphabetic() {
            while !self.input_buffer.end_of_input() && c.unwrap_or_default().is_ascii() {
                temp_token.lexeme += &String::from(c.unwrap());
                self.input_buffer.get_char();
            }
            if !self.input_buffer.end_of_input() {
                self.input_buffer.unget_char(c.unwrap());
            }
            if TokenType::from_str(&temp_token.lexeme).unwrap() != TokenType::ID {
                temp_token.token_type = TokenType::into(temp_token.token_type);
            } else {
                temp_token.token_type = TokenType::ID;
            }
        }
        else {
            if !self.input_buffer.end_of_input() {
                self.input_buffer.unget_char(c.unwrap());
            }
        }

        return temp_token;
    }

    fn get_token(&mut self) -> Token {

    }

    fn peek(&mut self, how_far: u8) -> Token {

    }

    fn skip_space(&mut self) -> bool {
        let mut c = self.input_buffer.get_char();
        let mut space_encountered = false;

        if c.is_some_and(|c| c == '\n') { self.line_number += 1; }

        while !self.input_buffer.end_of_input() && c.unwrap().is_whitespace() {
            space_encountered = true;
            c = self.input_buffer.get_char();
            if c.is_some_and(|c| c == '\n') { self.line_number += 1; }
        }

        if !self.input_buffer.end_of_input() { self.input_buffer.unget_char(c.unwrap()); }
        
        return space_encountered;
    }
}
