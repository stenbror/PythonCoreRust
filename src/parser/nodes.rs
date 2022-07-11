
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
    PlusArithExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    MinusArithExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    MulTerm(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    DivTerm(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    FloorDivTerm(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    ModuloTerm(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    MatriceTerm(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    UnaryPlus(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    UnaryMinus(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    UnaryInvert(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    PowerExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<Token>, Box<ASTNode>),
    AtomExpr(u32, u32, Option<Box<Token>>, Box<ASTNode>, Box<[ASTNode]>),
    AtomName(u32, u32, Box<Token>),
    AtomNumber(u32, u32, Box<Token>),
    AtomString(u32, u32, Box<[Token]>),
    AtomElipsis(u32, u32, Box<Token>),
    AtomNone(u32, u32, Box<Token>),
    AtomTrue(u32, u32, Box<Token>),
    AtomFalse(u32, u32, Box<Token>),
    AtomTuple(u32, u32, Box<Token>, Option<Box<ASTNode>>, Box<Token>),
    AtomList(u32, u32, Box<Token>, Option<Box<ASTNode>>, Box<Token>),
    AtomDictionary(u32, u32, Box<Token>, Option<Box<ASTNode>>, Box<Token>),
    AtomSet(u32, u32, Box<Token>, Box<Option<ASTNode>>, Box<Token>)
}

