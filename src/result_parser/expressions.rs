use crate::{ASTNode, Token};
use crate::result_parser::parser::{Parser, PythonCoreParser};
use crate::result_parser::tokenizer::Tokenizer;


pub trait Expressions {
    fn parse_expressions_parser_atom(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_parser_atom_expr(&mut self) -> Result<Box<ASTNode>, String>;
}


impl Expressions for PythonCoreParser {
    fn parse_expressions_parser_atom(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s) => {
                let symb1 = (&**s).clone();
                match symb1 {
                    Token::PyElipsis(..)  => {
                        let _ = self.advance();
                        Ok(Box::new(ASTNode::AtomElipsis(start_pos, self.lexer.get_position(), Box::new(symb1))))
                    },
                    Token::PyFalse(..)  => {
                        let _ = self.advance();
                        Ok(Box::new(ASTNode::AtomFalse(start_pos, self.lexer.get_position(), Box::new(symb1))))
                    },
                    Token::PyNone(..)  => {
                        let _ = self.advance();
                        Ok(Box::new(ASTNode::AtomNone(start_pos, self.lexer.get_position(), Box::new(symb1))))
                    },
                    Token::PyTrue(..)  => {
                        let _ = self.advance();
                        Ok(Box::new(ASTNode::AtomTrue(start_pos, self.lexer.get_position(), Box::new(symb1))))
                    },

                    _ => Err(format!("SyntaxError at {}: Expecting symbol in atom expression!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in atom expression!", start_pos))
        }
    }

    fn parse_expressions_parser_atom_expr(&mut self) -> Result<Box<ASTNode>, String> {

        Ok(Box::new(ASTNode::Empty))
    }
}


// UnitTests for expression rules /////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{ASTNode, Token};
    use crate::result_parser::expressions::Expressions;
    use crate::result_parser::tokenizer::{PythonCoreTokenizer, Tokenizer};
    use crate::parser::trivias::Trivia;
    use crate::result_parser::parser::{Parser, PythonCoreParser};


    #[test]
    fn expression_atom_ellipsis() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_parser_atom();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::AtomElipsis( 0, 3, tok) => {
                        match &**tok {
                            Token::PyElipsis(0, 3, None) => assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            Err( .. ) => assert!(false)
        }
    }

    #[test]
    fn expression_atom_false() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("False".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_parser_atom();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::AtomFalse( 0, 5, tok) => {
                        match &**tok {
                            Token::PyFalse(0, 5, None) => assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            Err( .. ) => assert!(false)
        }
    }

    #[test]
    fn expression_atom_none() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("None".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_parser_atom();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::AtomNone( 0, 4, tok) => {
                        match &**tok {
                            Token::PyNone(0, 4, None) => assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            Err( .. ) => assert!(false)
        }
    }

    #[test]
    fn expression_atom_true() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("True".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_parser_atom();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::AtomTrue( 0, 4, tok) => {
                        match &**tok {
                            Token::PyTrue(0, 4, None) => assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            Err( .. ) => assert!(false)
        }
    }

}