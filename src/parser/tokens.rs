
use crate::parser::trivias::{ Trivia };

pub enum Token {
    PyFalse(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyNone(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyTrue(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyAnd(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyAs(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyAssert(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyAsync(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyAwait(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyBreak(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyClass(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyContinue(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyDef(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyDel(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyElif(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyElse(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyExcept(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyFinally(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyFor(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyFrom(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyGlobal(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyIf(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyImport(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyIn(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyIs(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyLambda(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyNonLocal(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyNot(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyOr(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyPass(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyRaise(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyReturn(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyTry(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyWhile(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyWith(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyYield(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyPlus(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyMinus(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyMul(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyPower(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyDiv(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyFloorDiv(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyModulo(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyMatrice(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyShiftLeft(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyShiftRight(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyBitAnd(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyBitOr(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyBitXor(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyBitInvert(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyColonAssign(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyLess(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyGreater(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyLessEqual(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyGreaterEqual(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyEqual(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyNotEqual(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyLeftParen(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyLeftBracket(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyLeftCurly(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyRightParen(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyRightBracket(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyRightCurly(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyComa(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyColon(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyDot(u32, u32, Option<Box<[Box<Trivia>]>>),
    PySemiColon(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyAssign(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyArrow(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyPlusAssign(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyMinusAssign(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyMulAssign(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyDivAssign(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyFloorDivAssign(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyModuloAssign(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyMatriceAssign(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyBitAndAssign(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyBitOrAssign(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyBitXorAssign(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyShiftLeftAssign(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyShiftRightAssign(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyPowerAssign(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyElipsis(u32, u32, Option<Box<[Box<Trivia>]>>),
    AtomName(u32, u32, Option<Box<[Box<Trivia>]>>, Box<String>),
    AtomNumber(u32, u32, Option<Box<[Box<Trivia>]>>, Box<String>),
    AtomString(u32, u32, Option<Box<[Box<Trivia>]>>, Box<[Box<String>]>),
    Newline(u32, u32, Option<Box<[Box<Trivia>]>>),
    Indent(Option<Box<[Box<Trivia>]>>),
    Dedent(Option<Box<[Box<Trivia>]>>),
    TypeComment(u32, u32, Box<String>),
    EOF(u32, Option<Box<[Box<Trivia>]>>),
    Empty
} 