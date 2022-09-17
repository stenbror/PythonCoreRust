
use crate::result_parser::tokens::{ Token };
use crate::parser::tokenizer::{ PythonCoreTokenizer };
use crate::parser::expressions;
use crate::parser::statements;
use crate::parser::patterns;
use crate::parser::blocks;


pub struct PythonCoreParser {
    pub lexer: Box<PythonCoreTokenizer>
}

pub trait Parser {
    fn new(lexer: Box<PythonCoreTokenizer>) -> Self;
}


impl Parser for PythonCoreParser {
    fn new(lexer: Box<PythonCoreTokenizer>) -> PythonCoreParser {
        PythonCoreParser { lexer }
    }
}