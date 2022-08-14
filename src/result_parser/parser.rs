use crate::parser::tokens::{ Token };
use crate::result_parser::tokenizer::{PythonCoreTokenizer, Tokenizer};

pub struct PythonCoreParser {
    pub lexer: Box<PythonCoreTokenizer>,
    pub(crate) symbol: Result<Box<Token>, String>
}

pub trait Parser {
    fn new(lexer: Box<PythonCoreTokenizer>) -> Self;
    fn advance(&mut self) -> ();
}


impl Parser for PythonCoreParser {
    fn new(mut lexer: Box<PythonCoreTokenizer>) -> PythonCoreParser {
        PythonCoreParser {
            lexer,
            symbol: Err("Token not advanced yet! ".to_string())
        }
    }

    fn advance(&mut self) -> () {
        self.symbol = self.lexer.get_symbol()
    }

}