use std::io::{self};
pub mod lexer;
pub mod input_buffer;
pub use crate::lexer::Lexer;

fn main() -> io::Result<()>{
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).expect("Failed to read line");
    file_path.remove(file_path.len()-1);
    Ok(())
}
