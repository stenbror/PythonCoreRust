
use crate::parser::tokens::{ Token };
use crate::parser::tokenizer::{ PythonCoreTokenizer };

pub struct PythonCoreParser {
    lexer: Box<PythonCoreTokenizer>
}

trait Parser {
    fn new(lexer: Box<PythonCoreTokenizer>) -> Self;
}


impl Parser for PythonCoreParser {
    fn new(lexer: Box<PythonCoreTokenizer>) -> PythonCoreParser {
        PythonCoreParser { lexer: lexer }
    }
}