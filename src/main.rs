use std::io::{self};
pub mod input_buffer;
pub use crate::input_buffer::InputBuffer;

fn main() -> io::Result<()>{
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).expect("Failed to read line");
    file_path.remove(file_path.len()-1);
    let mut input_buffer = InputBuffer::build_buffer(file_path);
    input_buffer.print_buffer();
    Ok(())
}
