
use crate::parser::nodes::{ ASTNode };
use crate::parser::tokens::{ Token };
use crate::parser::parser::{Parser, PythonCoreParser };


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
    use crate::parser::expressions::Expressions;
    use crate::parser::tokenizer::{PythonCoreTokenizer, Tokenizer};
    use crate::parser::trivias::Trivia;
    use crate::parser::parser::{Parser, PythonCoreParser};


    #[test]
    fn patterns_empty_template() {
        assert!(true)
    }

}