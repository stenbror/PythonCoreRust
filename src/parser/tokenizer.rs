
use crate::parser::trivias::{ Trivia };
use crate::parser::tokens::{ Token };

// Defining data structure and traits for tokenizing of PythonCore ////////////////////////////////

struct PythonCoreTokenizer {

}


trait Tokenizer {
    fn new() -> Self;
    fn advance();
    fn get_symbol() -> Box<Token>;
    fn get_position() -> u32;
}

// Implementing functions releated to tokenizing of PythonCore ////////////////////////////////////

impl Tokenizer for PythonCoreTokenizer {
    fn new() -> PythonCoreTokenizer {
        PythonCoreTokenizer { }
    }

    fn advance() {

    }

    fn get_symbol() -> Box<Token> {
       Box::new(Token::Empty)
    }

    fn get_position() -> u32 {
        0
    }
}