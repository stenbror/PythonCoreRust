
use crate::parser::trivias::{ Trivia };
use crate::parser::tokens::{ Token };

// Defining data structure and traits for tokenizing of PythonCore ////////////////////////////////

pub struct PythonCoreTokenizer {

}



// Implementing functions releated to tokenizing of PythonCore ////////////////////////////////////

impl PythonCoreTokenizer {
    fn new() -> PythonCoreTokenizer {
        PythonCoreTokenizer { }
    }

    pub fn advance(&self) {

    }

    pub fn get_symbol(&self) -> Box<Token> {
       Box::new(Token::Empty)
    }

    pub fn get_position(&self) -> u32 {
        0
    }
}