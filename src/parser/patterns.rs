
use crate::parser::nodes::{ ASTNode };
use crate::parser::parser::{ PythonCoreParser };


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



    #[test]
    fn patterns_empty_template() {
        assert!(true)
    }

}