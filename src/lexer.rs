use core::fmt;
pub use crate::input_buffer::InputBuffer;
#[derive(Debug)]
enum TokenType {
    END_OF_FILE,
    MAIN,
    PROC,
    ENDPROC,
    INPUT,
    OUTPUT,
    DO,
    EQUAL,
    NUM,
    ID,
    SEMICOLON,
    PLUS,
    MINUS,
    MULT,
    DIV,
    ERROR
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

const KEYWORDS_COUNT: u8 = 7;

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
    temp_token: Token,
    input_buffer: InputBuffer
}

impl Lexer {
    fn get_token_main() -> Token {
        let mut c: char;

        
    }

    fn skip_space(&mut self) -> bool {
        let mut c = self.input_buffer.get_char();
        let space_encountered = false;
    }
}
