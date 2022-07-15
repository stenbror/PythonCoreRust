
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
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_test_nocond(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_lambda_def(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_lambda_def_nocond(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
    }

    fn parse_expression_or_test(&self) -> Box<ASTNode> {
        Box::new(ASTNode::Empty)
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
