
use crate::parser::nodes::{ ASTNode };
use crate::parser::tokens::{ Token };
use crate::parser::parser::{ PythonCoreParser };


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
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_factor(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_power(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_atom_expr(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_atom(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_testlist_comp(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_trailer(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_subscript_list(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_subscript(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_expr_list(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_test_list(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_dictor_set_maker(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_arg_list(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_argument(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
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