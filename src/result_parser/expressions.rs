use crate::{ASTNode, Token};
use crate::result_parser::parser::{Parser, PythonCoreParser};
use crate::result_parser::tokenizer::Tokenizer;


pub trait Expressions {
    fn parse_expressions_atom(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_atom_expr(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_power(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_factor(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_term(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_arith(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_shift(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_and_expr(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_xor_expr(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_expr(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_star_expr(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_comparison(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_not_test(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_and_test(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_or_test(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_lambda_def(&mut self, cond: bool) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_no_cond_test(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_test(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_named_expression(&mut self) -> Result<Box<ASTNode>, String>;


    fn parse_expressions_trailer(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_var_args_list(&mut self) -> Result<Box<ASTNode>, String>;
}


impl Expressions for PythonCoreParser {
    fn parse_expressions_atom(&mut self) -> Result<Box<ASTNode>, String> {
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

    fn parse_expressions_atom_expr(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();

        /* Optional 'await' prefix */
        let mut await_symbol: Option<Box<Token>> = None;
        match self.symbol.clone() {
            Ok(s) => {
                let symbol1 = (*s).clone();
                match symbol1 {
                    Token::PyAwait(..) => {
                        await_symbol = Some( Box::new(symbol1 ));
                        let _ = &self.advance();
                    },
                    _ => {}
                }
            },
            _ => return Err(format!("SyntaxError at {}: Expecting symbol in atom_expr expression!", start_pos))
        }

        /* main node collector */
        let right_node_raw = self.parse_expressions_atom();
        match right_node_raw {
            Ok(s) => {
                let right_node = s;

                /* Optional trailers */
                let mut lst : Vec<Box<ASTNode>> = Vec::new();
                while   match self.symbol.clone() {
                            Ok(s) => {
                                match *s {
                                    Token::PyLeftParen(..) |
                                    Token::PyLeftBracket(..) |
                                    Token::PyDot(..) => {
                                        let next_node_raw = self.parse_expressions_trailer();
                                        match next_node_raw {
                                            Ok(s) => {
                                                lst.push(s);
                                            },
                                            _ => return next_node_raw
                                        }
                                        true
                                    },
                                    _ => false
                                }
                            },
                            _ => false
                        } {};

                /* Returning needed node */
                match ( &await_symbol, lst.len() ) {
                    ( None, 0 ) => Ok(right_node),
                    ( _ , _ ) => {
                        Ok(Box::new(ASTNode::AtomExpr(start_pos, self.lexer.get_position(), await_symbol, right_node, Box::new(lst))))
                    }
                }
            },
            _ => right_node_raw
        }
    }

    fn parse_expressions_power(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let left_node_raw = self.parse_expressions_atom_expr();
        match &left_node_raw {
            Ok(s) => {
                let left_node = (**s).clone();
                match &self.symbol {
                    Ok(s) => {
                        match &**s {
                            Token::PyPower(..) => {
                                let symbol = (**s).clone();
                                let _ = self.advance();
                                let right_node_raw = self.parse_expressions_factor();
                                match &right_node_raw {
                                    Ok(s) => {
                                        let right_node = (**s).clone();
                                        Ok(Box::new(ASTNode::PowerExpr(start_pos, self.lexer.get_position(), Box::new(left_node), Box::new(symbol), Box::new(right_node))))
                                    },
                                    _ => right_node_raw
                                }
                            },
                            _ => left_node_raw
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting symbol in power expression!", start_pos))
                }
            },
            _ => left_node_raw // This returns the error state for left node!
        }
    }

    fn parse_expressions_factor(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s) => {
                match &**s {
                    Token::PyPlus(..) => {
                        let symbol = (**s).clone();
                        let _ = self.advance();
                        let right_node_raw = self.parse_expressions_factor();
                        match &right_node_raw {
                            Ok(s) => {
                                let right_node = (**s).clone();
                                Ok(Box::new(ASTNode::UnaryPlus(start_pos, self.lexer.get_position(), Box::new(symbol), Box::new(right_node))))
                            },
                            _ => right_node_raw
                        }
                    },
                    Token::PyMinus(..) => {
                        let symbol = (**s).clone();
                        let _ = self.advance();
                        let right_node_raw = self.parse_expressions_factor();
                        match &right_node_raw {
                            Ok(s) => {
                                let right_node = (**s).clone();
                                Ok(Box::new(ASTNode::UnaryMinus(start_pos, self.lexer.get_position(), Box::new(symbol), Box::new(right_node))))
                            },
                            _ => right_node_raw
                        }
                    },
                    Token::PyBitInvert(..) => {
                        let symbol = (**s).clone();
                        let _ = self.advance();
                        let right_node_raw = self.parse_expressions_factor();
                        match &right_node_raw {
                            Ok(s) => {
                                let right_node = (**s).clone();
                                Ok(Box::new(ASTNode::UnaryInvert(start_pos, self.lexer.get_position(), Box::new(symbol), Box::new(right_node))))
                            },
                            _ => right_node_raw
                        }
                    },
                    _ => self.parse_expressions_power()
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in factor expression!", start_pos))
        }
    }

    fn parse_expressions_term(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node_raw = self.parse_expressions_factor();
        match &left_node_raw {
            Ok(..) => {
                while   match &self.symbol {
                            Ok(symbol_x) => {
                                let symbol = (**symbol_x).clone();
                                match &left_node_raw {
                                    Ok(s) => {
                                        let left_node = (**s).clone();
                                        match &symbol {
                                            Token::PyMul(..) => {
                                                let _ = self.advance();
                                                let right_node_raw = self.parse_expressions_factor();
                                                match &right_node_raw {
                                                    Ok(s) => {
                                                        let right_node = (**s).clone();
                                                        left_node_raw = Ok(Box::new(ASTNode::MulTerm(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                        true
                                                    },
                                                    _ => return right_node_raw
                                                }
                                            },
                                            Token::PyDiv(..) => {
                                                let _ = self.advance();
                                                let right_node_raw = self.parse_expressions_factor();
                                                match &right_node_raw {
                                                    Ok(s) => {
                                                        let right_node = (**s).clone();
                                                        left_node_raw = Ok(Box::new(ASTNode::DivTerm(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                        true
                                                    },
                                                    _ => return right_node_raw
                                                }
                                            },
                                            Token::PyFloorDiv(..) => {
                                                let _ = self.advance();
                                                let right_node_raw = self.parse_expressions_factor();
                                                match &right_node_raw {
                                                    Ok(s) => {
                                                        let right_node = (**s).clone();
                                                        left_node_raw = Ok(Box::new(ASTNode::FloorDivTerm(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                        true
                                                    },
                                                    _ => return right_node_raw
                                                }
                                            },
                                            Token::PyModulo(..) => {
                                                let _ = self.advance();
                                                let right_node_raw = self.parse_expressions_factor();
                                                match &right_node_raw {
                                                    Ok(s) => {
                                                        let right_node = (**s).clone();
                                                        left_node_raw = Ok(Box::new(ASTNode::ModuloTerm(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                        true
                                                    },
                                                    _ => return right_node_raw
                                                }
                                            },
                                            Token::PyMatrice(..) => {
                                                let _ = self.advance();
                                                let right_node_raw = self.parse_expressions_factor();
                                                match &right_node_raw {
                                                    Ok(s) => {
                                                        let right_node = (**s).clone();
                                                        left_node_raw = Ok(Box::new(ASTNode::MatriceTerm(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                        true
                                                    },
                                                    _ => return right_node_raw
                                                }
                                            },
                                            _ => false
                                        }
                                    },
                                    _ => false
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in term expression!", start_pos))
                        } {};
                left_node_raw
            },
            _ => left_node_raw
        }
    }

    fn parse_expressions_arith(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node_raw = self.parse_expressions_term();
        match &left_node_raw {
            Ok(..) => {
                while   match &self.symbol {
                    Ok(symbol_x) => {
                        let symbol = (**symbol_x).clone();
                        match &left_node_raw {
                            Ok(s) => {
                                let left_node = (**s).clone();
                                match &symbol {
                                    Token::PyPlus(..) => {
                                        let _ = self.advance();
                                        let right_node_raw = self.parse_expressions_term();
                                        match &right_node_raw {
                                            Ok(s) => {
                                                let right_node = (**s).clone();
                                                left_node_raw = Ok(Box::new(ASTNode::PlusArithExpr(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                true
                                            },
                                            _ => return right_node_raw
                                        }
                                    },
                                    Token::PyMinus(..) => {
                                        let _ = self.advance();
                                        let right_node_raw = self.parse_expressions_term();
                                        match &right_node_raw {
                                            Ok(s) => {
                                                let right_node = (**s).clone();
                                                left_node_raw = Ok(Box::new(ASTNode::MinusArithExpr(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                true
                                            },
                                            _ => return right_node_raw
                                        }
                                    },
                                    _ => false
                                }
                            },
                            _ => false
                        }
                    },
                    _ => return Err(format!("SyntaxError at {}: Expecting symbol in arith expression!", start_pos))
                } {};
                left_node_raw
            },
            _ => left_node_raw
        }
    }

    fn parse_expressions_shift(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node_raw = self.parse_expressions_arith();
        match &left_node_raw {
            Ok(..) => {
                while   match &self.symbol {
                    Ok(symbol_x) => {
                        let symbol = (**symbol_x).clone();
                        match &left_node_raw {
                            Ok(s) => {
                                let left_node = (**s).clone();
                                match &symbol {
                                    Token::PyShiftLeft(..) => {
                                        let _ = self.advance();
                                        let right_node_raw = self.parse_expressions_arith();
                                        match &right_node_raw {
                                            Ok(s) => {
                                                let right_node = (**s).clone();
                                                left_node_raw = Ok(Box::new(ASTNode::ShiftLeftExpr(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                true
                                            },
                                            _ => return right_node_raw
                                        }
                                    },
                                    Token::PyShiftRight(..) => {
                                        let _ = self.advance();
                                        let right_node_raw = self.parse_expressions_arith();
                                        match &right_node_raw {
                                            Ok(s) => {
                                                let right_node = (**s).clone();
                                                left_node_raw = Ok(Box::new(ASTNode::ShiftRightExpr(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                true
                                            },
                                            _ => return right_node_raw
                                        }
                                    },
                                    _ => false
                                }
                            },
                            _ => false
                        }
                    },
                    _ => return Err(format!("SyntaxError at {}: Expecting symbol in shift expression!", start_pos))
                } {};
                left_node_raw
            },
            _ => left_node_raw
        }
    }

    fn parse_expressions_and_expr(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node_raw = self.parse_expressions_shift();
        match &left_node_raw {
            Ok(..) => {
                while   match &self.symbol {
                    Ok(symbol_x) => {
                        let symbol = (**symbol_x).clone();
                        match &left_node_raw {
                            Ok(s) => {
                                let left_node = (**s).clone();
                                match &symbol {
                                    Token::PyBitAnd(..) => {
                                        let _ = self.advance();
                                        let right_node_raw = self.parse_expressions_shift();
                                        match &right_node_raw {
                                            Ok(s) => {
                                                let right_node = (**s).clone();
                                                left_node_raw = Ok(Box::new(ASTNode::AndExpr(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                true
                                            },
                                            _ => return right_node_raw
                                        }
                                    },
                                    _ => false
                                }
                            },
                            _ => false
                        }
                    },
                    _ => return Err(format!("SyntaxError at {}: Expecting symbol in and expression!", start_pos))
                } {};
                left_node_raw
            },
            _ => left_node_raw
        }
    }

    fn parse_expressions_xor_expr(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node_raw = self.parse_expressions_and_expr();
        match &left_node_raw {
            Ok(..) => {
                while   match &self.symbol {
                    Ok(symbol_x) => {
                        let symbol = (**symbol_x).clone();
                        match &left_node_raw {
                            Ok(s) => {
                                let left_node = (**s).clone();
                                match &symbol {
                                    Token::PyBitXor(..) => {
                                        let _ = self.advance();
                                        let right_node_raw = self.parse_expressions_and_expr();
                                        match &right_node_raw {
                                            Ok(s) => {
                                                let right_node = (**s).clone();
                                                left_node_raw = Ok(Box::new(ASTNode::XorExpr(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                true
                                            },
                                            _ => return right_node_raw
                                        }
                                    },
                                    _ => false
                                }
                            },
                            _ => false
                        }
                    },
                    _ => return Err(format!("SyntaxError at {}: Expecting symbol in xor expression!", start_pos))
                } {};
                left_node_raw
            },
            _ => left_node_raw
        }
    }

    fn parse_expressions_expr(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node_raw = self.parse_expressions_xor_expr();
        match &left_node_raw {
            Ok(..) => {
                while   match &self.symbol {
                    Ok(symbol_x) => {
                        let symbol = (**symbol_x).clone();
                        match &left_node_raw {
                            Ok(s) => {
                                let left_node = (**s).clone();
                                match &symbol {
                                    Token::PyBitOr(..) => {
                                        let _ = self.advance();
                                        let right_node_raw = self.parse_expressions_xor_expr();
                                        match &right_node_raw {
                                            Ok(s) => {
                                                let right_node = (**s).clone();
                                                left_node_raw = Ok(Box::new(ASTNode::Expr(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                true
                                            },
                                            _ => return right_node_raw
                                        }
                                    },
                                    _ => false
                                }
                            },
                            _ => false
                        }
                    },
                    _ => return Err(format!("SyntaxError at {}: Expecting symbol in xor expression!", start_pos))
                } {};
                left_node_raw
            },
            _ => left_node_raw
        }
    }

    fn parse_expressions_star_expr(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s) => {
                match &**s {
                    Token::PyMul(..) => {
                        let symbol = (**s).clone();
                        let _ = self.advance();
                        let right_node_raw = self.parse_expressions_expr();
                        match &right_node_raw {
                            Ok(s) => {
                                let right_node = (**s).clone();
                                Ok(Box::new(ASTNode::StarExpr(start_pos, self.lexer.get_position(), Box::new(symbol), Box::new(right_node))))
                            },
                            _ => right_node_raw
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting '*' in star expression!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in star expression!", start_pos))
        }
    }

    fn parse_expressions_comparison(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node_raw = self.parse_expressions_expr();
        match &left_node_raw {
            Ok(..) => {
                while   match &self.symbol {
                    Ok(symbol_x) => {
                        let symbol = (**symbol_x).clone();
                        match &left_node_raw {
                            Ok(s) => {
                                let left_node = (**s).clone();
                                match &symbol {
                                    Token::PyLess(..) => {
                                        let _ = self.advance();
                                        let right_node_raw = self.parse_expressions_expr();
                                        match &right_node_raw {
                                            Ok(s) => {
                                                let right_node = (**s).clone();
                                                left_node_raw = Ok(Box::new(ASTNode::LessComparison(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                true
                                            },
                                            _ => return right_node_raw
                                        }
                                    },
                                    Token::PyLessEqual(..) => {
                                        let _ = self.advance();
                                        let right_node_raw = self.parse_expressions_expr();
                                        match &right_node_raw {
                                            Ok(s) => {
                                                let right_node = (**s).clone();
                                                left_node_raw = Ok(Box::new(ASTNode::LessEqualComparison(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                true
                                            },
                                            _ => return right_node_raw
                                        }
                                    },
                                    Token::PyEqual(..) => {
                                        let _ = self.advance();
                                        let right_node_raw = self.parse_expressions_expr();
                                        match &right_node_raw {
                                            Ok(s) => {
                                                let right_node = (**s).clone();
                                                left_node_raw = Ok(Box::new(ASTNode::EqualComparison(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                true
                                            },
                                            _ => return right_node_raw
                                        }
                                    },
                                    Token::PyGreaterEqual(..) => {
                                        let _ = self.advance();
                                        let right_node_raw = self.parse_expressions_expr();
                                        match &right_node_raw {
                                            Ok(s) => {
                                                let right_node = (**s).clone();
                                                left_node_raw = Ok(Box::new(ASTNode::GreaterEqualComparison(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                true
                                            },
                                            _ => return right_node_raw
                                        }
                                    },
                                    Token::PyGreater(..) => {
                                        let _ = self.advance();
                                        let right_node_raw = self.parse_expressions_expr();
                                        match &right_node_raw {
                                            Ok(s) => {
                                                let right_node = (**s).clone();
                                                left_node_raw = Ok(Box::new(ASTNode::GreaterComparison(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                true
                                            },
                                            _ => return right_node_raw
                                        }
                                    },
                                    Token::PyNotEqual(..) => {
                                        let _ = self.advance();
                                        let right_node_raw = self.parse_expressions_expr();
                                        match &right_node_raw {
                                            Ok(s) => {
                                                let right_node = (**s).clone();
                                                left_node_raw = Ok(Box::new(ASTNode::NotEqualComparison(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                true
                                            },
                                            _ => return right_node_raw
                                        }
                                    },
                                    Token::PyIn(..) => {
                                        let _ = self.advance();
                                        let right_node_raw = self.parse_expressions_expr();
                                        match &right_node_raw {
                                            Ok(s) => {
                                                let right_node = (**s).clone();
                                                left_node_raw = Ok(Box::new(ASTNode::InComparison(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                true
                                            },
                                            _ => return right_node_raw
                                        }
                                    },
                                    Token::PyIs(..) => {
                                        let _ = self.advance();
                                        match &self.symbol {
                                            Ok(symbol_x) => {
                                                let symbol2 = (**symbol_x).clone();
                                                match &symbol2 {
                                                    Token::PyNot(..) => {
                                                        let _ = self.advance();
                                                        let right_node_raw = self.parse_expressions_expr();
                                                        match &right_node_raw {
                                                            Ok(s) => {
                                                                let right_node = (**s).clone();
                                                                left_node_raw = Ok(Box::new(ASTNode::IsNotComparison(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(symbol2), Box::new(right_node))));
                                                            },
                                                            _ => return right_node_raw
                                                        }
                                                    },
                                                    _ => {
                                                        let right_node_raw = self.parse_expressions_expr();
                                                        match &right_node_raw {
                                                            Ok(s) => {
                                                                let right_node = (**s).clone();
                                                                left_node_raw = Ok(Box::new(ASTNode::IsComparison(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                            },
                                                            _ => return right_node_raw
                                                        }
                                                    }
                                                }
                                            },
                                            _ => {
                                                return Err(format!("SyntaxError at {}: Expecting 'is' pr 'is not' in comparison expression!", start_pos))
                                            }
                                        }
                                        true
                                    },
                                    Token::PyNot(..) => {
                                        let _ = self.advance();
                                        match &self.symbol {
                                            Ok(symbol_x) => {
                                                let symbol2 = (**symbol_x).clone();
                                                match &symbol2 {
                                                    Token::PyIn(..) => {
                                                        let _ = self.advance();
                                                        let right_node_raw = self.parse_expressions_expr();
                                                        match &right_node_raw {
                                                            Ok(s) => {
                                                                let right_node = (**s).clone();
                                                                left_node_raw = Ok(Box::new(ASTNode::NotInComparison(start_pos, self.lexer.get_position(), Box::new(left_node), Box::new(symbol), Box::new(symbol2), Box::new(right_node))));
                                                            },
                                                            _ => return right_node_raw
                                                        }
                                                    },
                                                    _ => {
                                                        return Err(format!("SyntaxError at {}: Expecting 'in' pr 'not in' in comparison expression!", start_pos))
                                                    }
                                                }
                                            },
                                            _ => {
                                                return Err(format!("SyntaxError at {}: Expecting 'is' pr 'is not' in comparison expression!", start_pos))
                                            }
                                        }
                                        true
                                    },
                                    _ => false
                                }
                            },
                            _ => false
                        }
                    },
                    _ => return Err(format!("SyntaxError at {}: Expecting symbol in comparison expression!", start_pos))
                } {};
                left_node_raw
            },
            _ => left_node_raw
        }
    }

    fn parse_expressions_not_test(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s) => {
                match &**s {
                    Token::PyNot(..) => {
                        let symbol = (**s).clone();
                        let _ = self.advance();
                        let right_node_raw = self.parse_expressions_not_test();
                        match &right_node_raw {
                            Ok(s) => {
                                let right_node = (**s).clone();
                                Ok(Box::new(ASTNode::NotTest(start_pos, self.lexer.get_position(), Box::new(symbol), Box::new(right_node))))
                            },
                            _ => right_node_raw
                        }
                    },
                    _ => self.parse_expressions_power()
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in not test expression!", start_pos))
        }
    }

    fn parse_expressions_and_test(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node_raw = self.parse_expressions_not_test();
        match &left_node_raw {
            Ok(..) => {
                while   match &self.symbol {
                    Ok(symbol_x) => {
                        let symbol = (**symbol_x).clone();
                        match &left_node_raw {
                            Ok(s) => {
                                let left_node = (**s).clone();
                                match &symbol {
                                    Token::PyAnd(..) => {
                                        let _ = self.advance();
                                        let right_node_raw = self.parse_expressions_not_test();
                                        match &right_node_raw {
                                            Ok(s) => {
                                                let right_node = (**s).clone();
                                                left_node_raw = Ok(Box::new(ASTNode::AndTest(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                true
                                            },
                                            _ => return right_node_raw
                                        }
                                    },
                                    _ => false
                                }
                            },
                            _ => false
                        }
                    },
                    _ => return Err(format!("SyntaxError at {}: Expecting symbol in and test expression!", start_pos))
                } {};
                left_node_raw
            },
            _ => left_node_raw
        }
    }

    fn parse_expressions_or_test(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node_raw = self.parse_expressions_and_test();
        match &left_node_raw {
            Ok(..) => {
                while   match &self.symbol {
                    Ok(symbol_x) => {
                        let symbol = (**symbol_x).clone();
                        match &left_node_raw {
                            Ok(s) => {
                                let left_node = (**s).clone();
                                match &symbol {
                                    Token::PyOr(..) => {
                                        let _ = self.advance();
                                        let right_node_raw = self.parse_expressions_and_test();
                                        match &right_node_raw {
                                            Ok(s) => {
                                                let right_node = (**s).clone();
                                                left_node_raw = Ok(Box::new(ASTNode::OrTest(start_pos, self.lexer.get_position(), Box::new(left_node),Box::new(symbol), Box::new(right_node))));
                                                true
                                            },
                                            _ => return right_node_raw
                                        }
                                    },
                                    _ => false
                                }
                            },
                            _ => false
                        }
                    },
                    _ => return Err(format!("SyntaxError at {}: Expecting symbol in and test expression!", start_pos))
                } {};
                left_node_raw
            },
            _ => left_node_raw
        }
    }

    fn parse_expressions_lambda_def(&mut self, cond: bool) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(symbol_x) => {
                let symbol1 = (**symbol_x).clone(); // 'lambda'
                match &symbol1 {
                    Token::PyLambda(..) => {
                        let _ = self.advance();
                        let mut left_node : Option<Box<ASTNode>> = None;

                        match &self.symbol {
                            Ok(..) => {
                                //let symbol2 = (**symbol_x).clone();

                                /* Handle optional left node or colon */
                                match &symbol1 {
                                    Token::PyColon(..) => {},
                                    _ => {
                                        let left_node_raw = self.parse_expressions_and_test();
                                        match &left_node_raw {
                                            Ok(s1) => {
                                                left_node = Some(Box::new((**s1).clone()));
                                            },
                                            _ => return left_node_raw
                                        }
                                    }
                                }

                                match &self.symbol {
                                    Ok(symbol_x) => {
                                        let symbol3 = (**symbol_x).clone();
                                        match &symbol1 {
                                            Token::PyColon(..) => {
                                                let _ = self.advance(); // ':'
                                                let right_node = if cond { self.parse_expressions_test() } else { self.parse_expressions_no_cond_test() };
                                                match right_node {
                                                    Ok(right) => {
                                                        Ok(Box::new(ASTNode::Lambda(start_pos, self.lexer.get_position(), Box::new(symbol1), left_node, Box::new(symbol3), right)))
                                                    },
                                                    _ => Err(format!("SyntaxError at {}: Expecting expression after ':' in lambda expression!", self.lexer.get_position()))
                                                }
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting ':' in lambda expression!", self.lexer.get_position()))
                                        }
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting symbol in lambda expression!", self.lexer.get_position()))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in lambda expression!", self.lexer.get_position()))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'lambda' in lambda expression!", self.lexer.get_position()))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in lambda expression!", self.lexer.get_position()))
        }
    }

    fn parse_expressions_no_cond_test(&mut self) -> Result<Box<ASTNode>, String> {
        match &self.symbol {
            Ok(symbol_x) => {
                let symbol = (**symbol_x).clone();
                match &symbol {
                    Token::PyLambda(..) => self.parse_expressions_lambda_def(false),
                    _ => self.parse_expressions_or_test()
                }
            },
            _ => return Err(format!("SyntaxError at {}: Expecting symbol in non conditional test expression!", self.lexer.get_position()))
        }
    }

    fn parse_expressions_test(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s) => {
                match (**s).clone() {
                    Token::PyLambda(..) => self.parse_expressions_lambda_def(true),
                    _ => {
                        let left = self.parse_expressions_or_test()?;
                        match &self.symbol {
                            Ok(s2) => {
                                let symbol1 = (**s2).clone();
                                match &symbol1 {
                                    Token::PyIf(..) => {
                                        let _ = self.advance();
                                        let right = self.parse_expressions_or_test()?;
                                        match &self.symbol {
                                            Ok(s3) => {
                                                let symbol2 = (**s3).clone();
                                                match &symbol2 {
                                                    Token::PyElse(..) => {
                                                        let _ = self.advance();
                                                        let next = self.parse_expressions_test()?;
                                                        Ok(Box::new(ASTNode::Test(start_pos, self.lexer.get_position(), left, Box::new(symbol1), right, Box::new(symbol2), next)))
                                                    },
                                                    _ => Err(format!("SyntaxError at {}: Expecting 'else' in test expression!", self.lexer.get_position()))
                                                }
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting symbol in test expression!", self.lexer.get_position()))
                                        }
                                    },
                                    _ => Ok(left)
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in test expression!", self.lexer.get_position()))
                        }
                    }
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in non conditional test expression!", self.lexer.get_position()))
        }
    }

    fn parse_expressions_named_expression(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let left = self.parse_expressions_test()?;
        match &self.symbol {
            Ok(s) => {
                let symbol = (**s).clone();
                match &symbol {
                    Token::PyColonAssign(..) => {
                        let _ = self.advance();
                        let right = self.parse_expressions_test()?;
                        Ok(Box::new(ASTNode::NamedExpr(start_pos, self.lexer.get_position(), left, Box::new(symbol), right)))
                    },
                    _ => Ok(left)
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in named expression!", self.lexer.get_position()))
        }
    }

    fn parse_expressions_trailer(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_expressions_var_args_list(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
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
        let res = parser.parse_expressions_atom();
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
        let res = parser.parse_expressions_atom();
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
        let res = parser.parse_expressions_atom();
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
        let res = parser.parse_expressions_atom();
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
        let res = parser.parse_expressions_atom();
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
        let res = parser.parse_expressions_atom();
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
        let res = parser.parse_expressions_atom();
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
        let res = parser.parse_expressions_atom();
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

    #[test]
    fn expression_atom_non_await_name() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("__init__".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_atom_expr();
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
    fn expression_atom_await_name() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("await __init__".to_string()) ); // 14
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_atom_expr();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::AtomExpr(0, 14, tok, right, next) => {
                        match &tok {
                            Some( s ) => {
                                match &**s {
                                    Token::PyAwait( 0, 5, None) => assert!(true),
                                    _ => assert!(false)
                                }
                                assert!(true)
                            },
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName(6, 14, tok2) => {
                                let symbol_text = &**tok2;
                                let pattern = Box::new( ("__init__".to_string()));
                                match &**tok2 {
                                    Token::AtomName(6, 14, trivia , pattern) => {
                                        match &trivia {
                                            Some(s) => {
                                                let x = &**s;
                                                assert_eq!(1, x.len());
                                                match *x[0] {
                                                    Trivia::WhiteSpace( 5 , 6 , ' ') => assert!(true),
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
                        assert_eq!(0, (**next).len()); // No trailers added to node as expected!
                    },
                    _ => assert!(false)
                }
            }
            Err( .. ) => assert!(false)
        }
    }

    #[test]
    fn expression_power_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a ** b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_power();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::PowerExpr( 0, 6, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyPower(2, 4, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 5, 6 , _ ) => assert!(true),
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
    fn expression_no_power_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_power();
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
    fn expression_factor_operator_plus() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("+b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_factor();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::UnaryPlus( 0, 2, symbol, right) => {
                        match &**symbol {
                            Token::PyPlus(0, 1, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 1, 2 , _ ) => assert!(true),
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
    fn expression_factor_operator_minus() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("-b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_factor();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::UnaryMinus( 0, 2, symbol, right) => {
                        match &**symbol {
                            Token::PyMinus(0, 1, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 1, 2 , _ ) => assert!(true),
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
    fn expression_factor_operator_bit_invert() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("~b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_factor();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::UnaryInvert( 0, 2, symbol, right) => {
                        match &**symbol {
                            Token::PyBitInvert(0, 1, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 1, 2 , _ ) => assert!(true),
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
    fn expression_no_factor_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_factor();
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
    fn expression_term_operator_mul() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a * b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_term();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::MulTerm( 0, 5, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyMul(2, 3, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 4, 5 , _ ) => assert!(true),
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
    fn expression_term_operator_div() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a / b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_term();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::DivTerm( 0, 5, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyDiv(2, 3, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 4, 5 , _ ) => assert!(true),
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
    fn expression_term_operator_modulo() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a % b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_term();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::ModuloTerm( 0, 5, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyModulo(2, 3, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 4, 5 , _ ) => assert!(true),
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
    fn expression_term_operator_matrices() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a @ b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_term();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::MatriceTerm( 0, 5, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyMatrice(2, 3, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 4, 5 , _ ) => assert!(true),
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
    fn expression_term_operator_floor_div() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a // b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_term();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::FloorDivTerm( 0, 6, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyFloorDiv(2, 4, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 5, 6 , _ ) => assert!(true),
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
    fn expression_no_term_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_term();
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
    fn expression_term_operator_matrices_double() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a @ b @ c".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_term();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::MatriceTerm( 0, 9, left, symbol, right) => {
                        match &**left {
                            ASTNode::MatriceTerm( 0, 6 , left2 , symbol2 , right2 ) => {
                                match &**left2 {
                                    ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**symbol2 {
                                    Token::PyMatrice(2, 3, _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**right2 {
                                    ASTNode::AtomName( 4, 6 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                            },
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyMatrice(6, 7, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 8, 9 , _ ) => assert!(true),
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
    fn expression_arith_operator_plus() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a + b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_arith();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::PlusArithExpr( 0, 5, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyPlus(2, 3, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 4, 5 , _ ) => assert!(true),
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
    fn expression_arith_operator_minus() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a - b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_arith();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::MinusArithExpr( 0, 5, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyMinus(2, 3, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 4, 5 , _ ) => assert!(true),
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
    fn expression_no_arith_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_arith();
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
    fn expression_arith_operator_plus_minus_double() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a + b - c".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_arith();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::MinusArithExpr( 0, 9, left, symbol, right) => {
                        match &**left {
                            ASTNode::PlusArithExpr( 0, 6 , left2 , symbol2 , right2 ) => {
                                match &**left2 {
                                    ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**symbol2 {
                                    Token::PyPlus(2, 3, _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**right2 {
                                    ASTNode::AtomName( 4, 6 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                            },
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyMinus(6, 7, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 8, 9 , _ ) => assert!(true),
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
    fn expression_shift_operator_left() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a << b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_shift();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::ShiftLeftExpr( 0, 6, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyShiftLeft(2, 4, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 5, 6 , _ ) => assert!(true),
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
    fn expression_shift_operator_right() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a >> b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_shift();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::ShiftRightExpr( 0, 6, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyShiftRight(2, 4, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 5, 6 , _ ) => assert!(true),
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
    fn expression_no_shift_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_shift();
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
    fn expression_shift_operator_left_right_double() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a << b >> c".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_shift();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::ShiftRightExpr( 0, 11, left, symbol, right) => {
                        match &**left {
                            ASTNode::ShiftLeftExpr( 0, 7 , left2 , symbol2 , right2 ) => {
                                match &**left2 {
                                    ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**symbol2 {
                                    Token::PyShiftLeft(2, 4, _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**right2 {
                                    ASTNode::AtomName( 5, 7 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                            },
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyShiftRight(7, 9, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 10, 11 , _ ) => assert!(true),
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
    fn expression_bit_and_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a & b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_and_expr();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::AndExpr( 0, 5, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyBitAnd(2, 3, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 4, 5 , _ ) => assert!(true),
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
    fn expression_no_bit_and_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_and_expr();
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
    fn expression_bit_and_operator_double() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a & b & c".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_and_expr();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::AndExpr( 0, 9, left, symbol, right) => {
                        match &**left {
                            ASTNode::AndExpr( 0, 6 , left2 , symbol2 , right2 ) => {
                                match &**left2 {
                                    ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**symbol2 {
                                    Token::PyBitAnd(2, 3, _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**right2 {
                                    ASTNode::AtomName( 4, 6 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                            },
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyBitAnd(6, 7, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 8, 9 , _ ) => assert!(true),
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
    fn expression_bit_xor_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a ^ b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_xor_expr();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::XorExpr( 0, 5, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyBitXor(2, 3, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 4, 5 , _ ) => assert!(true),
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
    fn expression_no_bit_xor_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_xor_expr();
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
    fn expression_bit_xor_operator_double() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a ^ b ^ c".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_xor_expr();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::XorExpr( 0, 9, left, symbol, right) => {
                        match &**left {
                            ASTNode::XorExpr( 0, 6 , left2 , symbol2 , right2 ) => {
                                match &**left2 {
                                    ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**symbol2 {
                                    Token::PyBitXor(2, 3, _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**right2 {
                                    ASTNode::AtomName( 4, 6 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                            },
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyBitXor(6, 7, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 8, 9 , _ ) => assert!(true),
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
    fn expression_bit_or_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a | b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_expr();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::Expr( 0, 5, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyBitOr(2, 3, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 4, 5 , _ ) => assert!(true),
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
    fn expression_no_bit_or_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_expr();
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
    fn expression_bit_or_operator_double() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a | b | c".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_expr();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::Expr( 0, 9, left, symbol, right) => {
                        match &**left {
                            ASTNode::Expr( 0, 6 , left2 , symbol2 , right2 ) => {
                                match &**left2 {
                                    ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**symbol2 {
                                    Token::PyBitOr(2, 3, _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**right2 {
                                    ASTNode::AtomName( 4, 6 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                            },
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyBitOr(6, 7, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 8, 9 , _ ) => assert!(true),
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
    fn expression_star_expression_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("*b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_star_expr();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::StarExpr( 0, 2, symbol, right) => {
                        match &**symbol {
                            Token::PyMul(0, 1, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 1, 2 , _ ) => assert!(true),
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
    fn expression_comparison_operator_less() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a < b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_comparison();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::LessComparison( 0, 5, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyLess(2, 3, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 4, 5 , _ ) => assert!(true),
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
    fn expression_comparison_operator_less_equal() {
        let mut lexer = Box::new(PythonCoreTokenizer::new("a <= b".to_string()));
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_comparison();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::LessEqualComparison(0, 6, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName(0, 2, _) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyLessEqual(2, 4, _) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName(5, 6, _) => assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            Err(..) => assert!(false)
        }
    }

    #[test]
    fn expression_comparison_operator_equal() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a == b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_comparison();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::EqualComparison( 0, 6, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyEqual(2, 4, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 5, 6 , _ ) => assert!(true),
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
    fn expression_comparison_operator_greater_equal() {
        let mut lexer = Box::new(PythonCoreTokenizer::new("a >= b".to_string()));
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_comparison();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::GreaterEqualComparison(0, 6, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName(0, 2, _) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyGreaterEqual(2, 4, _) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName(5, 6, _) => assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            Err(..) => assert!(false)
        }
    }

    #[test]
    fn expression_comparison_operator_not_equal() {
        let mut lexer = Box::new(PythonCoreTokenizer::new("a != b".to_string()));
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_comparison();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::NotEqualComparison(0, 6, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName(0, 2, _) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyNotEqual(2, 4, _) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName(5, 6, _) => assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            Err(..) => assert!(false)
        }
    }

    #[test]
    fn expression_comparison_operator_not_equal_legacy() {
        let mut lexer = Box::new(PythonCoreTokenizer::new("a <> b".to_string()));
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_comparison();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::NotEqualComparison(0, 6, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName(0, 2, _) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyNotEqual(2, 4, _) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName(5, 6, _) => assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            Err(..) => assert!(false)
        }
    }

    #[test]
    fn expression_comparison_operator_greater() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a > b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_comparison();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::GreaterComparison( 0, 5, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyGreater(2, 3, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 4, 5 , _ ) => assert!(true),
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
    fn expression_comparison_operator_is() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a is b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_comparison();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::IsComparison( 0, 6, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyIs(2, 4, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 5, 6 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            Err( .. ) => assert!(false)
        }

        #[test]
        fn expression_comparison_operator_is_not() {
            let mut lexer = Box::new(PythonCoreTokenizer::new("a is not b".to_string()));
            let mut parser = PythonCoreParser::new(lexer);
            parser.advance();
            let res = parser.parse_expressions_comparison();
            match &res {
                Ok(s) => {
                    match &**s {
                        ASTNode::IsNotComparison(0, 10, left, symbol1, symbol2, right) => {
                            match &**left {
                                ASTNode::AtomName(0, 2, _) => assert!(true),
                                _ => assert!(false)
                            }
                            match &**symbol1 {
                                Token::PyIs(2, 4, _) => assert!(true),
                                _ => assert!(false)
                            }
                            match &**symbol2 {
                                Token::PyNot(5, 9, _) => assert!(true),
                                _ => assert!(false)
                            }
                            match &**right {
                                ASTNode::AtomName(5, 6, _) => assert!(true),
                                _ => assert!(false)
                            }
                        },
                        _ => assert!(false)
                    }
                }
                Err(..) => assert!(false)
            }
        }

        #[test]
        fn expression_comparison_operator_not_in() {
            let mut lexer = Box::new(PythonCoreTokenizer::new("a not in b".to_string()));
            let mut parser = PythonCoreParser::new(lexer);
            parser.advance();
            let res = parser.parse_expressions_comparison();
            match &res {
                Ok(s) => {
                    match &**s {
                        ASTNode::NotInComparison(0, 10, left, symbol1, symbol2, right) => {
                            match &**left {
                                ASTNode::AtomName(0, 2, _) => assert!(true),
                                _ => assert!(false)
                            }
                            match &**symbol1 {
                                Token::PyNot(2, 6, _) => assert!(true),
                                _ => assert!(false)
                            }
                            match &**symbol2 {
                                Token::PyIn(6, 9, _) => assert!(true),
                                _ => assert!(false)
                            }
                            match &**right {
                                ASTNode::AtomName(9, 10, _) => assert!(true),
                                _ => assert!(false)
                            }
                        },
                        _ => assert!(false)
                    }
                }
                Err(..) => assert!(false)
            }
        }
    }

    #[test]
    fn expression_comparison_operator_double() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a < b > c".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_comparison();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::GreaterComparison( 0, 9, left, symbol, right) => {
                        match &**left {
                            ASTNode::LessComparison( 0, 6 , left2 , symbol2 , right2 ) => {
                                match &**left2 {
                                    ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**symbol2 {
                                    Token::PyLess(2, 3, _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**right2 {
                                    ASTNode::AtomName( 4, 6 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                            },
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyGreater(6, 7, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 8, 9 , _ ) => assert!(true),
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
    fn expression_not_test_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("not b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_not_test();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::NotTest( 0, 5, symbol, right) => {
                        match &**symbol {
                            Token::PyNot(0, 3, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 4, 5 , _ ) => assert!(true),
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
    fn expression_no_not_test_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_not_test();
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
    fn expression_and_test_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a and b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_and_test();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::AndTest( 0, 7, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyAnd(2, 5, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 6, 7 , _ ) => assert!(true),
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
    fn expression_comparison_and_test_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a and b and c".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_and_test();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::AndTest( 0, 13, left, symbol, right) => {
                        match &**left {
                            ASTNode::AndTest( 0, 8 , left2 , symbol2 , right2 ) => {
                                match &**left2 {
                                    ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**symbol2 {
                                    Token::PyAnd(2, 5, _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**right2 {
                                    ASTNode::AtomName( 6, 8 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                            },
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyAnd(8, 11, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 12, 13 , _ ) => assert!(true),
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
    fn expression_no_and_test_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_and_test();
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
    fn expression_or_test_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a or b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_or_test();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::OrTest( 0, 6, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyOr(2, 4, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 5, 6 , _ ) => assert!(true),
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
    fn expression_comparison_or_test_operator_multiple() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a or b or c".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_or_test();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::OrTest( 0, 11, left, symbol, right) => {
                        match &**left {
                            ASTNode::OrTest( 0, 7 , left2 , symbol2 , right2 ) => {
                                match &**left2 {
                                    ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**symbol2 {
                                    Token::PyOr(2, 4, _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &**right2 {
                                    ASTNode::AtomName( 5, 7 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                            },
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyOr(7, 9, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 10, 11 , _ ) => assert!(true),
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
    fn expression_no_or_test_operator() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_or_test();
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
    fn expression_named_expression() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a := b".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_named_expression();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::NamedExpr( 0, 6, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyColonAssign(2, 4, _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 5, 6 , _ ) => assert!(true),
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
    fn expression_empty_named_expression() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_named_expression();
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
    fn expression_empty_test_expression() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_test();
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
    fn expression_if_else_test_expression() {
        let mut lexer = Box::new( PythonCoreTokenizer::new("a if b else c".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_test();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::Test( 0, _, left, symbol1, right, symbol2, next ) => {
                       assert!(true)
                    },
                    _ => assert!(false)
                }
            }
            Err( .. ) => assert!(false)
        }
    }
}