
use crate::parser::nodes::{ ASTNode };
use crate::parser::tokens::{ Token };
use crate::parser::parser::{ PythonCoreParser };
use crate::parser::expressions::{ Expressions };

trait Functions {
    fn parse_functions_func_type(&self) -> Box<ASTNode>;
    fn parse_functions_type_list(&self) -> Box<ASTNode>;
}

impl Functions for PythonCoreParser {
    fn parse_functions_func_type(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyLeftParen( .. ) => {
                let symbol1 = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let mut left_node : Option<Box<ASTNode>> = None;
                match &*self.lexer.get_symbol() {
                    Token::PyRightParen( .. ) => {},
                    _ => {
                        left_node = Some( self.parse_functions_type_list() );
                    }
                }
                match &*self.lexer.get_symbol() {
                    Token::PyRightParen( .. ) => {
                        let symbol2 = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        match &*self.lexer.get_symbol() {
                            Token::PyArrow( .. ) => {
                                let symbol3 = self.lexer.get_symbol();
                                let _ = &self.lexer.advance();
                                let right_node = self.parse_expression_test();
                                let end_pos = &self.lexer.get_position();
                                Box::new( ASTNode::FuncType(*start_pos, *end_pos, symbol1, left_node, symbol2, symbol3, right_node) )
                            },
                            _ => {
                                panic!("Syntax Error at {} - Expected '->' in function!", &self.lexer.get_position())
                            }
                        }
                    },
                    _ => {
                        panic!("Syntax Error at {} - Expected ')' in function!", &self.lexer.get_position())
                    }
                }
            },
            _ => {
                panic!("Syntax Error at {} - Expected '(' in function!", &self.lexer.get_position())
            }
        }
    }

    fn parse_functions_type_list(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }
}