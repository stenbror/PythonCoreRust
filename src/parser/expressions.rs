
use crate::parser::nodes::{ ASTNode };
use crate::parser::tokens::{ Token };
use crate::parser::parser::{ PythonCoreParser };
use std::vec;


pub trait Expressions {
    fn parse_expression_named_expr(&self) -> Box<ASTNode>;
    fn parse_expression_test(&self) -> Box<ASTNode>;
    fn parse_expression_test_nocond(&self) -> Box<ASTNode>;
    fn parse_expression_lambda_def(&self, cond: bool) -> Box<ASTNode>;
    fn parse_expression_or_test(&self) -> Box<ASTNode>;
    fn parse_expression_and_test(&self) -> Box<ASTNode>;
    fn parse_expression_not_test(&self) -> Box<ASTNode>;
    fn parse_expression_comparison(&self) -> Box<ASTNode>;
    fn parse_expression_star_expr(&self) -> Box<ASTNode>;
    fn parse_expression_expr(&self) -> Box<ASTNode>;
    fn parse_expression_xor_expr(&self) -> Box<ASTNode>;
    fn parse_expression_and_expr(&self) -> Box<ASTNode>;
    fn parse_expression_shift_expr(&self) -> Box<ASTNode>;
    fn parse_expression_arith_expr(&self) -> Box<ASTNode>;
    fn parse_expression_term(&self) -> Box<ASTNode>;
    fn parse_expression_factor(&self) -> Box<ASTNode>;
    fn parse_expression_power(&self) -> Box<ASTNode>;
    fn parse_expression_atom_expr(&self) -> Box<ASTNode>;
    fn parse_expression_atom(&self) -> Box<ASTNode>;
    fn parse_expression_testlist_comp(&self) -> Box<ASTNode>;
    fn parse_expression_trailer(&self) -> Box<ASTNode>;
    fn parse_expression_subscript_list(&self) -> Box<ASTNode>;
    fn parse_expression_subscript(&self) -> Box<ASTNode>;
    fn parse_expression_expr_list(&self) -> Box<ASTNode>;
    fn parse_expression_test_list(&self) -> Box<ASTNode>;
    fn parse_expression_dictor_set_maker(&self) -> Box<ASTNode>;
    fn parse_expression_arg_list(&self) -> Box<ASTNode>;
    fn parse_expression_argument(&self) -> Box<ASTNode>;
    fn parse_expression_comp_iter(&self) -> Box<ASTNode>;
    fn parse_expression_sync_comp_for(&self) -> Box<ASTNode>;
    fn parse_expression_comp_for(&self) -> Box<ASTNode>;
    fn parse_expression_comp_if(&self) -> Box<ASTNode>;
    fn parse_expression_yield_expr(&self) -> Box<ASTNode>;
    fn parse_expression_testlist_star_expr(&self) -> Box<ASTNode>;
    fn parse_expression_var_args_list(&self) -> Box<ASTNode>;
    fn parse_expression_var_args_assignment(&self) -> Box<ASTNode>;
    fn parse_expression_vfp_def(&self) -> Box<ASTNode>;
}

impl Expressions for PythonCoreParser {
    fn parse_expression_named_expr(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let left_node = self.parse_expression_test();
        match &*self.lexer.get_symbol() {
            Token::PyColonAssign( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let right_node = self.parse_expression_test();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::NamedExpr(*start_pos, *end_pos, left_node, symbol, right_node) )
            },
            _ => left_node
        }
    }

    fn parse_expression_test(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyLambda( .. ) => { 
                self.parse_expression_lambda_def(true)
            },
            _ => {
                let left_node = self.parse_expression_test();
                match &*self.lexer.get_symbol() {
                    Token::PyIf( .. ) => {
                        let symbol1 = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        let right_node = self.parse_expression_or_test();
                        match &*self.lexer.get_symbol() {
                            Token::PyElse( .. ) => {
                                let symbol2 = self.lexer.get_symbol();
                                let _ = &self.lexer.advance();
                                let next_node = self.parse_expression_or_test();
                                let end_pos = &self.lexer.get_position();
                                Box::new( ASTNode::Test(*start_pos, *end_pos, left_node, symbol1, right_node, symbol2, next_node) )
                            },
                            _ => {
                                panic!("Syntax Error at {} - Especting 'else' in test expression!", &self.lexer.get_position())
                            }
                        }
                    },
                    _ => {
                        left_node
                    }
                }
            }
        }
    }

    fn parse_expression_test_nocond(&self) -> Box<ASTNode> {
        match &*self.lexer.get_symbol() {
            Token::PyLambda( .. ) => {
                self.parse_expression_lambda_def(false)
            }
            _ => {
                self.parse_expression_or_test()
            }
        }
    }

    fn parse_expression_lambda_def(&self, cond: bool) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyLambda( .. ) => {
                let symbol1 = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let mut left_node : Option<Box<ASTNode>> = None;
                match &*self.lexer.get_symbol() {
                    Token::PyColon( .. ) => {},
                    _ => {
                        left_node = Some( self.parse_expression_var_args_list() )
                    }
                }
                match &*self.lexer.get_symbol() {
                    Token::PyColon( .. ) => {
                        let symbol2 = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        let right_node = if cond { self.parse_expression_test() } else { self.parse_expression_test_nocond() };
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::Lambda(*start_pos, *end_pos, symbol1, left_node, symbol2, right_node) )
                    },
                    _ => {
                        panic!("Syntax Error at {} - Expected ':' keyword in lambda expression!", &self.lexer.get_position())
                    }
                }
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'lambda' keyword in lambda expression!", &self.lexer.get_position())
            }
        }
    }

    fn parse_expression_or_test(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut left_node = self.parse_expression_and_test();
        match &*self.lexer.get_symbol() {
            Token::PyOr( .. ) => {
                while 
                    match &*self.lexer.get_symbol() {
                        Token::PyOr( .. ) => {
                            let symbol = self.lexer.get_symbol();
                            let _ = &self.lexer.advance();
                            let right_node = self.parse_expression_and_test();
                            let end_pos = &self.lexer.get_position();
                            left_node = Box::new( ASTNode::OrTest(*start_pos, *end_pos, left_node, symbol, right_node) );
                            true
                        },
                        _ => {
                            false
                        }
                    } {};
            },
            _ => {}   
        }
        left_node
    }

    fn parse_expression_and_test(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut left_node = self.parse_expression_not_test();
        match &*self.lexer.get_symbol() {
            Token::PyAnd( .. ) => {
                while 
                    match &*self.lexer.get_symbol() {
                        Token::PyAnd( .. ) => {
                            let symbol = self.lexer.get_symbol();
                            let _ = &self.lexer.advance();
                            let right_node = self.parse_expression_not_test();
                            let end_pos = &self.lexer.get_position();
                            left_node = Box::new( ASTNode::AndTest(*start_pos, *end_pos, left_node, symbol, right_node) );
                            true
                        },
                        _ => {
                            false
                        }
                    } {};
            },
            _ => {}   
        }
        left_node
    }

    fn parse_expression_not_test(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyNot( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let right_node = self.parse_expression_not_test();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::NotTest(*start_pos, *end_pos, symbol, right_node) )
            },
            _ => {
                self.parse_expression_comparison()
            }
         }
    }

    fn parse_expression_comparison(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut left_node = self.parse_expression_expr();
        while 
            match &*self.lexer.get_symbol() {
                Token::PyLess( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_expr();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::LessComparison(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                Token::PyLessEqual( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_expr();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::LessEqualComparison(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                Token::PyEqual( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_expr();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::EqualComparison(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                Token::PyGreaterEqual( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_expr();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::GreaterEqualComparison(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                Token::PyGreater( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_expr();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::GreaterComparison(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                Token::PyNotEqual( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_expr();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::NotEqualComparison(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                Token::PyIn( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_expr();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::InComparison(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                Token::PyIs( .. ) => {
                    let symbol1 = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    match &*self.lexer.get_symbol() {
                        Token::PyNot(_ , _ , _ ) => {
                            let symbol2 = self.lexer.get_symbol();
                            let _ = &self.lexer.advance();
                            let right_node = self.parse_expression_expr();
                            let end_pos = &self.lexer.get_position();
                            left_node = Box::new( ASTNode::IsNotComparison(*start_pos, *end_pos, left_node, symbol1, symbol2, right_node) );
                            true
                        },
                        _ => {
                            let right_node = self.parse_expression_expr();
                            let end_pos = &self.lexer.get_position();
                            left_node = Box::new( ASTNode::IsComparison(*start_pos, *end_pos, left_node, symbol1, right_node) );
                            true
                        }
                    }

                },
                Token::PyNot( .. ) => {
                    let symbol1 = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    match &*self.lexer.get_symbol() {
                        Token::PyIn(_ , _ , _ ) => {
                            let symbol2 = self.lexer.get_symbol();
                            let _ = &self.lexer.advance();
                            let right_node = self.parse_expression_expr();
                            let end_pos = &self.lexer.get_position();
                            left_node = Box::new( ASTNode::NotInComparison(*start_pos, *end_pos, left_node, symbol1, symbol2, right_node) );
                            true
                        },
                        _ => {
                            panic!("Syntax Error at {} - Especting 'in' in 'not in' expression!", &self.lexer.get_position())
                        }
                    }
                },
                _ => {
                    false
                }
            } {};
        left_node
    }

    fn parse_expression_star_expr(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyMul( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let right_node = self.parse_expression_expr();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::StarExpr(*start_pos, *end_pos, symbol, right_node) )
            },
            _ => {
                panic!("Syntax Error at {} - Especting '*' in star expression!", &self.lexer.get_position())
            }
         }
    }

    fn parse_expression_expr(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut left_node = self.parse_expression_xor_expr();
        while 
            match &*self.lexer.get_symbol() {
                Token::PyBitOr( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_xor_expr();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::Expr(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                _ => {
                    false
                }
            } {};
        left_node
    }

    fn parse_expression_xor_expr(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut left_node = self.parse_expression_and_expr();
        while 
            match &*self.lexer.get_symbol() {
                Token::PyBitXor( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_and_expr();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::XorExpr(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                _ => {
                    false
                }
            } {};
        left_node
    }

    fn parse_expression_and_expr(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut left_node = self.parse_expression_shift_expr();
        while 
            match &*self.lexer.get_symbol() {
                Token::PyBitAnd( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_shift_expr();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::AndExpr(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                _ => {
                    false
                }
            } {};
        left_node
    }

    fn parse_expression_shift_expr(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut left_node = self.parse_expression_arith_expr();
        while 
            match &*self.lexer.get_symbol() {
                Token::PyShiftLeft( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_arith_expr();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::ShiftLeftExpr(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                Token::PyShiftRight( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_arith_expr();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::ShiftRightExpr(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                _ => {
                    false
                }
            } {};
        left_node
    }

    fn parse_expression_arith_expr(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut left_node = self.parse_expression_term();
        while 
            match &*self.lexer.get_symbol() {
                Token::PyPlus( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_term();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::PlusArithExpr(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                Token::PyMinus( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_term();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::MinusArithExpr(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                _ => {
                    false
                }
            } {};
        left_node
    }

    fn parse_expression_term(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut left_node = self.parse_expression_factor();
        while 
            match &*self.lexer.get_symbol() {
                Token::PyMul( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_factor();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::MulTerm(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                Token::PyDiv( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_factor();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::DivTerm(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                Token::PyFloorDiv( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_factor();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::FloorDivTerm(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                Token::PyModulo( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_factor();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::ModuloTerm(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                Token::PyMatrice( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let right_node = self.parse_expression_factor();
                    let end_pos = &self.lexer.get_position();
                    left_node = Box::new( ASTNode::MatriceTerm(*start_pos, *end_pos, left_node, symbol, right_node) );
                    true
                },
                _ => {
                    false
                }
            } {};
        left_node
    }

    fn parse_expression_factor(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyPlus( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let right_node = self.parse_expression_factor();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::UnaryPlus(*start_pos, *end_pos, symbol, right_node) )
            },
            Token::PyMinus( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let right_node = self.parse_expression_factor();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::UnaryMinus(*start_pos, *end_pos, symbol, right_node) )
            },
            Token::PyBitInvert( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let right_node = self.parse_expression_factor();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::UnaryInvert(*start_pos, *end_pos, symbol, right_node) )
            },
            _ => {
                self.parse_expression_power()
            }
         }
    }

    fn parse_expression_power(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let left_node = self.parse_expression_atom_expr();
        match &*self.lexer.get_symbol() {
            Token::PyPlus( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let right_node = self.parse_expression_factor();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::PowerExpr(*start_pos, *end_pos, left_node, symbol, right_node) )
            },
            _ => {
                left_node
            }
        }
    }

    fn parse_expression_atom_expr(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut await_node : Option<Box<Token>> = None;
        match &*self.lexer.get_symbol() {
            Token::PyAwait( .. ) => {
                await_node = Some(self.lexer.get_symbol());
                self.lexer.advance();
            },
            _ => {}
        }
        let right_node = self.parse_expression_atom();
        let mut trailer_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        while
            match &*self.lexer.get_symbol() {
                Token::PyDot( .. ) |
                Token::PyLeftParen( .. ) |
                Token::PyLeftBracket( .. ) => {
                    trailer_list.push( self.parse_expression_trailer()  );
                    true
                },
                _ => {
                    false
                }  
            } {};
        trailer_list.reverse();
        let end_pos = &self.lexer.get_position();
        match ( &await_node, &trailer_list.len() ) {
            ( None, 0 ) => {
                right_node
            },
            _ => {
                Box::new( ASTNode::AtomExpr(*start_pos, *end_pos, await_node, right_node, trailer_list ) )
            }
        }
    }

    fn parse_expression_atom(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyElipsis( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::AtomElipsis(*start_pos, *end_pos, symbol) )
            },
            Token::PyNone( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::AtomNone(*start_pos, *end_pos, symbol) )
            },
            Token::PyTrue( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::AtomTrue(*start_pos, *end_pos, symbol) )
            },
            Token::PyFalse( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::AtomFalse(*start_pos, *end_pos, symbol) )
            },
            Token::AtomName( _ , _ , _ , _ ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::AtomName(*start_pos, *end_pos, symbol) )
            },
            Token::AtomNumber( _ , _ , _ , _ ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::AtomNumber(*start_pos, *end_pos, symbol) )
            },
            Token::AtomString( _ , _ , _ , _ ) => {
                let mut nodes : Box<Vec<Box<Token>>> = Box::new( Vec::new() );
                while
                    match &*self.lexer.get_symbol() {
                        Token::AtomString( _, _ , _ , _ ) => {
                            let symbol = self.lexer.get_symbol();
                            let _ = &self.lexer.advance();
                            nodes.push(symbol);
                            true
                        },
                        _ => {
                            false
                        }
                    } {};
                nodes.reverse();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::AtomString(*start_pos, *end_pos, nodes) )
            },
            Token::PyLeftParen( .. ) => {
                let symbol1 = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let mut right : Option<Box<ASTNode>> = None;
                match &*self.lexer.get_symbol() {
                    Token::PyYield( .. ) => {
                        right = Some( self.parse_expression_yield_expr() );
                    },
                    Token::PyRightParen( .. ) => { },
                    _ => {
                        right = Some( self.parse_expression_testlist_comp() );
                    }
                }
                match &*self.lexer.get_symbol() {
                    Token::PyRightParen( .. ) => {
                        let symbol2 = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::AtomTuple(*start_pos, *end_pos, symbol1, right, symbol2) )
                    },
                    _ => {
                        panic!("Syntax Error at {} - Especting ')' in tupple!", &self.lexer.get_position())
                    }
                }
            },
            Token::PyLeftBracket( .. ) => {
                let symbol1 = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let mut right : Option<Box<ASTNode>> = None;
                match &*self.lexer.get_symbol() {
                    Token::PyRightBracket( .. ) => { },
                    _ => {
                        right = Some( self.parse_expression_testlist_comp() );
                    }
                }
                match &*self.lexer.get_symbol() {
                    Token::PyRightBracket( .. ) => {
                        let symbol2 = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::AtomList(*start_pos, *end_pos, symbol1, right, symbol2) )
                    },
                    _ => {
                        panic!("Syntax Error at {} - Especting ']' in list!", &self.lexer.get_position())
                    }
                }
            },
            Token::PyLeftCurly( .. ) => {
                let symbol1 = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let mut right : Option<Box<ASTNode>> = None;
                match &*self.lexer.get_symbol() {
                    Token::PyRightCurly( .. ) => { },
                    _ => {
                        right = Some( self.parse_expression_dictor_set_maker() );
                    }
                }
                match &*self.lexer.get_symbol() {
                    Token::PyRightCurly( .. ) => {
                        let symbol2 = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        let end_pos = &self.lexer.get_position();
                        match right {
                            Some( ref a ) => {
                                match &**a {
                                    ASTNode::DictionaryContainer( _ , _ , _ , _ ) => {
                                        Box::new( ASTNode::AtomDictionary(*start_pos, *end_pos, symbol1, right, symbol2) )
                                    },
                                    ASTNode::SetContainer( _ , _ , _ , _ ) => {
                                        Box::new( ASTNode::AtomSet(*start_pos, *end_pos, symbol1, right, symbol2) )
                                    },
                                    _ => { 
                                        panic!("Syntax Error at {} - Especting a dictionary or set", &self.lexer.get_position())
                                    }
                                }
                            },
                            None => {
                                panic!("Syntax Error at {} - Especting a dictionary or set", &self.lexer.get_position())
                            }
                        }
                    },
                    _ => {
                        panic!("Syntax Error at {} - Especting end marker in dictionary!", &self.lexer.get_position())
                    }
                }
            },
            _ => {
                panic!("Syntax Error at {} - Especting a valid atom expression!", &self.lexer.get_position())
            }
        }
    }

    fn parse_expression_testlist_comp(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        match &*self.lexer.get_symbol() {
            Token::PyMul( .. ) => {
                nodes_list.push( self.parse_expression_star_expr() )
            },
            _ => {
                nodes_list.push( self.parse_expression_named_expr() )
            }
        }
        match &*self.lexer.get_symbol() {
            Token::PyFor( .. ) |
            Token::PyAsync( .. ) => {
                nodes_list.push( self.parse_expression_comp_for() );
            },
            Token::PyComa( .. ) => {
                while 
                    match &*self.lexer.get_symbol() {
                        Token::PyComa( .. ) => {
                            separators_list.push( self.lexer.get_symbol() );
                            let _ = &self.lexer.advance();
                            match &*self.lexer.get_symbol() {
                                Token::PyMul( .. ) => {
                                    nodes_list.push( self.parse_expression_star_expr() )
                                },
                                _ => {
                                    nodes_list.push( self.parse_expression_named_expr() )
                                }
                            }
                            true
                        },
                        _ => {
                            false
                        }
                    } {};
            },
            _ => {}
        }
        let end_pos = &self.lexer.get_position();
        separators_list.reverse();
        nodes_list.reverse();
        Box::new( ASTNode::TestListComp(*start_pos, *end_pos, nodes_list, separators_list) )
    }

    fn parse_expression_trailer(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyDot( .. ) => {
                let symbol1 = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::AtomName( _ , _ , _ , _ ) => {
                        let symbol2 = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::DotNameTrailer(*start_pos, *end_pos, symbol1, symbol2) )
                    },
                    _ => {
                        panic!("Syntax Error at {} - Expecting a valid name after '.' in trailer expression!", &self.lexer.get_position())
                    }
                }
            },
            Token::PyLeftParen( .. ) => {
                let symbol1 = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let mut right : Option<Box<ASTNode>> = None;
                match &*self.lexer.get_symbol() {
                    Token::PyRightBracket( .. ) => {}
                    _ => {
                        right = Some(self.parse_expression_subscript_list())
                    }
                }
                match &*self.lexer.get_symbol() {
                    Token::PyRightBracket( .. ) => {
                        let symbol2 = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::CallTrailer(*start_pos, *end_pos, symbol1, right, symbol2) )
                    },
                    _ => {
                        panic!("Syntax Error at {} - Expecting a ')' in trailer expression!", &self.lexer.get_position())
                    }
                }
            },
            Token::PyLeftBracket( .. ) => {
                let symbol1 = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let right = self.parse_expression_subscript_list();
                match &*self.lexer.get_symbol() {
                    Token::PyRightBracket( .. ) => {
                        let symbol2 = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::IndexTrailer(*start_pos, *end_pos, symbol1, right, symbol2) )
                    },
                    _ => {
                        panic!("Syntax Error at {} - Expecting a ']' in trailer expression!", &self.lexer.get_position())
                    }
                }
            },
            _ => {
                panic!("Syntax Error at {} - Expecting a valid trailer expression!", &self.lexer.get_position())
            }
        }
    }

    fn parse_expression_subscript_list(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        nodes_list.push( self.parse_expression_subscript() );
        while
            match &*self.lexer.get_symbol() {
                Token::PyComa( .. ) => {
                    separators_list.push( self.lexer.get_symbol() );
                    let _ = &self.lexer.advance();
                    nodes_list.push( self.parse_expression_subscript() );
                    true
                },
                _ => {
                    false
                }
            } {};
        nodes_list.reverse();
        separators_list.reverse();
        let end_pos = &self.lexer.get_position();
        Box::new( ASTNode::SubscriptList(*start_pos, *end_pos, nodes_list, separators_list) )
    }

    fn parse_expression_subscript(&self) -> Box<ASTNode> {
        let mut first_node : Option<Box<ASTNode>> = None;
        let mut second_node : Option<Box<ASTNode>> = None;
        let mut third_node : Option<Box<ASTNode>> = None;
        let mut symbol1 : Option<Box<Token>> = None;
        let mut symbol2 : Option<Box<Token>> = None;
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyColon( .. ) => {},
            _ => {
                first_node = Some( self.parse_expression_test() )
            }
        }
        match &*self.lexer.get_symbol() {
            Token::PyColon( .. ) => {
                symbol1 = Some( self.lexer.get_symbol() );
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyRightBracket( .. ) |
                    Token::PyColon( .. ) => {},
                    _ => {
                        second_node = Some( self.parse_expression_test() )
                    }
                }
                match &*self.lexer.get_symbol() {
                    Token::PyComa( .. ) => {
                        symbol2 = Some( self.lexer.get_symbol() );
                        let _ = &self.lexer.advance();
                        match &*self.lexer.get_symbol() {
                            Token::PyRightBracket( .. ) => {},
                            _ => {
                                third_node = Some( self.parse_expression_test() )
                            }
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
        let end_pos = &self.lexer.get_position();
        Box::new( ASTNode::Subscript(*start_pos, *end_pos, first_node, symbol1, second_node, symbol2, third_node) )
    }

    fn parse_expression_expr_list(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        match &*self.lexer.get_symbol() {
            Token::PyMul( .. ) => {
                nodes_list.push( self.parse_expression_star_expr() )
            },
            _ => {
                nodes_list.push( self.parse_expression_named_expr() )
            }
        }
        while
            match &*self.lexer.get_symbol() {
                Token::PyComa( .. ) => {
                    separators_list.push( self.lexer.get_symbol() );
                    let _ = &self.lexer.advance();
                    match &*self.lexer.get_symbol() {
                        Token::PyIn( .. ) => {
                            false
                        },
                        Token::PyComa( .. ) => {
                            panic!("Syntax Error at {} - Unexpected ',' after allowed ',' in expression list!", &self.lexer.get_position())
                        },
                        _ => {
                            match &*self.lexer.get_symbol() {
                                Token::PyMul( .. ) => {
                                    nodes_list.push( self.parse_expression_star_expr() )
                                },
                                _ => {
                                    nodes_list.push( self.parse_expression_named_expr() )
                                }
                            }
                            true
                        }
                    }
                },
                _ => {
                    false
                }
            } {};
        let end_pos = &self.lexer.get_position();
        Box::new( ASTNode::ExprList(*start_pos, *end_pos, nodes_list, separators_list) )
    }

    fn parse_expression_test_list(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        nodes_list.push( self.parse_expression_test() );
        while
            match &*self.lexer.get_symbol() {
                Token::PyComa( .. ) => {
                    separators_list.push( self.lexer.get_symbol() );
                    let _ = &self.lexer.advance();
                    match &*self.lexer.get_symbol() {
                        Token:: Newline( .. ) |
                        Token::PySemiColon( .. ) |
                        Token::EOF( .. ) => {
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
        let end_pos = &self.lexer.get_position();
        Box::new( ASTNode::TestList(*start_pos, *end_pos, nodes_list, separators_list) )
    }

    fn parse_expression_dictor_set_maker(&self) -> Box<ASTNode> {
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        let mut is_dictionary = true;
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyMul( .. ) => {
                is_dictionary = false;
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let right_node = self.parse_expression_expr();
                let end_pos = &self.lexer.get_position();
                nodes_list.push( Box::new( ASTNode::MulSet(*start_pos, *end_pos, symbol, right_node) ) )
            },
            Token::PyPower( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let right_node = self.parse_expression_expr();
                let end_pos = &self.lexer.get_position();
                nodes_list.push( Box::new( ASTNode::PowerDictionary(*start_pos, *end_pos, symbol, right_node) ) )
            },
            _ => {
                let left_node = self.parse_expression_test();
                match &*self.lexer.get_symbol() {
                    Token::PyColon( .. ) => {
                        let symbol = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        let right_node = self.parse_expression_test();
                        let end_pos = &self.lexer.get_position();
                        let node = Box::new( ASTNode::DictionaryEntry(*start_pos, *end_pos, left_node, symbol, right_node) );
                        nodes_list.push( node );
                    },
                    _ => {
                        is_dictionary = false;
                        nodes_list.push( left_node )
                    }
                }
            }
        }

        match is_dictionary {
            true => {
                while 
                    match &*self.lexer.get_symbol() {
                        Token::PyComa( .. ) => {
                            separators_list.push( self.lexer.get_symbol() );
                            let _ = &self.lexer.advance();
                            match &*self.lexer.get_symbol() {
                                Token::PyPower( .. ) => {
                                    let symbol = self.lexer.get_symbol();
                                    let _ = &self.lexer.advance();
                                    let right_node = self.parse_expression_expr();
                                    let end_pos = &self.lexer.get_position();
                                    nodes_list.push( Box::new( ASTNode::PowerDictionary(*start_pos, *end_pos, symbol, right_node) ) );
                                    true
                                },
                                Token::PyRightCurly( .. ) => {
                                    false
                                }
                                _ => {
                                    let left_node = self.parse_expression_test();
                                    match &*self.lexer.get_symbol() {
                                        Token::PyColon( .. ) => {
                                            let symbol = self.lexer.get_symbol();
                                            let _ = &self.lexer.advance();
                                            let right_node = self.parse_expression_test();
                                            let end_pos = &self.lexer.get_position();
                                            let node = Box::new( ASTNode::DictionaryEntry(*start_pos, *end_pos, left_node, symbol, right_node) );
                                            nodes_list.push( node );
                                        },
                                        _ => {
                                            panic!("Syntax Error at {} - Expected ':' in dictionary entry!", &self.lexer.get_position())
                                        }
                                    }
                                    true
                                }
                            }
                        },
                        _ => {
                            false
                        }
                    } {};
            },
            _ => {
                while
                    match &*self.lexer.get_symbol() {
                        Token::PyComa( .. ) => {
                            separators_list.push( self.lexer.get_symbol() );
                            let _ = &self.lexer.advance();
                            match &*self.lexer.get_symbol() {
                                Token::PyRightCurly( .. ) => {
                                    false
                                },
                                Token::PyMul( .. ) => {
                                    let symbol = self.lexer.get_symbol();
                                    let _ = &self.lexer.advance();
                                    let right_node = self.parse_expression_expr();
                                    let end_pos = &self.lexer.get_position();
                                    nodes_list.push( Box::new( ASTNode::MulSet(*start_pos, *end_pos, symbol, right_node) ) );
                                    true
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
            }
        }

        separators_list.reverse();
        nodes_list.reverse();
        let end_pos = &self.lexer.get_position();
        match is_dictionary {
            true => {
                Box::new( ASTNode::DictionaryContainer(*start_pos, *end_pos, nodes_list, separators_list) )
            },
            _ => {
                Box::new( ASTNode::SetContainer(*start_pos, *end_pos, nodes_list, separators_list) )
            }
        }
    }

    fn parse_expression_arg_list(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        nodes_list.push( self.parse_expression_argument() );
        while
            match &*self.lexer.get_symbol() {
                Token::PyComa( .. ) => {
                    separators_list.push( self.lexer.get_symbol() );
                    let _ = &self.lexer.advance();
                    match &*self.lexer.get_symbol() {
                        Token::PyRightParen( .. ) => {
                            false
                        }
                        _ => {
                            nodes_list.push( self.parse_expression_argument() );
                            true
                        }
                    }
                },
                _ => {
                    false
                }
            } {};
        let end_pos = &self.lexer.get_position();
        Box::new( ASTNode::ArgList(*start_pos, *end_pos, nodes_list, separators_list) )
    }

    fn parse_expression_argument(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyMul( .. ) |
            Token::PyPower( .. ) => {
                let symbol = Some( self.lexer.get_symbol() );
                let _ = &self.lexer.advance();
                let right_node = Some( self.parse_expression_test() );
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::Argument(*start_pos, *end_pos, None, symbol, right_node) )
            },
            _ => {
                let left_node = Some( self.parse_expression_test() );
                match &*self.lexer.get_symbol() {
                    Token::PyFor( .. ) |
                    Token::PyAsync( .. ) => {
                        let right_node = Some( self.parse_expression_comp_for() );
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::Argument(*start_pos, *end_pos, left_node, None, right_node) )
                    },
                    Token::PyColonAssign( .. ) |
                    Token::PyAssign( .. ) => {
                        let symbol = Some( self.lexer.get_symbol() );
                        let _ = &self.lexer.advance();
                        let right_node = Some( self.parse_expression_test() );
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::Argument(*start_pos, *end_pos, left_node, symbol, right_node) )
                    },
                    _ => {
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::Argument(*start_pos, *end_pos, left_node, None, None) )
                    }
                }
            }
        }
    }

    fn parse_expression_comp_iter(&self) -> Box<ASTNode> {
        match &*self.lexer.get_symbol() {
            Token::PyAsync( .. ) |
            Token::PyFor( .. ) => {
                self.parse_expression_comp_for()
            },
            Token::PyIf( .. ) => {
                self.parse_expression_comp_if()
            }
            _ => {
                panic!("Syntax Error at {} - Expected 'async', 'for' or 'if' in comprehension!", &self.lexer.get_position())
            }
        }
    }

    fn parse_expression_sync_comp_for(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyFor( .. ) => {
                let symbol1 = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let left_node = self.parse_expression_expr_list();
                match &*self.lexer.get_symbol() {
                    Token::PyIn( .. ) => {
                        let symbol2 = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        let right_node = self.parse_expression_or_test();
                        match &*self.lexer.get_symbol() {
                            Token::PyAsync( .. ) |
                            Token::PyFor( .. ) |
                            Token::PyIf( .. ) => {
                                let next_node = Some( self.parse_expression_comp_iter() );
                                let end_pos = &self.lexer.get_position();
                                Box::new( ASTNode::SyncCompForComprehension(*start_pos, *end_pos, symbol1, left_node, symbol2, right_node, next_node) )
                            },
                            _ => {
                                let end_pos = &self.lexer.get_position();
                                Box::new( ASTNode::SyncCompForComprehension(*start_pos, *end_pos, symbol1, left_node, symbol2, right_node, None) )
                            }
                        }
                    },
                    _ => {
                        panic!("Syntax Error at {} - Expected 'in' in for comprehension!", &self.lexer.get_position())
                    }
                }
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'for' in for comprehension!", &self.lexer.get_position())
            }
        }
    }

    fn parse_expression_comp_for(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyAsync( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let right_node = self.parse_expression_sync_comp_for();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::CompForComprehension(*start_pos, *end_pos, symbol, right_node) )
            },
            _ => {
                self.parse_expression_sync_comp_for()
            }
        }
    }

    fn parse_expression_comp_if(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyIf( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let right_node = self.parse_expression_test_nocond();
                match &*self.lexer.get_symbol() {
                    Token::PyAsync( .. ) |
                    Token::PyFor( .. ) |
                    Token::PyIf( .. ) => {
                        let next_node = Some( self.parse_expression_comp_iter() );
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::CompIfComprehension(*start_pos, *end_pos, symbol, right_node, next_node) )
                    },
                    _ => {
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::CompIfComprehension(*start_pos, *end_pos, symbol, right_node, None) )
                    }
                }
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'if' in if comprehension!", &self.lexer.get_position())
            }
        }
    }
    
    fn parse_expression_yield_expr(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyYield( .. ) => {
                let symbol1 = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyFrom( .. ) => {
                        let symbol2 = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        let right_node = self.parse_expression_test();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::YieldFromExpr(*start_pos, *end_pos, symbol1, symbol2, right_node) )
                    },
                    _ => {
                        let right_node = self.parse_expression_testlist_star_expr();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::YieldExpr(*start_pos, *end_pos, symbol1, right_node) )
                    }
                }
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'yield' in yield expression!", &self.lexer.get_position())
            }
        }
    }

    fn parse_expression_testlist_star_expr(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        match &*self.lexer.get_symbol() {
            Token::PyMul( .. ) => {
                nodes_list.push( self.parse_expression_star_expr() )
            },
            _ => {
                nodes_list.push( self.parse_expression_test() )
            }
        }
        while
            match &*self.lexer.get_symbol() {
                Token::PyComa( .. ) => {
                    separators_list.push( self.lexer.get_symbol() );
                    let _ = &self.lexer.advance();
                    match &*self.lexer.get_symbol() {
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
                        Token::PyColon( .. ) => {
                            false
                        },
                        Token::PyComa( .. ) => {
                            panic!("Syntax Error at {} - Unexpected ',' after allowed ',' in expression list!", &self.lexer.get_position())
                        },
                        _ => {
                            match &*self.lexer.get_symbol() {
                                Token::PyMul( .. ) => {
                                    nodes_list.push( self.parse_expression_star_expr() )
                                },
                                _ => {
                                    nodes_list.push( self.parse_expression_test() )
                                }
                            }
                            true
                        }
                    }
                },
                _ => {
                    false
                }
            } {};
        let end_pos = &self.lexer.get_position();
        Box::new( ASTNode::TestListStarExpr(*start_pos, *end_pos, nodes_list, separators_list) )
    }

    fn parse_expression_var_args_list(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        let mut mul_symbol : Option<Box<Token>> = None;
        let mut mul_node : Option<Box<ASTNode>> = None;
        let mut power_symbol : Option<Box<Token>> = None;
        let mut power_node : Option<Box<ASTNode>> = None;
        let mut div_symbol : Option<Box<Token>> = None;
        let mut coma_found : bool = false;
        match &*self.lexer.get_symbol() {
            Token::PyMul( .. ) => {
                mul_symbol = Some( self.lexer.get_symbol() );
                let _ = &self.lexer.advance();
                mul_node = Some( self.parse_expression_var_args_assignment() );
                while
                    match &*self.lexer.get_symbol() {
                        Token::PyComa( .. ) => {
                            separators_list.push( self.lexer.get_symbol() );
                            let _ = &self.lexer.advance();
                            match &*self.lexer.get_symbol() {
                                Token::PyPower( .. ) => {
                                    power_symbol = Some( self.lexer.get_symbol() );
                                    let _ = &self.lexer.advance();
                                    power_node = Some( self.parse_expression_var_args_assignment() );
                                    match &*self.lexer.get_symbol() {
                                        Token::PyComa( .. ) => {
                                            separators_list.push( self.lexer.get_symbol() );
                                            let _ = &self.lexer.advance();
                                        },
                                        _ => {}
                                    }
                                    false
                                },
                                _ => {
                                    nodes_list.push( self.parse_expression_var_args_assignment() );
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
                power_node = Some( self.parse_expression_var_args_assignment() );
                match &*self.lexer.get_symbol() {
                    Token::PyComa( .. ) => {
                        separators_list.push( self.lexer.get_symbol() );
                        let _ = &self.lexer.advance();
                    },
                    _ => {}
                }
            },
            _ => {
                nodes_list.push( self.parse_expression_var_args_assignment() );
                while
                    match &*self.lexer.get_symbol() {
                        Token::PyComa( .. ) => {
                            separators_list.push( self.lexer.get_symbol() );
                            let _ = &self.lexer.advance();
                            match &*self.lexer.get_symbol() {
                                Token::PyDiv( .. ) => {
                                    div_symbol = Some( self.lexer.get_symbol() );
                                    let _ = &self.lexer.advance();
                                    while 
                                        match &*self.lexer.get_symbol() {
                                            Token::PyComa( .. ) => {
                                                separators_list.push( self.lexer.get_symbol() );
                                                let _ = &self.lexer.advance();
                                                match &*self.lexer.get_symbol() {
                                                    Token::PyMul( .. ) |
                                                    Token::PyPower( .. ) => {
                                                        coma_found = true;
                                                        false
                                                    },
                                                    _ => {
                                                        nodes_list.push( self.parse_expression_var_args_assignment() );
                                                        true
                                                    }
                                                }
                                            },
                                            _ => {
                                                false
                                            }
                                        } {};
                                    match (coma_found, &*self.lexer.get_symbol()) {
                                        ( true, Token::PyMul( .. ) ) => {
                                            mul_symbol = Some( self.lexer.get_symbol() );
                                            let _ = &self.lexer.advance();
                                            mul_node = Some( self.parse_expression_var_args_assignment() );
                                            while
                                                match &*self.lexer.get_symbol() {
                                                    Token::PyComa( .. ) => {
                                                        separators_list.push( self.lexer.get_symbol() );
                                                        let _ = &self.lexer.advance();
                                                        match &*self.lexer.get_symbol() {
                                                            Token::PyPower( .. ) => {
                                                                power_symbol = Some( self.lexer.get_symbol() );
                                                                let _ = &self.lexer.advance();
                                                                power_node = Some( self.parse_expression_var_args_assignment() );
                                                                match &*self.lexer.get_symbol() {
                                                                    Token::PyComa( .. ) => {
                                                                        separators_list.push( self.lexer.get_symbol() );
                                                                        let _ = &self.lexer.advance();
                                                                    },
                                                                    _ => {}
                                                                }
                                                                false
                                                            },
                                                            _ => {
                                                                nodes_list.push( self.parse_expression_var_args_assignment() );
                                                                true
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        false
                                                    }
                                                } {};
                                            false
                                        },
                                        ( true, Token::PyPower( .. ) ) => {
                                            power_symbol = Some( self.lexer.get_symbol() );
                                            let _ = &self.lexer.advance();
                                            power_node = Some( self.parse_expression_var_args_assignment() );
                                            match &*self.lexer.get_symbol() {
                                                Token::PyComa( .. ) => {
                                                    separators_list.push( self.lexer.get_symbol() );
                                                    let _ = &self.lexer.advance();
                                                },
                                                _ => {}
                                            }
                                            false
                                        },
                                        _ => {
                                            false
                                        }
                                    }
                                },
                                Token::PyMul( .. ) => {
                                    mul_symbol = Some( self.lexer.get_symbol() );
                                    let _ = &self.lexer.advance();
                                    mul_node = Some( self.parse_expression_var_args_assignment() );
                                    while
                                        match &*self.lexer.get_symbol() {
                                            Token::PyComa( .. ) => {
                                                separators_list.push( self.lexer.get_symbol() );
                                                let _ = &self.lexer.advance();
                                                match &*self.lexer.get_symbol() {
                                                    Token::PyPower( .. ) => {
                                                        power_symbol = Some( self.lexer.get_symbol() );
                                                        let _ = &self.lexer.advance();
                                                        power_node = Some( self.parse_expression_var_args_assignment() );
                                                        match &*self.lexer.get_symbol() {
                                                            Token::PyComa( .. ) => {
                                                                separators_list.push( self.lexer.get_symbol() );
                                                                let _ = &self.lexer.advance();
                                                            },
                                                            _ => {}
                                                        }
                                                        false
                                                    },
                                                    _ => {
                                                        nodes_list.push( self.parse_expression_var_args_assignment() );
                                                        true
                                                    }
                                                }
                                            },
                                            _ => {
                                                false
                                            }
                                        } {};
                                    false
                                },
                                Token::PyPower( .. ) => {
                                    power_symbol = Some( self.lexer.get_symbol() );
                                    let _ = &self.lexer.advance();
                                    power_node = Some( self.parse_expression_var_args_assignment() );
                                    match &*self.lexer.get_symbol() {
                                        Token::PyComa( .. ) => {
                                            separators_list.push( self.lexer.get_symbol() );
                                            let _ = &self.lexer.advance();
                                        },
                                        _ => {}
                                    }
                                    false
                                },
                                _ => {
                                    nodes_list.push( self.parse_expression_var_args_assignment() );
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
        Box::new( ASTNode::VarArgsList(*start_pos, *end_pos, nodes_list, separators_list, mul_symbol, mul_node, power_symbol, power_node, div_symbol) )
    }

    fn parse_expression_var_args_assignment(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let left_node = self.parse_expression_vfp_def();
        match &*self.lexer.get_symbol() {
            Token::PyAssign( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let right_node = self.parse_expression_test();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::VFPAssign(*start_pos, *end_pos, left_node, symbol, right_node) )
            },
            _ => {
                left_node
            }
        }
    }

    fn parse_expression_vfp_def(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::AtomName( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::VFPDef(*start_pos, *end_pos, symbol) )
            },
            _ => {
                panic!("Syntax Error at {} - Unexpected name literal in variable args list!", &self.lexer.get_position())
            }
        }
    }
}
