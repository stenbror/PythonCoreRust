
use crate::parser::nodes::{ ASTNode };
use crate::parser::tokens::{ Token };
use crate::parser::parser::{ PythonCoreParser };
use crate::parser::patterns::{ Patterns };
use crate::parser::expressions::{ Expressions };
use std::vec;


pub trait Blocks {
    fn parse_blocks_eval_input(&self) -> Box<ASTNode>;
    fn parse_blocks_file_input(&self) -> Box<ASTNode>;
    fn parse_blocks_single_input(&self) -> Box<ASTNode>;
}


impl Blocks for PythonCoreParser {
    fn parse_blocks_eval_input(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_file_input(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_single_input(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }
}