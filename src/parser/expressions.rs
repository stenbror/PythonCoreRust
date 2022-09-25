
use crate::{ASTNode, Token };
use crate::parser::parser::{Parser, PythonCoreParser };
use crate::parser::tokenizer::Tokenizer;


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
    fn parse_expressions_testlist_comp(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_trailer(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_subscript_list(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_subscript(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_exprlist(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_testlist(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_dictorset_maker(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_arglist(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_argument(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_comp_iter(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_sync_comp_for(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_comp_for(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_comp_if(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_yield_expr(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_testlist_star_expr(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_var_args_list(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_var_args_assignments(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_expressions_vfp_def(&mut self) -> Result<Box<ASTNode>, String>;
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
                    },
                    Token::PyLeftParen(..) => {
                        let _ = self.advance();
                        let mut right : Option<Box<ASTNode>> = None;
                        match &self.symbol {
                            Ok(s) => {
                                match **s {
                                    Token::PyYield(..) => {
                                        right = Some( self.parse_expressions_yield_expr()? );
                                    },
                                    Token::PyRightParen(..) => { },
                                    _ => {
                                        right = Some( self.parse_expressions_testlist_comp()? );
                                    }
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in atom expression!", start_pos))
                        }
                        match &self.symbol {
                            Ok(s2) => {
                                match **s2 {
                                    Token::PyRightParen(..) => {
                                        let symbol2 = (**s2).clone();
                                        let _ = self.advance();
                                        Ok(Box::new(ASTNode::AtomTuple(start_pos, self.lexer.get_position(), Box::new(symbol1), right, Box::new(symbol2))))
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting ')' in tuple atom expression!", start_pos))
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in atom expression!", start_pos))
                        }
                    },
                    Token::PyLeftBracket(..) => {
                        let _ = self.advance();
                        let mut right : Option<Box<ASTNode>> = None;
                        match &self.symbol {
                            Ok(s) => {
                                match **s {
                                    Token::PyRightBracket(..) => { },
                                    _ => {
                                        right = Some( self.parse_expressions_testlist_comp()? );
                                    }
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in atom expression!", start_pos))
                        }
                        match &self.symbol {
                            Ok(s2) => {
                                match **s2 {
                                    Token::PyRightBracket(..) => {
                                        let symbol2 = (**s2).clone();
                                        let _ = self.advance();
                                        Ok(Box::new(ASTNode::AtomList(start_pos, self.lexer.get_position(), Box::new(symbol1), right, Box::new(symbol2))))
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting ']' in list atom expression!", start_pos))
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in atom expression!", start_pos))
                        }
                    },
                    Token::PyLeftCurly(..) => {
                        let _ = self.advance();
                        let mut right : Option<Box<ASTNode>> = None;
                        match &self.symbol {
                            Ok(s) => {
                                match **s {
                                    Token::PyRightCurly(..) => { },
                                    _ => {
                                        right = Some( self.parse_expressions_dictorset_maker()? );
                                    }
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in atom expression!", start_pos))
                        }
                        match &self.symbol {
                            Ok(s2) => {
                                match **s2 {
                                    Token::PyRightCurly(..) => {
                                        let symbol2 = (**s2).clone();
                                        let _ = self.advance();
                                        match right {
                                            Some( ref a ) => {
                                                match &**a {
                                                    ASTNode::DictionaryContainer(..) => {
                                                        Ok(Box::new(ASTNode::AtomDictionary(start_pos, self.lexer.get_position(), Box::new(symbol1), right, Box::new(symbol2))))
                                                    },
                                                    ASTNode::SetContainer(..) => {
                                                        Ok(Box::new(ASTNode::AtomSet(start_pos, self.lexer.get_position(), Box::new(symbol1), right, Box::new(symbol2))))
                                                    },
                                                    _ => Ok(Box::new(ASTNode::AtomDictionary(start_pos, self.lexer.get_position(), Box::new(symbol1), None, Box::new(symbol2))))
                                                }
                                            },
                                            None => Err(format!("SyntaxError at {}: Expecting dictionary/set in atom expression!", start_pos))
                                        }
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting end marker in dictionary/set atom expression!", start_pos))
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in atom expression!", start_pos))
                        }
                    },
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
        let left_node = self.parse_expressions_atom_expr()?;
        match &self.symbol {
            Ok(s) => {
                match **s {
                    Token::PyPower(..) => {
                        let symbol = (**s).clone();
                        let _ = self.advance();
                        let right_node = self.parse_expressions_factor()?;
                        Ok(Box::new(ASTNode::PowerExpr(start_pos, self.lexer.get_position(), left_node, Box::new(symbol), right_node)))
                    },
                    _ => Ok(left_node)
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in power expression!", start_pos))
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
                        let right_node = self.parse_expressions_factor()?;
                        Ok(Box::new(ASTNode::UnaryPlus(start_pos, self.lexer.get_position(), Box::new(symbol), right_node)))
                    },
                    Token::PyMinus(..) => {
                        let symbol = (**s).clone();
                        let _ = self.advance();
                        let right_node = self.parse_expressions_factor()?;
                        Ok(Box::new(ASTNode::UnaryMinus(start_pos, self.lexer.get_position(), Box::new(symbol), right_node)))
                    },
                    Token::PyBitInvert(..) => {
                        let symbol = (**s).clone();
                        let _ = self.advance();
                        let right_node = self.parse_expressions_factor()?;
                        Ok(Box::new(ASTNode::UnaryInvert(start_pos, self.lexer.get_position(), Box::new(symbol), right_node)))
                    },
                    _ => self.parse_expressions_power()
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in factor expression!", start_pos))
        }
    }

    fn parse_expressions_term(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node = self.parse_expressions_factor()?;
        while   match &self.symbol {
                    Ok(symbol_x) => {
                        let symbol = (**symbol_x).clone();
                        match &symbol {
                            Token::PyMul(..) => {
                                let _ = self.advance();
                                let right_node = self.parse_expressions_factor()?;
                                left_node = Box::new(ASTNode::MulTerm(start_pos, self.lexer.get_position(),left_node.clone(), Box::new(symbol), right_node));
                                true
                            },
                            Token::PyDiv(..) => {
                                let _ = self.advance();
                                let right_node = self.parse_expressions_factor()?;
                                left_node = Box::new(ASTNode::DivTerm(start_pos, self.lexer.get_position(),left_node.clone(), Box::new(symbol), right_node));
                                true
                            },
                            Token::PyFloorDiv(..) => {
                                let _ = self.advance();
                                let right_node = self.parse_expressions_factor()?;
                                left_node = Box::new(ASTNode::FloorDivTerm(start_pos, self.lexer.get_position(),left_node.clone(), Box::new(symbol), right_node));
                                true
                            },
                            Token::PyModulo(..) => {
                                let _ = self.advance();
                                let right_node = self.parse_expressions_factor()?;
                                left_node = Box::new(ASTNode::ModuloTerm(start_pos, self.lexer.get_position(), left_node.clone(), Box::new(symbol), right_node));
                                true
                            },
                            Token::PyMatrice(..) => {
                                let _ = self.advance();
                                let right_node = self.parse_expressions_factor()?;
                                left_node = Box::new(ASTNode::MatriceTerm(start_pos, self.lexer.get_position(),left_node.clone(), Box::new(symbol), right_node));
                                true
                            },
                            _ => false
                        }
                    },
                    _ => return Err(format!("SyntaxError at {}: Expecting symbol in term expression!", start_pos))
                } {};
        Ok(left_node)
    }

    fn parse_expressions_arith(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node = self.parse_expressions_term()?;
        while   match &self.symbol {
            Ok(symbol_x) => {
                let symbol = (**symbol_x).clone();
                match &symbol {
                    Token::PyPlus(..) => {
                        let _ = self.advance();
                        let right_node = self.parse_expressions_term()?;
                        left_node = Box::new(ASTNode::PlusArithExpr(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), right_node));
                        true
                    },
                    Token::PyMinus(..) => {
                        let _ = self.advance();
                        let right_node = self.parse_expressions_term()?;
                        left_node = Box::new(ASTNode::MinusArithExpr(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), right_node));
                        true
                    },
                    _ => false
                }
            },
            _ => return Err(format!("SyntaxError at {}: Expecting symbol in arith expression!", start_pos))
        } {};
        Ok(left_node)
    }

    fn parse_expressions_shift(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node = self.parse_expressions_arith()?;
        while   match &self.symbol {
            Ok(symbol_x) => {
                let symbol = (**symbol_x).clone();
                match &symbol {
                    Token::PyShiftLeft(..) => {
                        let _ = self.advance();
                        let right_node = self.parse_expressions_arith()?;
                        left_node = Box::new(ASTNode::ShiftLeftExpr(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), right_node));
                        true
                    },
                    Token::PyShiftRight(..) => {
                        let _ = self.advance();
                        let right_node = self.parse_expressions_arith()?;
                        left_node = Box::new(ASTNode::ShiftRightExpr(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), right_node));
                        true
                    },
                    _ => false
                }
            },
            _ => return Err(format!("SyntaxError at {}: Expecting symbol in shift expression!", start_pos))
        } {};
        Ok(left_node)
    }

    fn parse_expressions_and_expr(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node = self.parse_expressions_shift()?;
        while   match &self.symbol {
            Ok(symbol_x) => {
                let symbol = (**symbol_x).clone();
                match &symbol {
                    Token::PyBitAnd(..) => {
                        let _ = self.advance();
                        let right_node = self.parse_expressions_shift()?;
                        left_node = Box::new(ASTNode::AndExpr(start_pos, self.lexer.get_position(), left_node.clone(), Box::new(symbol), right_node));
                        true
                    },
                    _ => false
                }
            },
            _ => return Err(format!("SyntaxError at {}: Expecting symbol in and expression!", start_pos))
        } {};
        Ok(left_node)
    }

    fn parse_expressions_xor_expr(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node = self.parse_expressions_and_expr()?;
        while   match &self.symbol {
            Ok(symbol_x) => {
                let symbol = (**symbol_x).clone();
                match &symbol {
                    Token::PyBitXor(..) => {
                        let _ = self.advance();
                        let right_node = self.parse_expressions_and_expr()?;
                        left_node = Box::new(ASTNode::XorExpr(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), right_node));
                        true
                    },
                    _ => false
                }
            },
            _ => return Err(format!("SyntaxError at {}: Expecting symbol in xor expression!", start_pos))
        } {};
        Ok(left_node)
    }

    fn parse_expressions_expr(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node = self.parse_expressions_xor_expr()?;
        while   match &self.symbol {
            Ok(symbol_x) => {
                let symbol = (**symbol_x).clone();
                match &symbol {
                    Token::PyBitOr(..) => {
                        let _ = self.advance();
                        let right_node = self.parse_expressions_xor_expr()?;
                        left_node = Box::new(ASTNode::Expr(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), right_node));
                        true
                    },
                    _ => false
                }
            },
            _ => return Err(format!("SyntaxError at {}: Expecting symbol in or expression!", start_pos))
        } {};
        Ok(left_node)
    }

    fn parse_expressions_star_expr(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s) => {
                match &**s {
                    Token::PyMul(..) => {
                        let symbol = (**s).clone();
                        let _ = self.advance();
                        let right_node = self.parse_expressions_expr()?;
                        Ok(Box::new(ASTNode::StarExpr(start_pos, self.lexer.get_position(), Box::new(symbol), right_node)))
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting '*' in star expression!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in star expression!", start_pos))
        }
    }

    fn parse_expressions_comparison(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node = self.parse_expressions_expr()?;
        while   match &self.symbol {
            Ok(symbol_x) => {
                let symbol = (**symbol_x).clone();
                match &symbol {
                    Token::PyLess(..) => {
                        let _ = self.advance();
                        let right_node = self.parse_expressions_expr()?;
                        left_node = Box::new(ASTNode::LessComparison(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), right_node));
                        true
                    },
                    Token::PyLessEqual(..) => {
                        let _ = self.advance();
                        let right_node = self.parse_expressions_expr()?;
                        left_node = Box::new(ASTNode::LessEqualComparison(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), right_node));
                        true
                    },
                    Token::PyEqual(..) => {
                        let _ = self.advance();
                        let right_node = self.parse_expressions_expr()?;
                        left_node = Box::new(ASTNode::EqualComparison(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), right_node));
                        true
                    },
                    Token::PyGreaterEqual(..) => {
                        let _ = self.advance();
                        let right_node = self.parse_expressions_expr()?;
                        left_node = Box::new(ASTNode::GreaterEqualComparison(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), right_node));
                        true
                    },
                    Token::PyGreater(..) => {
                        let _ = self.advance();
                        let right_node = self.parse_expressions_expr()?;
                        left_node = Box::new(ASTNode::GreaterComparison(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), right_node));
                        true
                    },
                    Token::PyNotEqual(..) => {
                        let _ = self.advance();
                        let right_node = self.parse_expressions_expr()?;
                        left_node = Box::new(ASTNode::NotEqualComparison(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), right_node));
                        true
                    },
                    Token::PyIn(..) => {
                        let _ = self.advance();
                        let right_node = self.parse_expressions_expr()?;
                        left_node = Box::new(ASTNode::InComparison(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), right_node));
                        true
                    },
                    Token::PyIs(..) => {
                        let _ = self.advance();
                        match &self.symbol {
                            Ok(symbol_x) => {
                                let symbol2 = (**symbol_x).clone();
                                match &symbol2 {
                                    Token::PyNot(..) => {
                                        let _ = self.advance();
                                        let right_node = self.parse_expressions_expr()?;
                                        left_node = Box::new(ASTNode::IsNotComparison(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), Box::new(symbol2), right_node));
                                    },
                                    _ => {
                                        let right_node = self.parse_expressions_expr()?;
                                        left_node = Box::new(ASTNode::IsComparison(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), right_node));
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
                                        let right_node = self.parse_expressions_expr()?;
                                        left_node = Box::new(ASTNode::NotInComparison(start_pos, self.lexer.get_position(), left_node.clone(), Box::new(symbol), Box::new(symbol2), right_node));
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
            _ => return Err(format!("SyntaxError at {}: Expecting symbol in comparison expression!", start_pos))
        } {};
        Ok(left_node)
    }

    fn parse_expressions_not_test(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s) => {
                match &**s {
                    Token::PyNot(..) => {
                        let symbol = (**s).clone();
                        let _ = self.advance();
                        let right_node = self.parse_expressions_not_test()?;
                        Ok(Box::new(ASTNode::NotTest(start_pos, self.lexer.get_position(), Box::new(symbol), right_node)))
                    },
                    _ => self.parse_expressions_comparison()
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in not test expression!", start_pos))
        }
    }

    fn parse_expressions_and_test(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node = self.parse_expressions_not_test()?;
        while   match &self.symbol {
            Ok(symbol_x) => {
                let symbol = (**symbol_x).clone();
                match &symbol {
                    Token::PyAnd(..) => {
                        let _ = self.advance();
                        let right_node = self.parse_expressions_not_test()?;
                        left_node = Box::new(ASTNode::AndTest(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), right_node));
                        true
                    },
                    _ => false
                }
            },
            _ => return Err(format!("SyntaxError at {}: Expecting symbol in and test expression!", start_pos))
        } {};
        Ok(left_node)
    }

    fn parse_expressions_or_test(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut left_node = self.parse_expressions_and_test()?;
        while   match &self.symbol {
            Ok(symbol_x) => {
                let symbol = (**symbol_x).clone();
                match &symbol {
                    Token::PyOr(..) => {
                        let _ = self.advance();
                        let right_node = self.parse_expressions_and_test()?;
                        left_node = Box::new(ASTNode::OrTest(start_pos, self.lexer.get_position(), left_node.clone(),Box::new(symbol), right_node));
                        true
                    },
                    _ => false
                }
            },
            _ => return Err(format!("SyntaxError at {}: Expecting symbol in and test expression!", start_pos))
        } {};
        Ok(left_node)
    }

    fn parse_expressions_lambda_def(&mut self, cond: bool) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s1) => {
                let symbol1 = (*s1).clone();
                let _ = self.advance();
                let mut left : Option<Box<ASTNode>> = None;
                match &self.symbol {
                    Ok(s2) => {
                        match &**s2 {
                            Token::PyColon(..) => { },
                            _ => left = Some(self.parse_expressions_var_args_list()?)
                        }
                    },
                    _=> return Err(format!("SyntaxError at {}: Expecting symbol in 'lambda' expression!", self.lexer.get_position()))
                }
                match &self.symbol {
                    Ok(s2) => {
                        match &**s2 {
                            Token::PyColon(..) => {
                                let symbol2 = (*s2).clone();
                                let _ = self.advance();
                                match cond {
                                    true => {
                                        let right = self.parse_expressions_test()?;
                                        Ok(Box::new(ASTNode::Lambda(start_pos, self.lexer.get_position(), symbol1, left, symbol2, right )))
                                    },
                                    _ => {
                                        let right = self.parse_expressions_or_test()?;
                                        Ok(Box::new(ASTNode::Lambda(start_pos, self.lexer.get_position(), symbol1, left, symbol2, right )))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting ':' in 'lambda' expression!", self.lexer.get_position()))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting symbol in 'lambda' expression!", self.lexer.get_position()))
                }
            },
            _=> Err(format!("SyntaxError at {}: Expecting symbol in 'lambda' expression!", self.lexer.get_position()))
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
            _ => Err(format!("SyntaxError at {}: Expecting symbol in non conditional test expression!", self.lexer.get_position()))
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
                                                        Ok(Box::new(ASTNode::Test(start_pos, self.lexer.get_position(), left, Box::new(symbol1.clone()), right, Box::new(symbol2.clone()), next)))
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

    fn parse_expressions_testlist_comp(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        match &self.symbol {
            Ok(s) => {
                match &(**s) {
                    Token::PyMul(..) => {
                        nodes_list.push(self.parse_expressions_star_expr()?)
                    },
                    _ => nodes_list.push(self.parse_expressions_named_expression()?)
                }
            },
            _ => return Err(format!("SyntaxError at {}: Expecting symbol in list expression!", self.lexer.get_position()))
        }
        match &self.symbol {
            Ok(s2) => {
                match &(**s2) {
                    Token::PyFor( .. ) |
                    Token::PyAsync( .. ) => {
                        nodes_list.push( self.parse_expressions_comp_for()? );
                    },
                    Token::PyComa( .. ) => {
                        while match &self.symbol {
                            Ok(s3) => {
                                match &(**s3) {
                                    Token::PyComa( .. ) => {
                                        separators_list.push(Box::new((**s3).clone()));
                                        let _ = self.advance();

                                        match &self.symbol {
                                            Ok(s4) => {
                                                match &(**s4) {
                                                    Token::PyMul(..) => {
                                                        nodes_list.push(self.parse_expressions_star_expr()?)
                                                    },
                                                    _ => nodes_list.push(self.parse_expressions_named_expression()?)
                                                }
                                            },
                                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in list expression!", self.lexer.get_position()))
                                        }
                                        true
                                    },
                                    _ => false
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in list expression!", self.lexer.get_position()))
                        } {};
                    },
                    _ => {}
                }
            },
            _ => return Err(format!("SyntaxError at {}: Expecting symbol in list expression!", self.lexer.get_position()))
        }
        nodes_list.reverse();
        separators_list.reverse();
        Ok(Box::new(ASTNode::TestListComp(start_pos, self.lexer.get_position(), nodes_list, separators_list)))
    }

    fn parse_expressions_trailer(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s) => {
                let symbol1 = (**s).clone();
                match &symbol1 {
                    Token::PyLeftParen(..) => {
                        let mut right : Option<Box<ASTNode>> = None;
                        let _ = self.advance();
                        match &self.symbol {
                            Ok(s2) => {
                                match &(**s2) {
                                    Token::PyRightParen(..) => { },
                                    _ => right = Some(self.parse_expressions_subscript_list()?)
                                }
                            },
                            _ => return Err(format!("Syntax Error at {} - Expecting symbol in trailer expression!", self.lexer.get_position()))
                        }
                        match &self.symbol {
                            Ok(s3) => {
                                let symbol2 = (**s3).clone();
                                match &symbol2 {
                                    Token::PyRightParen(..) => {
                                        let _ = self.advance();
                                        Ok(Box::new(ASTNode::CallTrailer(start_pos, self.lexer.get_position(), Box::new(symbol1), right, Box::new(symbol2))))
                                    },
                                    _ => Err(format!("Syntax Error at {} - Expecting ')' in trailer expression!", self.lexer.get_position()))
                                }
                            },
                            _ => Err(format!("Syntax Error at {} - Expecting symbol in trailer expression!", self.lexer.get_position()))
                        }
                    },
                    Token::PyLeftBracket(..) => {
                        let _ = self.advance();
                        let right = self.parse_expressions_subscript_list()?;
                        match &self.symbol {
                            Ok(s3) => {
                                let symbol2 = (**s3).clone();
                                match &symbol2 {
                                    Token::PyRightBracket(..) => {
                                        let _ = self.advance();
                                        Ok(Box::new(ASTNode::IndexTrailer(start_pos, self.lexer.get_position(), Box::new(symbol1), right, Box::new(symbol2))))
                                    },
                                    _ => Err(format!("Syntax Error at {} - Expecting ']' in trailer expression!", self.lexer.get_position()))
                                }
                            },
                            _ => Err(format!("Syntax Error at {} - Expecting symbol in trailer expression!", self.lexer.get_position()))
                        }
                    },
                    Token::PyDot(..) => {
                        let _ = self.advance();
                        match &self.symbol {
                            Ok(s2) => {
                                let symbol2 = (**s2).clone();
                                match &symbol2 {
                                    Token::AtomName(..) => {
                                        let _ = self.advance();
                                        Ok(Box::new(ASTNode::DotNameTrailer(start_pos, self.lexer.get_position(), Box::new(symbol1), Box::new(symbol2))))
                                    },
                                    _ => Err(format!("Syntax Error at {} - Expecting a valid name after '.' in trailer expression!", self.lexer.get_position()))
                                }
                            },
                            _ => Err(format!("Syntax Error at {} - Expecting symbol in trailer expression!", self.lexer.get_position()))
                        }
                    }
                    _ => Err(format!("Syntax Error at {} - Expecting a valid trailer expression!", self.lexer.get_position()))
                }
            },
            _ => Err(format!("Syntax Error at {} - Expecting symbol in trailer expression!", self.lexer.get_position()))
        }
    }

    fn parse_expressions_subscript_list(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        nodes_list.push( self.parse_expressions_subscript()? );
        while
            match &self.symbol {
                Ok(s) => {
                    match &**s {
                        Token::PyComa(..) => {
                            let symbol1 = (**s).clone();
                            separators_list.push( Box::new(symbol1) );
                            let _ = self.advance();
                            nodes_list.push( self.parse_expressions_subscript()? );
                            true
                        },
                        _ => false
                    }
                },
                _ => return Err(format!("Syntax Error at {} - Expecting symbol in subscript list expression!", self.lexer.get_position()))
            } {};
        separators_list.reverse();
        nodes_list.reverse();
        Ok(Box::new(ASTNode::SubscriptList(start_pos, self.lexer.get_position(), nodes_list, separators_list)))
    }

    fn parse_expressions_subscript(&mut self) -> Result<Box<ASTNode>, String> {
        let mut first_node : Option<Box<ASTNode>> = None;
        let mut second_node : Option<Box<ASTNode>> = None;
        let mut third_node : Option<Box<ASTNode>> = None;
        let mut symbol1 : Option<Box<Token>> = None;
        let mut symbol2 : Option<Box<Token>> = None;
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s) => {
                match &**s {
                    Token::PyColon(..) => { },
                    _ => first_node = Some(self.parse_expressions_test()?)
                }
            },
            _ => return Err(format!("Syntax Error at {} - Expecting symbol in subscript expression!", self.lexer.get_position()))
        };
        match &self.symbol.clone() {
            Ok(s2) => {
                match &**s2{
                    Token::PyColon(..) => {
                        symbol1 = Some(Box::new((**s2).clone()));
                        let _ = self.advance();
                        match &self.symbol {
                            Ok(s3) => {
                                match &**s3 {
                                    Token::PyRightBracket(..) |
                                    Token::PyColon(..) => { },
                                    _ => second_node = Some(self.parse_expressions_test()?)
                                }
                            },
                            _ => return Err(format!("Syntax Error at {} - Expecting symbol in subscript expression!", self.lexer.get_position()))
                        };
                        match &self.symbol {
                            Ok(s3) => {
                                match &**s2{
                                    Token::PyColon(..) => {
                                        symbol2 = Some(Box::new((**s3).clone()));
                                        let _ = self.advance();
                                        match &self.symbol {
                                            Ok(s4) => {
                                                match &**s4 {
                                                    Token::PyRightBracket(..) |
                                                    Token::PyColon(..) => { },
                                                    _ => third_node = Some(self.parse_expressions_test()?)
                                                }
                                            },
                                            _ => return Err(format!("Syntax Error at {} - Expecting symbol in subscript expression!", self.lexer.get_position()))
                                        }; //
                                    },
                                    _ => { }
                                }
                            },
                            _ => return Err(format!("Syntax Error at {} - Expecting symbol in subscript expression!", self.lexer.get_position()))
                        };
                    },
                    _ => { }
                }
            },
            _ => return Err(format!("Syntax Error at {} - Expecting symbol in subscript expression!", self.lexer.get_position()))
        };
        Ok(Box::new(ASTNode::Subscript(start_pos, self.lexer.get_position(), first_node, symbol1, second_node, symbol2, third_node)))
    }

    fn parse_expressions_exprlist(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        match &self.symbol {
            Ok(s) => {
                match &(**s) {
                    Token::PyMul(..) => {
                        nodes_list.push(self.parse_expressions_star_expr()?)
                    },
                    _ => nodes_list.push(self.parse_expressions_named_expression()?)
                }
            },
            _ => return Err(format!("SyntaxError at {}: Expecting symbol in list expression!", self.lexer.get_position()))
        };
        while
            match &self.symbol {
                Ok(s) => {
                    match &**s {
                        Token::PyComa(..) => {
                            let symbol1 = (**s).clone();
                            separators_list.push( Box::new(symbol1) );
                            let _ = self.advance();
                            match &self.symbol {
                                Ok(s2) => {
                                    match &(**s2) {
                                        Token::PyIn(..) => false,
                                        Token::PyComa(..) => return Err(format!("SyntaxError at {}: Missing elements between two ',' in list expression!", self.lexer.get_position())),
                                        Token::PyMul(..) => {
                                            nodes_list.push(self.parse_expressions_star_expr()?);
                                            true
                                        },
                                        _ => {
                                            nodes_list.push(self.parse_expressions_named_expression()?);
                                            true
                                        }
                                    }
                                },
                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in list expression!", self.lexer.get_position()))
                            };
                            true
                        },
                        _ => false
                    }
                },
                _ => return Err(format!("Syntax Error at {} - Expecting symbol in subscript list expression!", self.lexer.get_position()))
            } {};
        nodes_list.reverse();
        separators_list.reverse();
        Ok(Box::new(ASTNode::ExprList(start_pos, self.lexer.get_position(), nodes_list, separators_list)))
    }

    fn parse_expressions_testlist(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        nodes_list.push(self.parse_expressions_test()?);
        while
            match &self.symbol {
                Ok(s) => {
                    match &**s {
                        Token::PyComa(..) => {
                            let symbol1 = (**s).clone();
                            separators_list.push( Box::new(symbol1) );
                            let _ = self.advance();
                            match &self.symbol {
                                Ok(s2) => {
                                    match &(**s2) {
                                        Token::Newline(..) |
                                        Token::PySemiColon(..) |
                                        Token::EOF(..) => false,
                                        Token::PyComa(..) => return Err(format!("SyntaxError at {}: Missing elements between two ',' in list expression!", self.lexer.get_position())),
                                        _ => {
                                            nodes_list.push(self.parse_expressions_test()?);
                                            true
                                        }
                                    }
                                },
                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in list expression!", self.lexer.get_position()))
                            };
                            true
                        },
                        _ => false
                    }
                },
                _ => return Err(format!("Syntax Error at {} - Expecting symbol in subscript list expression!", self.lexer.get_position()))
            } {};
        nodes_list.reverse();
        separators_list.reverse();
        Ok(Box::new(ASTNode::TestList(start_pos, self.lexer.get_position(), nodes_list, separators_list)))
    }

    fn parse_expressions_dictorset_maker(&mut self) -> Result<Box<ASTNode>, String> {
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        let mut is_dictionary = true;
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s) => {
                match &**s {
                    Token::PyMul(..) => {
                        is_dictionary = false;
                        let symbol1 = (**s).clone();
                        let _ = self.advance();
                        let right_node = self.parse_expressions_expr()?;
                        nodes_list.push( Box::new( ASTNode::MulSet(start_pos, self.lexer.get_position(), Box::new(symbol1), right_node) ) )
                    },
                    Token::PyPower(..) => {
                        let symbol1 = (**s).clone();
                        let _ = self.advance();
                        let right_node = self.parse_expressions_expr()?;
                        nodes_list.push( Box::new( ASTNode::PowerDictionary(start_pos, self.lexer.get_position(), Box::new(symbol1), right_node) ) )
                    },
                    _ => {
                        let left_node = self.parse_expressions_test()?;
                        match &self.symbol {
                            Ok(s2) => {
                                match &**s2 {
                                    Token::PyColon(..) => {
                                        let symbol2 = (**s2).clone();
                                        let _ = self.advance();
                                        let right_node = self.parse_expressions_test()?;
                                        nodes_list.push( Box::new( ASTNode::DictionaryEntry(start_pos, self.lexer.get_position(), left_node, Box::new(symbol2), right_node) ) );
                                    },
                                    _ => {
                                        is_dictionary = false;
                                        nodes_list.push( left_node )
                                    }
                                }
                            },
                            _ => return Err(format!("Syntax Error at {} - Expecting symbol in dictionary/set expression!", self.lexer.get_position()))
                        }
                    }
                }
            },
            _ => return Err(format!("Syntax Error at {} - Expecting symbol in dictionary/set expression!", self.lexer.get_position()))
        }
        match is_dictionary {
            true => {
                while
                    match &self.symbol {
                        Ok(s) => {
                            match &**s {
                                Token::PyComa(..) => {
                                    let symbol1 = (**s).clone();
                                    separators_list.push( Box::new(symbol1) );
                                    let _ = self.advance();
                                    match &self.symbol {
                                        Ok(s2) => {
                                            match &(**s2) {
                                                Token::PyRightCurly(..) => false,
                                                Token::PyComa(..) => return Err(format!("SyntaxError at {}: Missing elements between two ',' in set list expression!", self.lexer.get_position())),
                                                Token::PyPower(..) => {
                                                    let symbol2 = (**s2).clone();
                                                    let _ = self.advance();
                                                    let right_node = self.parse_expressions_expr()?;
                                                    nodes_list.push( Box::new( ASTNode::PowerDictionary(start_pos, self.lexer.get_position(), Box::new(symbol2), right_node) ) );
                                                    true
                                                },
                                                _ => {
                                                    let left_node = self.parse_expressions_test()?;
                                                    match &self.symbol {
                                                        Ok(s3) => {
                                                            match &(**s3) {
                                                                Token::PyColon(..) => {
                                                                    let symbol3 = (**s3).clone();
                                                                    let _ = self.advance();
                                                                    let right_node = self.parse_expressions_test()?;
                                                                    nodes_list.push( Box::new( ASTNode::DictionaryEntry(start_pos, self.lexer.get_position(), left_node, Box::new(symbol3), right_node) ) );
                                                                    true
                                                                },
                                                                _ => return Err(format!("SyntaxError at {}: Expecting ':' in dictionary entry expression!", self.lexer.get_position()))
                                                            }
                                                        },
                                                        _ => return Err(format!("Syntax Error at {} - Expecting symbol in argument list expression!", self.lexer.get_position()))
                                                    }
                                                }
                                            }
                                        },
                                        _ => return Err(format!("SyntaxError at {}: Expecting symbol in argument list expression!", self.lexer.get_position()))
                                    };
                                    false
                                },
                                _ => false
                            }
                        },
                        _ => return Err(format!("Syntax Error at {} - Expecting symbol in argument list expression!", self.lexer.get_position()))
                    } {};
            },
            false => {
                while
                    match &self.symbol {
                        Ok(s) => {
                            match &**s {
                                Token::PyComa(..) => {
                                    let symbol1 = (**s).clone();
                                    separators_list.push( Box::new(symbol1) );
                                    let _ = self.advance();
                                    match &self.symbol {
                                        Ok(s2) => {
                                            match &(**s2) {
                                                Token::PyRightCurly(..) => false,
                                                Token::PyComa(..) => return Err(format!("SyntaxError at {}: Missing elements between two ',' in set list expression!", self.lexer.get_position())),
                                                Token::PyMul(..) => {
                                                    let symbol2 = (**s2).clone();
                                                    let _ = self.advance();
                                                    let right_node = self.parse_expressions_expr()?;
                                                    nodes_list.push( Box::new( ASTNode::MulSet(start_pos, self.lexer.get_position(), Box::new(symbol2), right_node) ) );
                                                    true
                                                },
                                                _ => {
                                                    nodes_list.push( self.parse_expressions_test()? );
                                                    true
                                                }
                                            }
                                        },
                                        _ => return Err(format!("SyntaxError at {}: Expecting symbol in argument list expression!", self.lexer.get_position()))
                                    };
                                    true
                                },
                                _ => false
                            }
                        },
                        _ => return Err(format!("Syntax Error at {} - Expecting symbol in argument list expression!", self.lexer.get_position()))
                    } {};
            }
        }
        separators_list.reverse();
        nodes_list.reverse();
        match is_dictionary {
            true => {
                Ok(Box::new( ASTNode::DictionaryContainer(start_pos, self.lexer.get_position(), nodes_list, separators_list) ))
            },
            _ => {
                Ok(Box::new( ASTNode::SetContainer(start_pos, self.lexer.get_position(), nodes_list, separators_list) ))
            }
        }
    }

    fn parse_expressions_arglist(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        nodes_list.push(self.parse_expressions_argument()?);
        while
            match &self.symbol {
                Ok(s) => {
                    match &**s {
                        Token::PyComa(..) => {
                            let symbol1 = (**s).clone();
                            separators_list.push( Box::new(symbol1) );
                            let _ = self.advance();
                            match &self.symbol {
                                Ok(s2) => {
                                    match &(**s2) {
                                        Token::PyRightParen(..) => false,
                                        Token::PyComa(..) => return Err(format!("SyntaxError at {}: Missing elements between two ',' in argument list expression!", self.lexer.get_position())),
                                        _ => {
                                            nodes_list.push(self.parse_expressions_argument()?);
                                            true
                                        }
                                    }
                                },
                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in argument list expression!", self.lexer.get_position()))
                            };
                            true
                        },
                        _ => false
                    }
                },
                _ => return Err(format!("Syntax Error at {} - Expecting symbol in argument list expression!", self.lexer.get_position()))
            } {};
        nodes_list.reverse();
        separators_list.reverse();
        Ok(Box::new(ASTNode::ArgList(start_pos, self.lexer.get_position(), nodes_list, separators_list)))
    }

    fn parse_expressions_argument(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s) => {
                match &**s {
                    Token::PyMul(..) |
                    Token::PyPower(..) => {
                        let symbol1 = Some(Box::new((**s).clone()));
                        let _ = self.advance();
                        let right_node = Some( self.parse_expressions_test()? );
                        Ok(Box::new(ASTNode::Argument(start_pos, self.lexer.get_position(), None, symbol1, right_node)))
                    },
                    _ => {
                        let left_node = Some( self.parse_expressions_test()? );
                        match &self.symbol {
                            Ok(s2) => {
                                match &**s2 {
                                    Token::PyFor(..) |
                                    Token::PyAsync(..) => {
                                        let right_node = Some( self.parse_expressions_comp_for()? );
                                        Ok(Box::new(ASTNode::Argument(start_pos, self.lexer.get_position(), left_node, None, right_node)))
                                    },
                                    Token::PyColonAssign(..) |
                                    Token::PyAssign(..) => {
                                        let symbol1 = Some(Box::new((**s2).clone()));
                                        let _ = self.advance();
                                        let right_node = Some( self.parse_expressions_test()? );
                                        Ok(Box::new(ASTNode::Argument(start_pos, self.lexer.get_position(), left_node, symbol1, right_node)))
                                    },
                                    _ => {
                                        Ok(Box::new(ASTNode::Argument(start_pos, self.lexer.get_position(), left_node, None, None)))
                                    }
                                }
                            },
                            _ => Err(format!("Syntax Error at {} - Expecting symbol in argument expression!", self.lexer.get_position()))
                        }
                    }
                }
            },
            _=> Err(format!("Syntax Error at {} - Expecting symbol in argument expression!", self.lexer.get_position()))
        }
    }

    fn parse_expressions_comp_iter(&mut self) -> Result<Box<ASTNode>, String> {
        match &self.symbol {
            Ok(s) => {
                match &**s {
                    Token::PyFor(..) |
                    Token::PyAsync(..) => {
                        self.parse_expressions_comp_for()
                    },
                    Token::PyIf(..) => {
                        self.parse_expressions_comp_if()
                    },
                    _ => Err(format!("Syntax Error at {} - Expecting 'if', 'for' or 'async' in compiter expression!", self.lexer.get_position()))
                }
            },
            _=> Err(format!("Syntax Error at {} - Expecting symbol in compiter expression!", self.lexer.get_position()))
        }
    }

    fn parse_expressions_sync_comp_for(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s1) => {
                match &(**s1) {
                    Token::PyFor(..) => {
                        let symbol1 = Box::new((**s1).clone());
                        let _ = self.advance();
                        let left_node = self.parse_expressions_exprlist()?;
                        match &self.symbol {
                            Ok(s2) => {
                                match &(**s2) {
                                    Token::PyIn(..) => {
                                        let symbol2 = Box::new((**s2).clone());
                                        let _ = self.advance();
                                        let right_node = self.parse_expressions_or_test()?;
                                        match &self.symbol {
                                            Ok(s3) => {
                                                match &(**s3) {
                                                    Token::PyAsync(..) |
                                                    Token::PyFor(..) |
                                                    Token::PyIf(..) => {
                                                        let next_node = Some( self.parse_expressions_comp_iter()? );
                                                        Ok(Box::new(ASTNode::SyncCompForComprehension(start_pos, self.lexer.get_position(), symbol1, left_node, symbol2, right_node, next_node)))
                                                    },
                                                    _ => {
                                                        Ok(Box::new(ASTNode::SyncCompForComprehension(start_pos, self.lexer.get_position(), symbol1, left_node, symbol2, right_node, None)))
                                                    }
                                                }
                                            },
                                            _=> Err(format!("Syntax Error at {} - Expecting symbol in comprehension 'for' expression!", self.lexer.get_position()))
                                        }
                                    },
                                    _=> Err(format!("Syntax Error at {} - Expecting 'in' in comprehension 'for' expression!", self.lexer.get_position()))
                                }
                            },
                            _=> Err(format!("Syntax Error at {} - Expecting 'for' in comprehension 'for' expression!", self.lexer.get_position()))
                        }
                    },
                    _=> Err(format!("Syntax Error at {} - Expecting 'for' in comprehension 'for' expression!", self.lexer.get_position()))
                }
            },
            _=> Err(format!("Syntax Error at {} - Expecting symbol in comprehension 'for' expression!", self.lexer.get_position()))
        }
    }

    fn parse_expressions_comp_for(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s1) => {
                match &(**s1) {
                    Token::PyAsync(..) => {
                        let symbol1 = Box::new((**s1).clone());
                        let _ = self.advance();
                        let right_node = self.parse_expressions_sync_comp_for()?;
                        Ok(Box::new(ASTNode::CompForComprehension(start_pos, self.lexer.get_position(), symbol1, right_node)))
                    },
                    _ => {
                        self.parse_expressions_sync_comp_for()
                    }
                }
            },
            _=> Err(format!("Syntax Error at {} - Expecting symbol in comprehension 'async' expression!", self.lexer.get_position()))
        }
    }

    fn parse_expressions_comp_if(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s1) => {
                match &(**s1) {
                    Token::PyIf(..) => {
                        let symbol1 = Box::new((**s1).clone());
                        let _ = self.advance();
                        let right_node = self.parse_expressions_no_cond_test()?;
                        match &self.symbol {
                            Ok(s2) => {
                                match &(**s2) {
                                    Token::PyAsync( .. ) |
                                    Token::PyFor( .. ) |
                                    Token::PyIf( .. ) => {
                                        let next_node = Some( self.parse_expressions_comp_iter()? );
                                        Ok(Box::new(ASTNode::CompIfComprehension(start_pos, self.lexer.get_position(), symbol1, right_node, next_node)))
                                    },
                                    _ => {
                                        Ok(Box::new(ASTNode::CompIfComprehension(start_pos, self.lexer.get_position(), symbol1, right_node, None)))
                                    }
                                }
                            },
                            _=> Err(format!("Syntax Error at {} - Expecting symbol in comprehension 'if' expression!", self.lexer.get_position()))
                        }
                    },
                    _=> Err(format!("Syntax Error at {} - Expecting 'if' in comprehension 'if' expression!", self.lexer.get_position()))
                }
            },
            _=> Err(format!("Syntax Error at {} - Expecting symbol in comprehension 'if' expression!", self.lexer.get_position()))
        }
    }

    fn parse_expressions_yield_expr(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s1) => {
                match &(**s1) {
                    Token::PyYield(..) => {
                        let symbol1 = Box::new((**s1).clone());
                        let _ = self.advance();
                        match &self.symbol {
                            Ok(s2) => {
                                match &(**s2) {
                                    Token::PyFrom(..) => {
                                        let symbol2 = Box::new((**s2).clone());
                                        let _ = self.advance();
                                        let right_node = self.parse_expressions_test()?;
                                        Ok(Box::new(ASTNode::YieldFromExpr(start_pos, self.lexer.get_position(), symbol1, symbol2, right_node)))
                                    },
                                    _ => {
                                        let right_node = self.parse_expressions_testlist_star_expr()?;
                                        Ok(Box::new(ASTNode::YieldExpr(start_pos, self.lexer.get_position(), symbol1, right_node)))
                                    }
                                }
                            },
                            _=> Err(format!("Syntax Error at {} - Expecting symbol in comprehension 'yield' expression!", self.lexer.get_position()))
                        }
                    },
                    _=> Err(format!("Syntax Error at {} - Expecting symbol in comprehension 'yield' expression!", self.lexer.get_position()))
                }
            },
            _=> Err(format!("Syntax Error at {} - Expecting symbol in comprehension 'yield' expression!", self.lexer.get_position()))
        }
    }

    fn parse_expressions_testlist_star_expr(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        match &self.symbol {
            Ok(s) => {
                match &(**s) {
                    Token::PyMul(..) => {
                        nodes_list.push(self.parse_expressions_star_expr()?)
                    },
                    _ => nodes_list.push(self.parse_expressions_test()?)
                }
            },
            _ => return Err(format!("SyntaxError at {}: Expecting symbol in list expression!", self.lexer.get_position()))
        };
        while
            match &self.symbol {
                Ok(s) => {
                    match &**s {
                        Token::PyComa(..) => {
                            let symbol1 = (**s).clone();
                            separators_list.push( Box::new(symbol1) );
                            let _ = self.advance();
                            match &self.symbol {
                                Ok(s2) => {
                                    match &(**s2) {
                                        Token::PyPlusAssign( .. ) |
                                        Token::PyMinusAssign( .. ) |
                                        Token::PyMulAssign( .. ) |
                                        Token::PyPowerAssign( .. ) |
                                        Token::PyModuloAssign( .. ) |
                                        Token::PyMatriceAssign( .. ) |
                                        Token::PyFloorDivAssign( .. ) |
                                        Token::PyDivAssign( .. ) |
                                        Token::PyShiftLeftAssign( .. ) |
                                        Token::PyShiftRightAssign( .. ) |
                                        Token::PyBitAndAssign( .. ) |
                                        Token::PyBitOrAssign( .. ) |
                                        Token::PyBitXorAssign( .. ) |
                                        Token::PyAssign( .. ) |
                                        Token::PySemiColon( .. ) |
                                        Token::Newline( .. ) |
                                        Token::EOF( .. ) |
                                        Token::PyColon( .. ) => false,
                                        Token::PyComa(..) => return Err(format!("SyntaxError at {}: Missing elements between two ',' in list expression!", self.lexer.get_position())),
                                        Token::PyMul(..) => {
                                            nodes_list.push(self.parse_expressions_star_expr()?);
                                            true
                                        },
                                        _ => {
                                            nodes_list.push(self.parse_expressions_test()?);
                                            true
                                        }
                                    }
                                },
                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in list expression!", self.lexer.get_position()))
                            };
                            true
                        },
                        _ => false
                    }
                },
                _ => return Err(format!("Syntax Error at {} - Expecting symbol in subscript list expression!", self.lexer.get_position()))
            } {};
        nodes_list.reverse();
        separators_list.reverse();
        match ( nodes_list.len(), separators_list.len() ) {
            (1, 0) => {
                Ok(nodes_list[0].clone())
            },
            _=> Ok(Box::new(ASTNode::TestListStarExpr(start_pos, self.lexer.get_position(), nodes_list, separators_list)))
        }
    }

    fn parse_expressions_var_args_list(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        let mut mul_symbol : Option<Box<Token>> = None;
        let mut mul_node : Option<Box<ASTNode>> = None;
        let mut power_symbol : Option<Box<Token>> = None;
        let mut power_node : Option<Box<ASTNode>> = None;
        let mut div_symbol : Option<Box<Token>> = None;
        let mut coma_found : bool = false;
        match &self.symbol {
            Ok(s) => {
                match &**s {
                    Token::PyMul(..) => {
                        mul_symbol = Some( s.clone() );
                        let _ = self.advance();
                        mul_node = Some( self.parse_expressions_var_args_assignments()?  );
                        while
                            match &self.symbol {
                                Ok(s2) => {
                                    match &**s2 {
                                        Token::PyComa(..) => {
                                            separators_list.push( s2.clone() );
                                            let _ = self.advance();
                                            match &self.symbol {
                                                Ok(s3) => {
                                                    match &**s3 {
                                                        Token::PyPower(..) => {
                                                            power_symbol = Some( s3.clone() );
                                                            let _ = self.advance();
                                                            power_node = Some( self.parse_expressions_var_args_assignments()? );
                                                            match &self.symbol {
                                                                Ok(s4) => {
                                                                    match &**s4 {
                                                                        Token::PyComa(..) => {
                                                                            separators_list.push( s4.clone());
                                                                            let _ = self.advance();
                                                                        },
                                                                        _ => { }
                                                                    }
                                                                },
                                                                _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                                                            }
                                                            false
                                                        },
                                                        _ => {
                                                            nodes_list.push( self.parse_expressions_var_args_assignments()? );
                                                            true
                                                        }
                                                    }
                                                },
                                                _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                                            }
                                        },
                                        _ => false
                                    }
                                },
                                _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                            } {};
                    },
                    Token::PyPower(..) => {
                        power_symbol = Some( s.clone() );
                        let _ = self.advance();
                        power_node = Some( self.parse_expressions_var_args_assignments()? );
                        match &self.symbol {
                            Ok(s2) => {
                                match &**s2 {
                                    Token::PyComa(..) => {
                                        separators_list.push( s2.clone());
                                        let _ = self.advance();
                                    },
                                    _ => { }
                                }
                            },
                            _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                        }
                    },
                    _ => {
                        nodes_list.push( (self.parse_expressions_var_args_assignments()?).clone() );
                        while
                            match &self.symbol {
                                Ok(s5) => {
                                    match &**s5 {
                                        Token::PyComa(..) => {
                                            separators_list.push(s5.clone());
                                            let _ = self.advance();
                                            match &self.symbol {
                                                Ok(s6) => {
                                                    match &**s6 {
                                                        Token::PyDiv( .. ) => {
                                                            div_symbol = Some(s6.clone());
                                                            let _ = self.advance();
                                                            while
                                                                match &self.symbol {
                                                                    Ok(s11) => {
                                                                        match &**s11 {
                                                                            Token::PyComa(..) => {
                                                                                separators_list.push(s11.clone());
                                                                                let _ = self.advance();
                                                                                match &self.symbol {
                                                                                    Ok(s12) => {
                                                                                        match &**s12 {
                                                                                            Token::PyMul( .. ) |
                                                                                            Token::PyPower( .. ) => {
                                                                                                coma_found = true;
                                                                                                false
                                                                                            },
                                                                                            _ => {
                                                                                                nodes_list.push( self.parse_expressions_var_args_assignments()? );
                                                                                                true
                                                                                            }
                                                                                        }
                                                                                    },
                                                                                    _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                                                                                }
                                                                            },
                                                                            _ => false
                                                                        }
                                                                    },
                                                                    _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                                                                } {};

                                                            match &self.symbol {
                                                                Ok(s20) => {
                                                                    match (coma_found, &**s20) {
                                                                        (true, Token::PyMul(..)) => {
                                                                            mul_symbol = Some( s20.clone() );
                                                                            let _ = self.advance();
                                                                            mul_node = Some( self.parse_expressions_var_args_assignments()?  );
                                                                            while
                                                                                match &self.symbol {
                                                                                    Ok(s30) => {
                                                                                        match &**s30 {
                                                                                            Token::PyComa(..) => {
                                                                                                separators_list.push( s30.clone() );
                                                                                                let _ = self.advance();
                                                                                                match &self.symbol {
                                                                                                    Ok(s31) => {
                                                                                                        match &**s31 {
                                                                                                            Token::PyPower(..) => {
                                                                                                                power_symbol = Some( s31.clone() );
                                                                                                                let _ = self.advance();
                                                                                                                power_node = Some( self.parse_expressions_var_args_assignments()? );
                                                                                                                match &self.symbol {
                                                                                                                    Ok(s32) => {
                                                                                                                        match &**s32 {
                                                                                                                            Token::PyComa(..) => {
                                                                                                                                separators_list.push( s32.clone());
                                                                                                                                let _ = self.advance();
                                                                                                                            },
                                                                                                                            _ => { }
                                                                                                                        }
                                                                                                                    },
                                                                                                                    _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                                                                                                                }
                                                                                                                false
                                                                                                            },
                                                                                                            _ => {
                                                                                                                nodes_list.push( self.parse_expressions_var_args_assignments()? );
                                                                                                                true
                                                                                                            }
                                                                                                        }
                                                                                                    },
                                                                                                    _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                                                                                                }
                                                                                            },
                                                                                            _ => false
                                                                                        }
                                                                                    },
                                                                                    _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                                                                                } {};
                                                                            false
                                                                        },
                                                                        (true, Token::PyPower(..)) => {
                                                                            power_symbol = Some( s20.clone() );
                                                                            let _ = self.advance();
                                                                            power_node = Some( self.parse_expressions_var_args_assignments()? );
                                                                            match &self.symbol {
                                                                                Ok(s21) => {
                                                                                    match &**s21 {
                                                                                        Token::PyComa(..) => {
                                                                                            separators_list.push( s21.clone());
                                                                                            let _ = self.advance();
                                                                                        },
                                                                                        _ => { }
                                                                                    }
                                                                                },
                                                                                _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                                                                            }
                                                                            false
                                                                        },
                                                                        _ => {
                                                                            false
                                                                        }
                                                                    }
                                                                },
                                                                _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                                                            }
                                                        },
                                                        Token::PyMul(..) => {
                                                            mul_symbol = Some( s6.clone() );
                                                            let _ = self.advance();
                                                            mul_node = Some( self.parse_expressions_var_args_assignments()?  );
                                                            while
                                                                match &self.symbol {
                                                                    Ok(s8) => {
                                                                        match &**s8 {
                                                                            Token::PyComa(..) => {
                                                                                separators_list.push( s8.clone() );
                                                                                let _ = self.advance();
                                                                                match &self.symbol {
                                                                                    Ok(s9) => {
                                                                                        match &**s9 {
                                                                                            Token::PyPower(..) => {
                                                                                                power_symbol = Some( s9.clone() );
                                                                                                let _ = self.advance();
                                                                                                power_node = Some( self.parse_expressions_var_args_assignments()? );
                                                                                                match &self.symbol {
                                                                                                    Ok(s10) => {
                                                                                                        match &**s10 {
                                                                                                            Token::PyComa(..) => {
                                                                                                                separators_list.push( s10.clone());
                                                                                                                let _ = self.advance();
                                                                                                            },
                                                                                                            _ => { }
                                                                                                        }
                                                                                                    },
                                                                                                    _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                                                                                                }
                                                                                                false
                                                                                            },
                                                                                            _ => {
                                                                                                nodes_list.push( self.parse_expressions_var_args_assignments()? );
                                                                                                true
                                                                                            }
                                                                                        }
                                                                                    },
                                                                                    _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                                                                                }
                                                                            },
                                                                            _ => false
                                                                        }
                                                                    },
                                                                    _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                                                                } {};
                                                            false
                                                        },
                                                        Token::PyPower(..) => {
                                                            power_symbol = Some( s6.clone() );
                                                            let _ = self.advance();
                                                            power_node = Some( self.parse_expressions_var_args_assignments()? );
                                                            match &self.symbol {
                                                                Ok(s7) => {
                                                                    match &**s7 {
                                                                        Token::PyComa(..) => {
                                                                            separators_list.push( s7.clone());
                                                                            let _ = self.advance();
                                                                        },
                                                                        _ => { }
                                                                    }
                                                                },
                                                                _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                                                            }
                                                            false
                                                        },
                                                        _ => {
                                                            nodes_list.push( self.parse_expressions_var_args_assignments()? );
                                                            true
                                                        }
                                                    }
                                                },
                                                _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                                            }
                                        },
                                        _ => false
                                    }
                                },
                                _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
                            } {};
                    }
                }
            },
            _ => return Err(format!("Syntax Error at {} - Expecting symbol in variable arguments list expression!", self.lexer.get_position()))
        }
        nodes_list.reverse();
        separators_list.reverse();
        Ok(Box::new( ASTNode::VarArgsList(start_pos, self.lexer.get_position(), nodes_list, separators_list, mul_symbol, mul_node, power_symbol, power_node, div_symbol) ))
    }

    fn parse_expressions_var_args_assignments(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let left_node = self.parse_expressions_vfp_def()?;
        match &self.symbol {
            Ok(s) => {
                match &**s {
                   Token::PyAssign(..) => {
                       let symbol1 = (**s).clone();
                       let _ = self.advance();
                       let right_node = self.parse_expressions_test()?;
                       Ok(Box::new(ASTNode::VFPAssign(start_pos, self.lexer.get_position(), left_node, Box::new(symbol1), right_node)))
                   },
                    _ => {
                        Ok(left_node)
                    }
                }
            },
            _ => Err(format!("Syntax Error at {} - Expecting symbol in varargs assignment expression!", self.lexer.get_position()))
        }
    }

    fn parse_expressions_vfp_def(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match &self.symbol {
            Ok(s) => {
                match &**s {
                    Token::AtomName(..) => {
                        let symbol1 = (**s).clone();
                        let _ = self.advance();
                        Ok(Box::new( ASTNode::VFPDef(start_pos, self.lexer.get_position(), Box::new(symbol1) )))
                    },
                    _ => Err(format!("Syntax Error at {} - Expecting name literal in vfpdef expression!", self.lexer.get_position()))
                }
            },
            _ => Err(format!("Syntax Error at {} - Expecting symbol in vfpdef expression!", self.lexer.get_position()))
        }
    }
}


// UnitTests for expression rules /////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{ASTNode, Token};
    use crate::parser::expressions::Expressions;
    use crate::parser::tokenizer::{PythonCoreTokenizer, Tokenizer};
    use crate::parser::trivias::Trivia;
    use crate::parser::parser::{Parser, PythonCoreParser};


    #[test]
    fn expression_atom_ellipsis() {
        let lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("False".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("None".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("True".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("__init__".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("0.32e-4J".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("'Hello, World!'".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("'Hello, World!''123'".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("__init__".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("await __init__".to_string()) ); // 14
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
                                let _symbol_text = &**tok2;
                                let _pattern = Box::new( "__init__".to_string() );
                                match &**tok2 {
                                    Token::AtomName(6, 14, trivia , _pattern) => {
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
        let lexer = Box::new( PythonCoreTokenizer::new("a ** b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("+b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("-b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("~b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a * b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a / b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a % b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a @ b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a // b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a @ b @ c".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a + b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a - b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a + b - c".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a << b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a >> b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a << b >> c".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a & b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a & b & c".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a ^ b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a ^ b ^ c".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a | b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a | b | c".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("*b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a < b".to_string()) );
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
        let lexer = Box::new(PythonCoreTokenizer::new("a <= b".to_string()));
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
        let lexer = Box::new( PythonCoreTokenizer::new("a == b".to_string()) );
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
        let lexer = Box::new(PythonCoreTokenizer::new("a >= b".to_string()));
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
        let lexer = Box::new(PythonCoreTokenizer::new("a != b".to_string()));
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
        let lexer = Box::new(PythonCoreTokenizer::new("a <> b".to_string()));
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
        let lexer = Box::new( PythonCoreTokenizer::new("a > b".to_string()) );
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
        let lexer = Box::new(PythonCoreTokenizer::new("a is b".to_string()));
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_comparison();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::IsComparison(0, 6, left, symbol, right) => {
                        match &**left {
                            ASTNode::AtomName(0, 2, _) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol {
                            Token::PyIs(2, 4, _) => assert!(true),
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
    fn expression_comparison_operator_is_not() {
        let lexer = Box::new(PythonCoreTokenizer::new("a is not b".to_string()));
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
                        match &**symbol2 { // Bug!
                            Token::PyNot(5, 8, _) => assert!(true),
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

    #[test]
    fn expression_comparison_operator_not_in() {
        let lexer = Box::new(PythonCoreTokenizer::new("a not in b".to_string()));
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
                        match &**symbol1 { // Bug!
                            Token::PyNot(2, 5, _) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol2 {
                            Token::PyIn(6, 8, _) => assert!(true),
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

    #[test]
    fn expression_comparison_operator_double() {
        let lexer = Box::new( PythonCoreTokenizer::new("a < b > c".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("not b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a and b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a and b and c".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a or b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a or b or c".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a := b".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("...".to_string()) );
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
        let lexer = Box::new( PythonCoreTokenizer::new("a if b else c".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_test();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::Test( 0, 13, left, symbol1, right, symbol2, next ) => {
                        match &**left {
                            ASTNode::AtomName( 0,  2 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match (&**symbol1).clone() {
                            Token::PyIf(2, 4, _) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::AtomName( 5, 7 , _ ) => assert!(true),
                            _ => assert!(false)
                        }
                        match &(**symbol2).clone() {
                            Token::PyElse(7, 11, _) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**next {
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
    fn expression_lambda_test_expression_no_arguments() {
        let lexer = Box::new( PythonCoreTokenizer::new("lambda: a * a".to_string()) );
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_expressions_test();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::Lambda( 0, 13, symbol1, None, symbol2, right) => {
                        match &**symbol1 {
                            Token::PyLambda(0, 6, None) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**symbol2 {
                            Token::PyColon(6, 7, _) => assert!(true),
                            _ => assert!(false)
                        }
                        match &**right {
                            ASTNode::MulTerm( 8, 13 , _ , _ , _ ) => assert!(true),
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