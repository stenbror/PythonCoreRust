
use crate::result_parser::nodes::{ ASTNode };
use crate::result_parser::tokens::{ Token };
use crate::result_parser::parser::{ Parser, PythonCoreParser };


pub trait Patterns {
    fn parse_patterns_match(&self) -> Result<Box<ASTNode>, String>;
}


impl Patterns for PythonCoreParser {
    fn parse_patterns_match(&self) -> Result<Box<ASTNode>, String> {
        todo!()
    }
}


// UnitTests for patterns rules ///////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{ASTNode, Token};
    use crate::result_parser::expressions::Expressions;
    use crate::result_parser::tokenizer::{PythonCoreTokenizer, Tokenizer};
    use crate::result_parser::trivias::Trivia;
    use crate::result_parser::parser::{Parser, PythonCoreParser};


    #[test]
    fn patterns_empty_template() {
        assert!(true)
    }

}