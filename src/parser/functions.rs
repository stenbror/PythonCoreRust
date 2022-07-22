
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
        let start_pos = &self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        let mut mul_node : Option<Box<ASTNode>> = None;
        let mut mul_symbol : Option<Box<Token>> = None;
        let mut power_symbol : Option<Box<Token>> = None;
        let mut power_node : Option<Box<ASTNode>> = None;
        match &*self.lexer.get_symbol() {
            Token::PyMul( .. ) => {
                mul_symbol = Some( self.lexer.get_symbol() );
                let _ = &self.lexer.advance();
                mul_node = Some( self.parse_expression_test() );
                while 
                    match &*self.lexer.get_symbol() {
                        Token::PyComa( .. ) => {
                            separators_list.push( self.lexer.get_symbol() );
                            let _ = &self.lexer.advance();
                            match &*self.lexer.get_symbol() {
                                Token::PyPower( .. ) => {
                                    power_symbol = Some( self.lexer.get_symbol() );
                                    let _ = &self.lexer.advance();
                                    power_node = Some( self.parse_expression_test() );
                                    false
                                },
                                _ => {
                                    nodes_list.push( self.parse_expression_test() );
                                    true
                                }
                            }
                        },
                        _ => {
                            false
                        }
                    } {};
            },
            Token::PyPower( .. ) => {
                power_symbol = Some( self.lexer.get_symbol() );
                let _ = &self.lexer.advance();
                power_node = Some( self.parse_expression_test() );
            },
            _ => {
                nodes_list.push( self.parse_expression_test() );
                while 
                    match &*self.lexer.get_symbol() {
                        Token::PyComa( .. ) => {
                            separators_list.push( self.lexer.get_symbol() );
                            let _ = &self.lexer.advance();
                            match &*self.lexer.get_symbol() {
                                Token::PyPower( .. ) => {
                                    power_symbol = Some( self.lexer.get_symbol() );
                                    let _ = &self.lexer.advance();
                                    power_node = Some( self.parse_expression_test() );
                                    false
                                },
                                Token::PyMul( .. ) => {
                                    mul_symbol = Some( self.lexer.get_symbol() );
                                    let _ = &self.lexer.advance();
                                    mul_node = Some( self.parse_expression_test() );
                                    while 
                                        match &*self.lexer.get_symbol() {
                                            Token::PyComa( .. ) => {
                                                separators_list.push( self.lexer.get_symbol() );
                                                let _ = &self.lexer.advance();
                                                match &*self.lexer.get_symbol() {
                                                    Token::PyPower( .. ) => {
                                                        power_symbol = Some( self.lexer.get_symbol() );
                                                        let _ = &self.lexer.advance();
                                                        power_node = Some( self.parse_expression_test() );
                                                        false
                                                    },
                                                    _ => {
                                                        nodes_list.push( self.parse_expression_test() );
                                                        true
                                                    }
                                                }
                                            },
                                            _ => {
                                                false
                                            }
                                        } {};
                                    false
                                }
                                _ => {
                                    nodes_list.push( self.parse_expression_test() );
                                    true
                                }
                            }
                        },
                        _ => {
                            false
                        }
                    } {};
            }
        }
        nodes_list.reverse();
        separators_list.reverse();
        let end_pos = &self.lexer.get_position();
        Box::new( ASTNode::TypeList(*start_pos, *end_pos, nodes_list, separators_list, mul_symbol, mul_node, power_symbol, power_node) )
    }
}