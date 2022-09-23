use crate::parser::parser::ET::ExceptNone;
use crate::parser::tokens::{ Token };
use crate::parser::tokenizer::{PythonCoreTokenizer, Tokenizer};

pub enum ET { ExceptNone, ExceptMul, Except }

pub struct PythonCoreParser {
    pub lexer: Box<PythonCoreTokenizer>,
    pub(crate) symbol: Result<Box<Token>, String>,
    pub except_status: ET
}

pub trait Parser {
    fn new(lexer: Box<PythonCoreTokenizer>) -> Self;
    fn advance(&mut self) -> ();
}


impl Parser for PythonCoreParser {
    fn new(mut lexer: Box<PythonCoreTokenizer>) -> PythonCoreParser {
        PythonCoreParser {
            lexer,
            symbol: Err("Token not advanced yet! ".to_string()),
            except_status: ExceptNone
        }
    }

    fn advance(&mut self) -> () {
        self.symbol = self.lexer.get_symbol()
    }

}