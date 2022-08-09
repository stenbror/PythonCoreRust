
use crate::result_parser::source_buffer::SourceBuffer;
use crate::result_parser::source_buffer::SourceBufferFunctionality;

pub struct PythonCoreTokenizer {
    source_buffer: Box<SourceBuffer>
}


trait Tokenizer {
    fn new(buffer: String) -> PythonCoreTokenizer;
}


impl Tokenizer for PythonCoreTokenizer {
    fn new(buffer: String) -> PythonCoreTokenizer {
        PythonCoreTokenizer {
            source_buffer: Box::new( SourceBuffer::new(buffer) )
        }
    }
}