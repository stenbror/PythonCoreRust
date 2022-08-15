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
        match self.symbol.clone() {
            Ok(s) => {
                let symbol1 = (*s).clone();
                match symbol1 {
                    Token::PyElipsis(..)  => {
                        let _ = self.advance();
                        Ok(Box::new(ASTNode::AtomElipsis(start_pos, self.lexer.get_position(), Box::new(symbol1))))
                    },
                    Token::PyFalse(..)  => {
                        let _ = self.advance();
                        Ok(Box::new(ASTNode::AtomFalse(start_pos, self.lexer.get_position(), Box::new(symbol1))))
                    },
                    Token::PyNone(..)  => {
                        let _ = self.advance();
                        Ok(Box::new(ASTNode::AtomNone(start_pos, self.lexer.get_position(), Box::new(symbol1))))
                    },
                    Token::PyTrue(..)  => {
                        let _ = self.advance();
                        Ok(Box::new(ASTNode::AtomTrue(start_pos, self.lexer.get_position(), Box::new(symbol1))))
                    },
                    Token::AtomName(..)  => {
                        let _ = self.advance();
                        Ok(Box::new(ASTNode::AtomName(start_pos, self.lexer.get_position(), Box::new(symbol1))))
                    },
                    Token::AtomNumber(..)  => {
                        let _ = self.advance();
                        Ok(Box::new(ASTNode::AtomNumber(start_pos, self.lexer.get_position(), Box::new(symbol1))))
                    },
                    Token::AtomString(..)  => {
                        let mut lst: Vec<Box<Token>> = Vec::new();
                        lst.push(Box::new(symbol1));
                        let _ = &self.advance();
                        while   match self.symbol.clone() {
                                Ok(s) => {
                                    let symbol1 = (*s).clone();
                                    match symbol1 {
                                        Token::AtomString(..) => {
                                            lst.push(Box::new(symbol1));
                                            let _ = &self.advance();
                                            true
                                        },
                                        _ => false
                                    }
                                },
                            _ => false
                            } {};
                        Ok(Box::new(ASTNode::AtomString(start_pos, self.lexer.get_position(), Box::new(lst))))
                    }
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

    #[test]
    fn expression_atom_name() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("__init__".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_parser_atom();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::AtomName( 0, 8, tok) => {
                        match &**tok {
                            Token::AtomName(0, 8, None, txt) => {
                                match &*txt.as_str() {
                                    "__init__" => assert!(true),
                                    _ => assert!(false)
                                }
                            },
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
    fn expression_atom_number() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("0.32e-4J".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_parser_atom();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::AtomNumber( 0, 8, tok) => {
                        match &**tok {
                            Token::AtomNumber(0, 8, None, txt) => {
                                match &*txt.as_str() {
                                    "0.32e-4J" => assert!(true),
                                    _ => assert!(false)
                                }
                            },
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
    fn expression_atom_single_string() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("'Hello, World!'".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_parser_atom();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::AtomString( 0, 15, tok) => {
                       let el = (*tok).last();
                        match el {
                            Some(el2) => {
                                match *el2.clone() {
                                    Token::AtomString( 0, 15, None, txt, None ) => {
                                        match &*txt.as_str() {
                                            "'Hello, World!'" => assert!(true),
                                            _ => assert!(false)
                                        }
                                    },
                                    _ => assert!(false)
                                }
                            },
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
    fn expression_atom_multiple_string() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("'Hello, World!''123'".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_parser_atom();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::AtomString( 0, 20 , tok) => {
                        let lst = (**tok).clone();
                        assert_eq!(2, lst.len());
                        let a = &*lst[0]; // First string Token
                        let b = &*lst[1]; // Second string Token
                        match a {
                            Token::AtomString( _ , _ , None, txt, None) => assert_eq!("'Hello, World!'", &**txt),
                            _ => assert!(false)
                        }
                        match b {
                            Token::AtomString( _ , _ , None, txt, None) => assert_eq!("'123'", &**txt),
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