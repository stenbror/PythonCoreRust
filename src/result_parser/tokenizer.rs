
use crate::result_parser::source_buffer::SourceBuffer;
use crate::result_parser::source_buffer::SourceBufferFunctionality;
use crate::Token;

pub struct PythonCoreTokenizer {
    source_buffer: Box<SourceBuffer>
}


trait Tokenizer {
    fn new(buffer: String) -> PythonCoreTokenizer;
    fn get_symbol() -> Result<Token, String>;
    fn get_position() -> u32;
}


impl Tokenizer for PythonCoreTokenizer {
    fn new(buffer: String) -> PythonCoreTokenizer {
        PythonCoreTokenizer {
            source_buffer: Box::new( SourceBuffer::new(buffer) )
        }
    }

    fn get_symbol() -> Result<Token, String> {
        Ok(Token::Empty)
    }

    fn get_position() -> u32 {
        0u32
    }
}