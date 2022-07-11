
use crate::parser::tokens::{ Token };

pub enum ASTNode {
    NamedExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    Test(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    Lambda(u32, u32, Box<Token>, Option<Box<ASTNode>>, Box<Token>, Box<ASTNode>),
    OrTest(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    AndTest(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    NotTest(u32, u32, Box<Token>, Box<ASTNode>),
    LessComparison(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    LessEqualComparison(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    EqualComparison(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    GreaterComparison(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    GreaterEqualComparison(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    NotEqualComparison(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    InComparison(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    IsComparison(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    NotInComparison(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    IsNotComparison(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    StarExpr(u32, u32, Box<Token>, Box<ASTNode>),
    Expr(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    XorExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    AndExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    ShiftLeftExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    ShiftRightExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    Name(u32, u32, Box<Token>)
}

