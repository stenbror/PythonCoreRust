
use crate::parser::trivias::{ Trivia };

#[derive(Clone)]
pub enum Token {
    PyFalse(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyNone(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyTrue(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyAnd(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyAs(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyAssert(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyAsync(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyAwait(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyBreak(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyClass(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyContinue(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyDef(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyDel(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyElif(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyElse(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyExcept(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyFinally(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyFor(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyFrom(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyGlobal(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyIf(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyImport(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyIn(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyIs(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyLambda(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyNonLocal(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyNot(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyOr(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyPass(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyRaise(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyReturn(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyTry(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyWhile(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyWith(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyYield(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyPlus(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyMinus(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyMul(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyPower(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyDiv(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyFloorDiv(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyModulo(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyMatrice(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyShiftLeft(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyShiftRight(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyBitAnd(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyBitOr(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyBitXor(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyBitInvert(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyColonAssign(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyLess(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyGreater(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyLessEqual(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyGreaterEqual(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyEqual(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyNotEqual(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyLeftParen(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyLeftBracket(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyLeftCurly(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyRightParen(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyRightBracket(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyRightCurly(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyComa(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyColon(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyDot(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PySemiColon(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyAssign(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyArrow(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyPlusAssign(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyMinusAssign(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyMulAssign(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyDivAssign(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyFloorDivAssign(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyModuloAssign(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyMatriceAssign(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyBitAndAssign(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyBitOrAssign(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyBitXorAssign(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyShiftLeftAssign(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyShiftRightAssign(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyPowerAssign(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    PyElipsis(u32, u32, Option<Box<Vec<Box<Trivia>>>>),
    AtomName(u32, u32, Option<Box<Vec<Box<Trivia>>>>, Box<String>),
    AtomNumber(u32, u32, Option<Box<[Box<Trivia>]>>, Box<String>),
    AtomString(u32, u32, Option<Box<[Box<Trivia>]>>, Box<[Box<String>]>),
    Newline(u32, u32, Option<Box<[Box<Trivia>]>>),
    Indent(Option<Box<[Box<Trivia>]>>),
    Dedent(Option<Box<[Box<Trivia>]>>),
    TypeComment(u32, u32, Box<String>),
    EOF(u32, Option<Box<[Box<Trivia>]>>),
    Empty
} 