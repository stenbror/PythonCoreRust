
use crate::parser::nodes::{ ASTNode };
use crate::parser::tokens::{ Token };
use crate::parser::parser::{ PythonCoreParser };


trait Expressions {
    fn parse_expression_named_expr(&self) -> Box<ASTNode>;
    fn parse_expression_test(&self) -> Box<ASTNode>;
}

impl Expressions for PythonCoreParser {
    fn parse_expression_named_expr(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_test(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }
}
