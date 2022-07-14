
use crate::parser::trivias::{ Trivia };
use crate::parser::tokens::{ Token };

// Defining data structure and traits for tokenizing of PythonCore ////////////////////////////////

pub struct PythonCoreTokenizer {

}


trait Tokenizer {
    fn new() -> Self;
    fn advance(&self);
    fn get_symbol(&self) -> Box<Token>;
    fn get_position(&self) -> u32;
}

// Implementing functions releated to tokenizing of PythonCore ////////////////////////////////////

impl Tokenizer for PythonCoreTokenizer {
    fn new() -> PythonCoreTokenizer {
        PythonCoreTokenizer { }
    }

    fn advance(&self) {

    }

    fn get_symbol(&self) -> Box<Token> {
       Box::new(Token::Empty)
    }

    fn get_position(&self) -> u32 {
        0
    }
}