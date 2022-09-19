
use crate::result_parser::nodes::{ ASTNode };
use crate::result_parser::tokens::{ Token };
use crate::result_parser::parser::{ Parser, PythonCoreParser };
use crate::result_parser::expressions::Expressions;
use crate::result_parser::tokenizer::Tokenizer;


pub trait Functions {
    fn parse_functions_func_type(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_functions_type_list(&mut self) -> Result<Box<ASTNode>, String>;
}


impl Functions for PythonCoreParser {
    fn parse_functions_func_type(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyLeftParen(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        let mut left_node : Option<Box<ASTNode>> = None;
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyRightParen(..) => { },
                                    _ => {
                                        left_node = Some(self.parse_functions_type_list()?);
                                    }
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in functional type!", start_pos))
                        }
                        match self.symbol.clone() {
                            Ok(s3) => {
                                match &*s3 {
                                    Token::PyRightParen(..) => {
                                        let symbol2 = s3;
                                        let _ = self.advance();
                                        match self.symbol.clone() {
                                            Ok(s4) => {
                                                match &*s4 {
                                                    Token::PyArrow(..) => {
                                                        let symbol3 = s4;
                                                        let _ = self.advance();
                                                        let right_node = self.parse_expressions_test()?;
                                                        Ok(Box::new( ASTNode::FuncType(start_pos, self.lexer.get_position(), symbol1, left_node, symbol2, symbol3, right_node) ))
                                                    },
                                                    _ => Err(format!("SyntaxError at {}: Expecting '(' in functional type!", start_pos))
                                                }
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting symbol in functional type!", start_pos))
                                        }
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting '(' in functional type!", start_pos))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in functional type!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting '(' in functional type!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in functional type!", start_pos))
        }
    }

    fn parse_functions_type_list(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }
}


// UnitTests for functions rules //////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{ASTNode, Token};
    use crate::result_parser::expressions::Expressions;
    use crate::result_parser::tokenizer::{PythonCoreTokenizer, Tokenizer};
    use crate::result_parser::trivias::Trivia;
    use crate::result_parser::parser::{Parser, PythonCoreParser};


    #[test]
    fn statements_empty_template() {
        assert!(true)
    }

}
