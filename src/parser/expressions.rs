
use crate::parser::nodes::{ ASTNode };
use crate::parser::tokens::{ Token };
use crate::parser::parser::{ PythonCoreParser };
use std::vec;


trait Expressions {
    fn parse_expression_named_expr(&self) -> Box<ASTNode>;
    fn parse_expression_test(&self) -> Box<ASTNode>;
    fn parse_expression_test_nocond(&self) -> Box<ASTNode>;
    fn parse_expression_lambda_def(&self) -> Box<ASTNode>;
    fn parse_expression_lambda_def_nocond(&self) -> Box<ASTNode>;
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
}

impl Expressions for PythonCoreParser {
    fn parse_expression_named_expr(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        let leftNode = self.parse_expression_test();
        match &*self.lexer.get_symbol() {
            Token::PyColonAssign( _ , _ , _ ) => {
                let symbol = self.lexer.get_symbol();
                &self.lexer.advance();
                let rightNode = self.parse_expression_test();
                let endPos = &self.lexer.get_position();
                Box::new( ASTNode::NamedExpr(*startPos, *endPos, leftNode, symbol, rightNode) )
            },
            _ => leftNode
        }
    }

    fn parse_expression_test(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyLambda( _ , _ , _ ) => { 
                self.parse_expression_lambda_def()
            },
            _ => {
                let leftNode = self.parse_expression_test();
                match &*self.lexer.get_symbol() {
                    Token::PyIf( _ , _ , _ ) => {
                        let symbol1 = self.lexer.get_symbol();
                        &self.lexer.advance();
                        let rightNode = self.parse_expression_or_test();
                        match &*self.lexer.get_symbol() {
                            Token::PyElse( _ , _ , _ ) => {
                                let symbol2 = self.lexer.get_symbol();
                                &self.lexer.advance();
                                let nextNode = self.parse_expression_or_test();
                                let endPos = &self.lexer.get_position();
                                Box::new( ASTNode::Test(*startPos, *endPos, leftNode, symbol1, rightNode, symbol2, nextNode) )
                            },
                            _ => {
                                panic!("Syntax Error at {} - Especting 'else' in test expression!", &self.lexer.get_position())
                            }
                        }
                    },
                    _ => {
                        leftNode
                    }
                }
            }
        }
    }

    fn parse_expression_test_nocond(&self) -> Box<ASTNode> {
        match &*self.lexer.get_symbol() {
            Token::PyLambda( _ , _ , _ ) => {
                self.parse_expression_lambda_def_nocond()
            }
            _ => {
                self.parse_expression_or_test()
            }
        }
    }

    fn parse_expression_lambda_def(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_lambda_def_nocond(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_or_test(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        let mut leftNode = self.parse_expression_and_test();
        match &*self.lexer.get_symbol() {
            Token::PyOr( _ , _ , _ ) => {
                while 
                    match &*self.lexer.get_symbol() {
                        Token::PyOr( _ , _ , _ ) => {
                            let symbol = self.lexer.get_symbol();
                            &self.lexer.advance();
                            let rightNode = self.parse_expression_and_test();
                            let endPos = &self.lexer.get_position();
                            leftNode = Box::new( ASTNode::OrTest(*startPos, *endPos, leftNode, symbol, rightNode) );
                            true
                        },
                        _ => {
                            false
                        }
                    } {};
            },
            _ => {}   
        }
        leftNode
    }

    fn parse_expression_and_test(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        let mut leftNode = self.parse_expression_not_test();
        match &*self.lexer.get_symbol() {
            Token::PyAnd( _ , _ , _ ) => {
                while 
                    match &*self.lexer.get_symbol() {
                        Token::PyAnd( _ , _ , _ ) => {
                            let symbol = self.lexer.get_symbol();
                            &self.lexer.advance();
                            let rightNode = self.parse_expression_not_test();
                            let endPos = &self.lexer.get_position();
                            leftNode = Box::new( ASTNode::AndTest(*startPos, *endPos, leftNode, symbol, rightNode) );
                            true
                        },
                        _ => {
                            false
                        }
                    } {};
            },
            _ => {}   
        }
        leftNode
    }

    fn parse_expression_not_test(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyNot( _ , _ , _ ) => {
                let symbol = self.lexer.get_symbol();
                &self.lexer.advance();
                let rightNode = self.parse_expression_not_test();
                let endPos = &self.lexer.get_position();
                Box::new( ASTNode::NotTest(*startPos, *endPos, symbol, rightNode) )
            },
            _ => {
                self.parse_expression_comparison()
            }
         }
    }

    fn parse_expression_comparison(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        let mut leftNode = self.parse_expression_expr();
        while 
            match &*self.lexer.get_symbol() {
                Token::PyLess( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_expr();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::LessComparison(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                Token::PyLessEqual( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_expr();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::LessEqualComparison(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                Token::PyEqual( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_expr();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::EqualComparison(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                Token::PyGreaterEqual( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_expr();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::GreaterEqualComparison(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                Token::PyGreater( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_expr();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::GreaterComparison(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                Token::PyNotEqual( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_expr();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::NotEqualComparison(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                Token::PyIn( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_expr();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::InComparison(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                Token::PyIs( _ , _ , _ ) => {
                    let symbol1 = self.lexer.get_symbol();
                    &self.lexer.advance();
                    match &*self.lexer.get_symbol() {
                        Token::PyNot(_ , _ , _ ) => {
                            let symbol2 = self.lexer.get_symbol();
                            &self.lexer.advance();
                            let rightNode = self.parse_expression_expr();
                            let endPos = &self.lexer.get_position();
                            leftNode = Box::new( ASTNode::IsNotComparison(*startPos, *endPos, leftNode, symbol1, symbol2, rightNode) );
                            true
                        },
                        _ => {
                            let rightNode = self.parse_expression_expr();
                            let endPos = &self.lexer.get_position();
                            leftNode = Box::new( ASTNode::IsComparison(*startPos, *endPos, leftNode, symbol1, rightNode) );
                            true
                        }
                    }

                },
                Token::PyNot( _ , _ , _ ) => {
                    let symbol1 = self.lexer.get_symbol();
                    &self.lexer.advance();
                    match &*self.lexer.get_symbol() {
                        Token::PyIn(_ , _ , _ ) => {
                            let symbol2 = self.lexer.get_symbol();
                            &self.lexer.advance();
                            let rightNode = self.parse_expression_expr();
                            let endPos = &self.lexer.get_position();
                            leftNode = Box::new( ASTNode::NotInComparison(*startPos, *endPos, leftNode, symbol1, symbol2, rightNode) );
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
        leftNode
    }

    fn parse_expression_star_expr(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyMul( _ , _ , _ ) => {
                let symbol = self.lexer.get_symbol();
                &self.lexer.advance();
                let rightNode = self.parse_expression_expr();
                let endPos = &self.lexer.get_position();
                Box::new( ASTNode::StarExpr(*startPos, *endPos, symbol, rightNode) )
            },
            _ => {
                panic!("Syntax Error at {} - Especting '*' in star expression!", &self.lexer.get_position())
            }
         }
    }

    fn parse_expression_expr(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        let mut leftNode = self.parse_expression_xor_expr();
        while 
            match &*self.lexer.get_symbol() {
                Token::PyBitOr( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_xor_expr();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::Expr(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                _ => {
                    false
                }
            } {};
        leftNode
    }

    fn parse_expression_xor_expr(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        let mut leftNode = self.parse_expression_and_expr();
        while 
            match &*self.lexer.get_symbol() {
                Token::PyBitXor( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_and_expr();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::XorExpr(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                _ => {
                    false
                }
            } {};
        leftNode
    }

    fn parse_expression_and_expr(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        let mut leftNode = self.parse_expression_shift_expr();
        while 
            match &*self.lexer.get_symbol() {
                Token::PyBitAnd( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_shift_expr();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::AndExpr(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                _ => {
                    false
                }
            } {};
        leftNode
    }

    fn parse_expression_shift_expr(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        let mut leftNode = self.parse_expression_arith_expr();
        while 
            match &*self.lexer.get_symbol() {
                Token::PyShiftLeft( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_arith_expr();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::ShiftLeftExpr(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                Token::PyShiftRight( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_arith_expr();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::ShiftRightExpr(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                _ => {
                    false
                }
            } {};
        leftNode
    }

    fn parse_expression_arith_expr(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        let mut leftNode = self.parse_expression_term();
        while 
            match &*self.lexer.get_symbol() {
                Token::PyPlus( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_term();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::PlusArithExpr(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                Token::PyMinus( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_term();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::MinusArithExpr(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                _ => {
                    false
                }
            } {};
        leftNode
    }

    fn parse_expression_term(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        let mut leftNode = self.parse_expression_factor();
        while 
            match &*self.lexer.get_symbol() {
                Token::PyMul( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_factor();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::MulTerm(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                Token::PyDiv( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_factor();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::DivTerm(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                Token::PyFloorDiv( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_factor();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::FloorDivTerm(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                Token::PyModulo( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_factor();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::ModuloTerm(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                Token::PyMatrice( _ , _ , _ ) => {
                    let symbol = self.lexer.get_symbol();
                    &self.lexer.advance();
                    let rightNode = self.parse_expression_factor();
                    let endPos = &self.lexer.get_position();
                    leftNode = Box::new( ASTNode::MatriceTerm(*startPos, *endPos, leftNode, symbol, rightNode) );
                    true
                },
                _ => {
                    false
                }
            } {};
        leftNode
    }

    fn parse_expression_factor(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyPlus( _ , _ , _ ) => {
                let symbol = self.lexer.get_symbol();
                &self.lexer.advance();
                let rightNode = self.parse_expression_factor();
                let endPos = &self.lexer.get_position();
                Box::new( ASTNode::UnaryPlus(*startPos, *endPos, symbol, rightNode) )
            },
            Token::PyMinus( _ , _ , _ ) => {
                let symbol = self.lexer.get_symbol();
                &self.lexer.advance();
                let rightNode = self.parse_expression_factor();
                let endPos = &self.lexer.get_position();
                Box::new( ASTNode::UnaryMinus(*startPos, *endPos, symbol, rightNode) )
            },
            Token::PyBitInvert( _ , _ , _ ) => {
                let symbol = self.lexer.get_symbol();
                &self.lexer.advance();
                let rightNode = self.parse_expression_factor();
                let endPos = &self.lexer.get_position();
                Box::new( ASTNode::UnaryInvert(*startPos, *endPos, symbol, rightNode) )
            },
            _ => {
                self.parse_expression_power()
            }
         }
    }

    fn parse_expression_power(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        let leftNode = self.parse_expression_atom_expr();
        match &*self.lexer.get_symbol() {
            Token::PyPlus( _ , _ , _ ) => {
                let symbol = self.lexer.get_symbol();
                &self.lexer.advance();
                let rightNode = self.parse_expression_factor();
                let endPos = &self.lexer.get_position();
                Box::new( ASTNode::PowerExpr(*startPos, *endPos, leftNode, symbol, rightNode) )
            },
            _ => {
                leftNode
            }
        }
    }

    fn parse_expression_atom_expr(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        let mut awaitNode : Option<Box<Token>> = None;
        match &*self.lexer.get_symbol() {
            Token::PyAwait( _ , _ , _ ) => {
                awaitNode = Some(self.lexer.get_symbol());
                self.lexer.advance();
            },
            _ => {}
        }
        let rightNode = self.parse_expression_atom();
        let mut trailerList : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        while
            match &*self.lexer.get_symbol() {
                Token::PyDot( _ , _ , _ ) |
                Token::PyLeftParen( _ , _ , _ ) |
                Token::PyLeftBracket( _ , _ , _ ) => {
                    trailerList.push( self.parse_expression_trailer()  );
                    true
                },
                _ => {
                    false
                }  
            } {};
        trailerList.reverse();
        let endPos = &self.lexer.get_position();
        match ( &awaitNode, &trailerList.len() ) {
            ( None, 0 ) => {
                rightNode
            },
            _ => {
                Box::new( ASTNode::AtomExpr(*startPos, *endPos, awaitNode, rightNode, trailerList ) )
            }
        }
    }

    fn parse_expression_atom(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyElipsis( _ , _ , _ ) => {
                let symbol = self.lexer.get_symbol();
                &self.lexer.advance();
                let endPos = &self.lexer.get_position();
                Box::new( ASTNode::AtomElipsis(*startPos, *endPos, symbol) )
            },
            Token::PyNone( _ , _ , _ ) => {
                let symbol = self.lexer.get_symbol();
                &self.lexer.advance();
                let endPos = &self.lexer.get_position();
                Box::new( ASTNode::AtomNone(*startPos, *endPos, symbol) )
            },
            Token::PyTrue( _ , _ , _ ) => {
                let symbol = self.lexer.get_symbol();
                &self.lexer.advance();
                let endPos = &self.lexer.get_position();
                Box::new( ASTNode::AtomTrue(*startPos, *endPos, symbol) )
            },
            Token::PyFalse( _ , _ , _ ) => {
                let symbol = self.lexer.get_symbol();
                &self.lexer.advance();
                let endPos = &self.lexer.get_position();
                Box::new( ASTNode::AtomFalse(*startPos, *endPos, symbol) )
            },
            Token::AtomName( _ , _ , _ , _ ) => {
                let symbol = self.lexer.get_symbol();
                &self.lexer.advance();
                let endPos = &self.lexer.get_position();
                Box::new( ASTNode::AtomName(*startPos, *endPos, symbol) )
            },
            Token::AtomNumber( _ , _ , _ , _ ) => {
                let symbol = self.lexer.get_symbol();
                &self.lexer.advance();
                let endPos = &self.lexer.get_position();
                Box::new( ASTNode::AtomNumber(*startPos, *endPos, symbol) )
            },
            Token::AtomString( _ , _ , _ , _ ) => {
                let mut nodes : Box<Vec<Box<Token>>> = Box::new( Vec::new() );
                while
                    match &*self.lexer.get_symbol() {
                        Token::AtomString( _, _ , _ , _ ) => {
                            let symbol = self.lexer.get_symbol();
                            &self.lexer.advance();
                            nodes.push(symbol);
                            true
                        },
                        _ => {
                            false
                        }
                    } {};
                nodes.reverse();
                let endPos = &self.lexer.get_position();
                Box::new( ASTNode::AtomString(*startPos, *endPos, nodes) )
            },
            Token::PyLeftParen( _ , _ , _ ) => {
                let symbol1 = self.lexer.get_symbol();
                &self.lexer.advance();
                let mut right : Option<Box<ASTNode>> = None;
                match &*self.lexer.get_symbol() {
                    Token::PyYield( _ , _ , _ ) => {
                        right = Some( self.parse_expression_yield_expr() );
                    },
                    Token::PyRightParen( _ , _ , _ ) => { },
                    _ => {
                        right = Some( self.parse_expression_testlist_comp() );
                    }
                }
                match &*self.lexer.get_symbol() {
                    Token::PyRightParen( _ , _ , _ ) => {
                        let symbol2 = self.lexer.get_symbol();
                        &self.lexer.advance();
                        let endPos = &self.lexer.get_position();
                        Box::new( ASTNode::AtomTuple(*startPos, *endPos, symbol1, right, symbol2) )
                    },
                    _ => {
                        panic!("Syntax Error at {} - Especting ')' in tupple!", &self.lexer.get_position())
                    }
                }
            },
            Token::PyLeftBracket( _ , _ , _ ) => {
                let symbol1 = self.lexer.get_symbol();
                &self.lexer.advance();
                let mut right : Option<Box<ASTNode>> = None;
                match &*self.lexer.get_symbol() {
                    Token::PyRightBracket( _ , _ , _ ) => { },
                    _ => {
                        right = Some( self.parse_expression_testlist_comp() );
                    }
                }
                match &*self.lexer.get_symbol() {
                    Token::PyRightBracket( _ , _ , _ ) => {
                        let symbol2 = self.lexer.get_symbol();
                        &self.lexer.advance();
                        let endPos = &self.lexer.get_position();
                        Box::new( ASTNode::AtomList(*startPos, *endPos, symbol1, right, symbol2) )
                    },
                    _ => {
                        panic!("Syntax Error at {} - Especting ']' in list!", &self.lexer.get_position())
                    }
                }
            },
            Token::PyLeftCurly( _ , _ , _ ) => {
                let symbol1 = self.lexer.get_symbol();
                &self.lexer.advance();
                let mut right : Option<Box<ASTNode>> = None;
                match &*self.lexer.get_symbol() {
                    Token::PyRightCurly( _ , _ , _ ) => { },
                    _ => {
                        right = Some( self.parse_expression_dictor_set_maker() );
                    }
                }
                match &*self.lexer.get_symbol() {
                    Token::PyRightCurly( _ , _ , _ ) => {
                        let symbol2 = self.lexer.get_symbol();
                        &self.lexer.advance();
                        let endPos = &self.lexer.get_position();
                        match right {
                            Some( ref a ) => {
                                match &**a {
                                    ASTNode::DictionaryContainer( _ , _ , _ , _ ) => {
                                        Box::new( ASTNode::AtomDictionary(*startPos, *endPos, symbol1, right, symbol2) )
                                    },
                                    ASTNode::SetContainer( _ , _ , _ , _ ) => {
                                        Box::new( ASTNode::AtomSet(*startPos, *endPos, symbol1, right, symbol2) )
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
        let startPos = &self.lexer.get_position();
        let mut nodesList : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separatorsList : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        match &*self.lexer.get_symbol() {
            Token::PyMul( _ , _ , _ ) => {
                nodesList.push( self.parse_expression_star_expr() )
            },
            _ => {
                nodesList.push( self.parse_expression_named_expr() )
            }
        }
        match &*self.lexer.get_symbol() {
            Token::PyFor( _ , _ , _ ) |
            Token::PyAsync( _ , _ , _ ) => {
                nodesList.push( self.parse_expression_comp_for() );
            },
            Token::PyComa( _ , _ , _ ) => {
                while 
                    match &*self.lexer.get_symbol() {
                        Token::PyComa( _ , _ , _ ) => {
                            separatorsList.push( self.lexer.get_symbol() );
                            &self.lexer.advance();
                            match &*self.lexer.get_symbol() {
                                Token::PyMul( _ , _ , _ ) => {
                                    nodesList.push( self.parse_expression_star_expr() )
                                },
                                _ => {
                                    nodesList.push( self.parse_expression_named_expr() )
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
        let endPos = &self.lexer.get_position();
        separatorsList.reverse();
        nodesList.reverse();
        Box::new( ASTNode::TestListComp(*startPos, *endPos, nodesList, separatorsList) )
    }

    fn parse_expression_trailer(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyDot( _ , _ , _ ) => {
                let symbol1 = self.lexer.get_symbol();
                &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::AtomName( _ , _ , _ , _ ) => {
                        let symbol2 = self.lexer.get_symbol();
                        &self.lexer.advance();
                        let endPos = &self.lexer.get_position();
                        Box::new( ASTNode::DotNameTrailer(*startPos, *endPos, symbol1, symbol2) )
                    },
                    _ => {
                        panic!("Syntax Error at {} - Expecting a valid name after '.' in trailer expression!", &self.lexer.get_position())
                    }
                }
            },
            Token::PyLeftParen( _ , _ , _ ) => {
                let symbol1 = self.lexer.get_symbol();
                &self.lexer.advance();
                let mut right : Option<Box<ASTNode>> = None;
                match &*self.lexer.get_symbol() {
                    Token::PyRightBracket( _ , _ , _ ) => {}
                    _ => {
                        right = Some(self.parse_expression_subscript_list())
                    }
                }
                match &*self.lexer.get_symbol() {
                    Token::PyRightBracket( _ , _ , _ ) => {
                        let symbol2 = self.lexer.get_symbol();
                        &self.lexer.advance();
                        let endPos = &self.lexer.get_position();
                        Box::new( ASTNode::CallTrailer(*startPos, *endPos, symbol1, right, symbol2) )
                    },
                    _ => {
                        panic!("Syntax Error at {} - Expecting a ')' in trailer expression!", &self.lexer.get_position())
                    }
                }
            },
            Token::PyLeftBracket( _ , _ , _ ) => {
                let symbol1 = self.lexer.get_symbol();
                &self.lexer.advance();
                let right = self.parse_expression_subscript_list();
                match &*self.lexer.get_symbol() {
                    Token::PyRightBracket( _ , _ , _ ) => {
                        let symbol2 = self.lexer.get_symbol();
                        &self.lexer.advance();
                        let endPos = &self.lexer.get_position();
                        Box::new( ASTNode::IndexTrailer(*startPos, *endPos, symbol1, right, symbol2) )
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
        let startPos = &self.lexer.get_position();
        let mut nodesList : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separatorsList : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        nodesList.push( self.parse_expression_subscript() );
        while
            match &*self.lexer.get_symbol() {
                Token::PyComa( _ , _ , _ ) => {
                    separatorsList.push( self.lexer.get_symbol() );
                    &self.lexer.advance();
                    nodesList.push( self.parse_expression_subscript() );
                    true
                },
                _ => {
                    false
                }
            } {};
        nodesList.reverse();
        separatorsList.reverse();
        let endPos = &self.lexer.get_position();
        Box::new( ASTNode::SubscriptList(*startPos, *endPos, nodesList, separatorsList) )
    }

    fn parse_expression_subscript(&self) -> Box<ASTNode> {
        let mut firstNode : Option<Box<ASTNode>> = None;
        let mut secondNode : Option<Box<ASTNode>> = None;
        let mut thirdNode : Option<Box<ASTNode>> = None;
        let mut symbol1 : Option<Box<Token>> = None;
        let mut symbol2 : Option<Box<Token>> = None;
        let startPos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyColon( .. ) => {},
            _ => {
                firstNode = Some( self.parse_expression_test() )
            }
        }
        match &*self.lexer.get_symbol() {
            Token::PyColon( .. ) => {
                symbol1 = Some( self.lexer.get_symbol() );
                &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyRightBracket( .. ) |
                    Token::PyColon( .. ) => {},
                    _ => {
                        secondNode = Some( self.parse_expression_test() )
                    }
                }
                match &*self.lexer.get_symbol() {
                    Token::PyComa( .. ) => {
                        symbol2 = Some( self.lexer.get_symbol() );
                        &self.lexer.advance();
                        match &*self.lexer.get_symbol() {
                            Token::PyRightBracket( .. ) => {},
                            _ => {
                                thirdNode = Some( self.parse_expression_test() )
                            }
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
        let endPos = &self.lexer.get_position();
        Box::new( ASTNode::Subscript(*startPos, *endPos, firstNode, symbol1, secondNode, symbol2, thirdNode) )
    }

    fn parse_expression_expr_list(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        let mut nodesList : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separatorsList : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        match &*self.lexer.get_symbol() {
            Token::PyMul( .. ) => {
                nodesList.push( self.parse_expression_star_expr() )
            },
            _ => {
                nodesList.push( self.parse_expression_named_expr() )
            }
        }
        while
            match &*self.lexer.get_symbol() {
                Token::PyComa( .. ) => {
                    separatorsList.push( self.lexer.get_symbol() );
                    &self.lexer.advance();
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
                                    nodesList.push( self.parse_expression_star_expr() )
                                },
                                _ => {
                                    nodesList.push( self.parse_expression_named_expr() )
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
        let endPos = &self.lexer.get_position();
        Box::new( ASTNode::ExprList(*startPos, *endPos, nodesList, separatorsList) )
    }

    fn parse_expression_test_list(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        let mut nodesList : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separatorsList : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        nodesList.push( self.parse_expression_test() );
        while
            match &*self.lexer.get_symbol() {
                Token::PyComa( .. ) => {
                    separatorsList.push( self.lexer.get_symbol() );
                    &self.lexer.advance();
                    match &*self.lexer.get_symbol() {
                        Token:: Newline( .. ) |
                        Token::PySemiColon( .. ) |
                        Token::EOF( .. ) => {
                            false
                        }
                        _ => {
                            nodesList.push( self.parse_expression_test() );
                            true
                        }
                    }
                },
                _ => {
                    false
                }
            } {};
        let endPos = &self.lexer.get_position();
        Box::new( ASTNode::TestList(*startPos, *endPos, nodesList, separatorsList) )
    }

    fn parse_expression_dictor_set_maker(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_arg_list(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        let mut nodesList : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separatorsList : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        nodesList.push( self.parse_expression_argument() );
        while
            match &*self.lexer.get_symbol() {
                Token::PyComa( .. ) => {
                    separatorsList.push( self.lexer.get_symbol() );
                    &self.lexer.advance();
                    match &*self.lexer.get_symbol() {
                        Token::PyRightParen( .. ) => {
                            false
                        }
                        _ => {
                            nodesList.push( self.parse_expression_argument() );
                            true
                        }
                    }
                },
                _ => {
                    false
                }
            } {};
        let endPos = &self.lexer.get_position();
        Box::new( ASTNode::ArgList(*startPos, *endPos, nodesList, separatorsList) )
    }

    fn parse_expression_argument(&self) -> Box<ASTNode> {
        let startPos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyMul( .. ) |
            Token::PyPower( .. ) => {
                let symbol = Some( self.lexer.get_symbol() );
                &self.lexer.advance();
                let rightNode = Some( self.parse_expression_test() );
                let endPos = &self.lexer.get_position();
                Box::new( ASTNode::Argument(*startPos, *endPos, None, symbol, rightNode) )
            },
            _ => {
                let leftNode = Some( self.parse_expression_test() );
                match &*self.lexer.get_symbol() {
                    Token::PyFor( .. ) |
                    Token::PyAsync( .. ) => {
                        let rightNode = Some( self.parse_expression_comp_for() );
                        let endPos = &self.lexer.get_position();
                        Box::new( ASTNode::Argument(*startPos, *endPos, leftNode, None, rightNode) )
                    },
                    Token::PyColonAssign( .. ) |
                    Token::PyAssign( .. ) => {
                        let symbol = Some( self.lexer.get_symbol() );
                        &self.lexer.advance();
                        let rightNode = Some( self.parse_expression_test() );
                        let endPos = &self.lexer.get_position();
                        Box::new( ASTNode::Argument(*startPos, *endPos, leftNode, symbol, rightNode) )
                    },
                    _ => {
                        let endPos = &self.lexer.get_position();
                        Box::new( ASTNode::Argument(*startPos, *endPos, leftNode, None, None) )
                    }
                }
            }
        }
    }

    fn parse_expression_comp_iter(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_sync_comp_for(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_comp_for(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_comp_if(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }
    
    fn parse_expression_yield_expr(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }
}
