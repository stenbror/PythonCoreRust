
use crate::parser::tokens::{ Token };

#[derive(Clone)]
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
    Expr(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    XorExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    AndExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    ShiftLeftExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    ShiftRightExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    PlusArithExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    MinusArithExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    MulTerm(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    DivTerm(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    FloorDivTerm(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    ModuloTerm(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    MatriceTerm(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    UnaryPlus(u32, u32, Box<Token>, Box<ASTNode>),
    UnaryMinus(u32, u32, Box<Token>, Box<ASTNode>),
    UnaryInvert(u32, u32, Box<Token>, Box<ASTNode>),
    PowerExpr(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    AtomExpr(u32, u32, Option<Box<Token>>, Box<ASTNode>, Box<Vec<Box<ASTNode>>>),
    AtomName(u32, u32, Box<Token>),
    AtomNumber(u32, u32, Box<Token>),
    AtomString(u32, u32, Box<Vec<Box<Token>>>),
    AtomElipsis(u32, u32, Box<Token>),
    AtomNone(u32, u32, Box<Token>),
    AtomTrue(u32, u32, Box<Token>),
    AtomFalse(u32, u32, Box<Token>),
    AtomTuple(u32, u32, Box<Token>, Option<Box<ASTNode>>, Box<Token>),
    AtomList(u32, u32, Box<Token>, Option<Box<ASTNode>>, Box<Token>),
    AtomDictionary(u32, u32, Box<Token>, Option<Box<ASTNode>>, Box<Token>),
    AtomSet(u32, u32, Box<Token>, Option<Box<ASTNode>>, Box<Token>),
    TestListComp(u32, u32, Box<Vec<Box<ASTNode>>>, Box<Vec<Box<Token>>>),
    CallTrailer(u32, u32, Box<Token>, Option<Box<ASTNode>>, Box<Token>),
    IndexTrailer(u32, u32, Box<Token>, Box<ASTNode>, Box<Token>),
    DotNameTrailer(u32, u32, Box<Token>, Box<Token>),
    SubscriptList(u32, u32, Box<Vec<Box<ASTNode>>>, Box<Vec<Box<Token>>>),
    Subscript(u32, u32, Option<Box<ASTNode>>, Option<Box<Token>>, Option<Box<ASTNode>>, Option<Box<Token>>, Option<Box<ASTNode>>),
    ExprList(u32, u32, Box<Vec<Box<ASTNode>>>, Box<Vec<Box<Token>>>),
    TestList(u32, u32, Box<Vec<Box<ASTNode>>>, Box<Vec<Box<Token>>>),
    DictionaryContainer(u32, u32, Box<Vec<Box<ASTNode>>>, Box<Vec<Box<Token>>>),
    DictionaryEntry(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    SetContainer(u32, u32, Box<Vec<Box<ASTNode>>>, Box<Vec<Box<Token>>>),
    MulSet(u32, u32, Box<Token>, Box<ASTNode>),
    PowerDictionary(u32, u32, Box<Token>, Box<ASTNode>),
    ClassDef(u32, u32, Box<Token>, Box<Token>, Option<Box<Token>>, Option<Box<ASTNode>>, Option<Box<Token>>, Box<Token>, Box<ASTNode>),
    ArgList(u32, u32, Box<Vec<Box<ASTNode>>>, Box<Vec<Box<Token>>>),
    Argument(u32, u32, Option<Box<ASTNode>>, Option<Box<Token>>, Option<Box<ASTNode>>),
    SyncCompForComprehension(u32, u32, Box<Token>, Box<ASTNode>, Box<Token>, Box<ASTNode>, Option<Box<ASTNode>>),
    CompForComprehension(u32, u32,  Box<Token>, Box<ASTNode>),
    CompIfComprehension(u32, u32, Box<Token>, Box<ASTNode>, Option<Box<ASTNode>>),
    YieldExpr(u32, u32, Box<Token>, Box<ASTNode>),
    YieldFromExpr(u32, u32, Box<Token>, Box<Token>, Box<ASTNode>),
    FuncBodySuite(u32, u32, Box<Token>, Option<Box<Token>>, Option<Box<Token>>, Box<Token>, Box<Vec<Box<ASTNode>>>, Box<Token>),
    FuncTypeInput(u32, u32, Box<ASTNode>, Box<Vec<Box<Token>>>, Box<Token>),
    FuncType(u32, u32, Box<Token>, Option<Box<ASTNode>>, Box<Token>, Box<Token>, Box<ASTNode>),
    TypeList(u32, u32, Box<Vec<Box<ASTNode>>>, Box<Vec<Box<Token>>>, Option<Box<Token>>, Option<Box<ASTNode>>, Option<Box<Token>>, Option<Box<ASTNode>>),
    TestListStarExpr(u32, u32, Box<Vec<Box<ASTNode>>>, Box<Vec<Box<Token>>>),
    SimpleStmtList(u32, u32, Box<Vec<Box<ASTNode>>>, Box<Vec<Box<Token>>>, Box<Token> ),
    PlusAssignStmt(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    MinusAssignStmt(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    MulAssignStmt(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    DivAssignStmt(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    FloorDivAssignStmt(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    ModuloAssignStmt(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    MatriceAssignStmt(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    PowerAssignStmt(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    ShiftLeftAssignStmt(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    ShiftRightAssignStmt(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    BitOrAssignStmt(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    BitXorAssignStmt(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    BitAndAssignStmt(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    AnnAssignStmt(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>, Option<(Box<Token>, Box<ASTNode>)>),
    AssignmentStmt(u32, u32, Box<ASTNode>, Box<Vec<Box<( Box<Token>, Box<ASTNode> )>>>, Option<Box<Token>>),
    DelStmt(u32, u32, Box<Token>, Box<ASTNode>),
    PassStmt(u32, u32, Box<Token>),
    BreakStmt(u32, u32, Box<Token>),
    ContinueStmt(u32, u32, Box<Token>),
    ReturnStmt(u32, u32, Box<Token>, Option<Box<ASTNode>>),
    RaiseStmt(u32, u32, Box<Token>, Option<(Box<ASTNode>, Option<(Box<Token>, Box<ASTNode>)>)>),
    ImportNameStmt(u32, u32, Box<Token>, Box<ASTNode>),
    ImportFromStmt(u32, u32, Box<Token>, Box<Vec<Box<Token>>>, Option<Box<ASTNode>>, Box<Token>, Option<Box<Token>>, Option<Box<ASTNode>>, Option<Box<Token>>),
    ImportAsName(u32, u32, Box<Token>, Option<(Box<Token>, Box<Token>)>),
    DottedAsNameStmt(u32, u32, Box<ASTNode>, Option<(Box<Token>, Box<Token>)>),
    ImportAsNamesStmt(u32, u32, Box<Vec<Box<ASTNode>>>, Box<Vec<Box<Token>>>),
    DottedAsNamesStmt(u32, u32, Box<Vec<Box<ASTNode>>>, Box<Vec<Box<Token>>>),
    DottedNameStmt(u32, u32, Box<Vec<Box<Token>>>, Box<Vec<Box<Token>>>),
    GlobalStmt(u32, u32, Box<Token>, Box<Vec<Box<Token>>>, Box<Vec<Box<Token>>>),
    NonLocalStmt(u32, u32, Box<Token>, Box<Vec<Box<Token>>>, Box<Vec<Box<Token>>>),
    AssertStmt(u32, u32, Box<Token>, Box<ASTNode>, Option<(Box<Token>, Box<ASTNode>)>),
    AsyncStmt(u32, u32, Box<Token>, Box<ASTNode>),
    IfStmt(u32, u32, Box<Token>, Box<ASTNode>, Box<Token>, Box<ASTNode>, Box<Vec<Box<ASTNode>>>, Option<Box<ASTNode>>),
    ElifStmt(u32, u32, Box<Token>, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    ElseStmt(u32, u32, Box<Token>, Box<Token>, Box<ASTNode>),
    WhileStmt(u32, u32, Box<Token>, Box<ASTNode>, Box<Token>, Box<ASTNode>, Option<Box<ASTNode>>),
    ForStmt(u32, u32, Box<Token>, Box<ASTNode>, Box<Token>, Box<ASTNode>, Box<Token>, Option<Box<Token>>, Box<ASTNode>, Option<Box<ASTNode>>),
    TryStmt(u32, u32, Box<Token>, Box<Token>, Box<ASTNode>, Option<Box<Vec<Box<ASTNode>>>>, Option<Box<ASTNode>>, Option<Box<ASTNode>>),
    FinallyStmt(u32, u32, Box<Token>, Box<Token>, Box<ASTNode>),
    WithStmt(u32, u32, Box<Token>, Option<Box<Token>>, Box<Vec<Box<ASTNode>>>, Box<Vec<Box<Token>>>, Option<Box<Token>>, Box<Token>, Box<ASTNode> ),
    WithItem(u32, u32, Box<ASTNode>, Option<(Box<Token>, Box<ASTNode>)>),
    ExceptClauseStmt(u32, u32, Box<Token>, Option<Box<Token>>, Option<(Box<ASTNode>, Option<(Box<Token>, Box<Token>)>)>),
    ExceptStmt(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    SuiteStmt(u32, u32, Box<Token>, Box<Token>, Box<Vec<Box<ASTNode>>>, Box<Token>),
    Decorator(u32, u32, Box<Token>, Box<ASTNode>, Option<Box<Token>>, Option<Box<ASTNode>>, Option<Box<Token>>, Box<Token>),
    Decorators(u32, u32, Box<Vec<Box<ASTNode>>>),
    Decorated(u32, u32, Box<ASTNode>, Box<ASTNode>),
    FuncDef(u32, u32, Box<Token>, Box<Token>, Option<Box<ASTNode>>, Option<Box<(Box<Token>, Box<ASTNode>)>>, Box<Token>, Option<Box<Token>>, Box<ASTNode>),
    Parameter(u32, u32, Box<Token>, Option<Box<ASTNode>>, Box<Token>),
    TypedArgsList(u32, u32, Box<Vec<Box<ASTNode>>>, Box<Vec<Box<Token>>>, Box<Vec<Box<Token>>>, Option<(Box<Token>, Box<ASTNode>)>, Option<(Box<Token>, Box<ASTNode>)>),
    TFPAssign(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    TFPDef(u32, u32, Box<Token>, Option<Box<(Box<Token>, Box<ASTNode>)>>),
    VarArgsList(u32, u32, Box<Vec<Box<ASTNode>>>, Box<Vec<Box<Token>>>, Option<Box<Token>>, Option<Box<ASTNode>>, Option<Box<Token>>, Option<Box<ASTNode>>, Option<Box<Token>>),
    VFPAssign(u32, u32, Box<ASTNode>, Box<Token>, Box<ASTNode>),
    VFPDef(u32, u32, Box<Token>),
    SingleInput(u32, u32, Option<Box<ASTNode>>, Option<Box<Token>>),
    FileInput(u32, u32, Box<Vec<Box<ASTNode>>>, Box<Vec<Box<Token>>>, Box<Token>),
    EvalInput(u32, u32, Box<ASTNode>, Box<Vec<Box<Token>>>, Box<Token>)
}

