
use crate::parser::trivias::{ Trivia };
use crate::parser::tokens::{ Token };


// Defining data structure and traits for tokenizing of PythonCore ////////////////////////////////


pub struct PythonCoreTokenizer {

}


// Implementing functions releated to tokenizing of PythonCore ////////////////////////////////////

impl PythonCoreTokenizer {
    fn new() -> PythonCoreTokenizer {
        PythonCoreTokenizer {

        }
    }

    pub fn advance(&self) -> () {}

    pub fn get_symbol(&self) -> Box<Token> {
        Box::new(Token::Empty)
    }

    pub fn get_position(&self) -> u32 {
        0
    }

    /// This method checks for reserved keywords or atom name literal and provides token with position and trivias collected in fron ot token
    fn is_reserved_keyword(start_pos: &u32, end_pos: &u32, trivia: Option<Box<Vec<Box<Trivia>>>>, buffer: &str) -> Token {
        match &*buffer {
            "False" => Token::PyFalse(*start_pos, *end_pos, trivia),
            "None" => Token::PyNone(*start_pos, *end_pos, trivia),
            "True" => Token::PyTrue(*start_pos, *end_pos, trivia),
            "and" => Token::PyAnd(*start_pos, *end_pos, trivia),
            "as" => Token::PyAs(*start_pos, *end_pos, trivia),
            _ => Token::Empty
        }
    }
}
