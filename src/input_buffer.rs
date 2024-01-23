use std::fs::File;
use std::io::Read;

pub struct InputBuffer {
    input_buffer: Vec<char>,
}

impl InputBuffer {
    pub fn build_buffer(file_name: String) -> InputBuffer {
        InputBuffer {
            input_buffer: InputBuffer::load_file(file_name)
        }
    }

    pub fn get_char(&mut self) -> Option<char> {
        return Some(self.input_buffer.pop().expect("Attempted to pop empty vector"));
    }

    pub fn unget_char(&mut self, c: char) -> char {
        if c == 0003 as char {
            self.input_buffer.push(c);
        }
        return c;
    }

    fn load_file(file_name: String) -> Vec<char> {
        let mut file = File::open(file_name).expect("File path not given or incorrect");
        let mut input_buffer = Vec::new();
        file.read_to_end(&mut input_buffer).expect("File could not be read");
        let mut eof_input_buffer: Vec<char> = input_buffer.into_iter().map(|x| x as char).collect();
        eof_input_buffer.push(0003 as char);
        return eof_input_buffer;
    }

    pub fn print_buffer(&mut self) {
        println!("{:?}", self.input_buffer)
    }
}
