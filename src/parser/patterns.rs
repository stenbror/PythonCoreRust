
use crate::parser::nodes::{ ASTNode };
use crate::parser::tokens::{ Token };
use crate::parser::parser::{ PythonCoreParser };


pub trait Patterns {
    fn parse_patterns_match(&self) -> Box<ASTNode>;
}

impl Patterns for PythonCoreParser {
    fn parse_patterns_match(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }
}