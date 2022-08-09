use crate::parser::trivias::Trivia;
use crate::result_parser::source_buffer::SourceBuffer;
use crate::result_parser::source_buffer::SourceBufferFunctionality;
use crate::Token;

pub struct PythonCoreTokenizer {
    source_buffer: Box<SourceBuffer>,
    token_start_position: u32
}


trait Tokenizer {
    fn new(buffer: String) -> PythonCoreTokenizer;
    fn get_symbol(&mut self) -> Result<Box<Token>, String>;
    fn get_position(&self) -> u32;
}


impl Tokenizer for PythonCoreTokenizer {
    fn new(buffer: String) -> PythonCoreTokenizer {
        PythonCoreTokenizer {
            source_buffer: Box::new( SourceBuffer::new(buffer) ),
            token_start_position: 0u32
        }
    }

    fn get_symbol(&mut self) -> Result<Box<Token>, String> {

        let trivia_collector : Box<Vec<Box<Trivia>>> = Box::new( Vec::new() );

        self.token_start_position = self.source_buffer.get_position(); // Saves starts of current token symbol.

        match self.source_buffer.peek_three_chars() {
            ( '*', '*', '=' ) => {
                for i in 1 ..= 3 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyPowerAssign(self.token_start_position, self.source_buffer.get_position(),
                                                  match trivia_collector.len() { 0 => None, _ => Some( trivia_collector ) } ) ))
            },
            ( '*', '*', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyPower(self.token_start_position, self.source_buffer.get_position(),
                                                  match trivia_collector.len() { 0 => None, _ => Some( trivia_collector ) } ) ))
            },
            ( '*', '=', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyMulAssign(self.token_start_position, self.source_buffer.get_position(),
                                            match trivia_collector.len() { 0 => None, _ => Some( trivia_collector ) } ) ))
            },
            ( '*', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyMul(self.token_start_position, self.source_buffer.get_position(),
                                            match trivia_collector.len() { 0 => None, _ => Some( trivia_collector ) } ) ))
            },
            _ => {
                let txt = format!( "Lexical error at ({}), found '{}'", self.source_buffer.get_position(), self.source_buffer.get_char() );
                Err(txt)
            }
        }

    }

    fn get_position(&self) -> u32 {
        0u32
    }
}