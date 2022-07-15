
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
    fn parse_expression_xor_Expr(&self) -> Box<ASTNode>;
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
        let mut leftNode = self.parse_expression_test();
        match &*self.lexer.get_symbol() {
            Token::PyOr( _ , _ , _ ) => {
                while 
                    match &*self.lexer.get_symbol() {
                        Token::PyOr( _ , _ , _ ) => {
                            let symbol = self.lexer.get_symbol();
                            &self.lexer.advance();
                            let rightNode = self.parse_expression_test();
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
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_not_test(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_comparison(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_star_expr(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_expr(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_xor_Expr(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_and_expr(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_shift_expr(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_arith_expr(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
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
