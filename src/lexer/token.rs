use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    // Arithmetic
    Add,
    Subtract,
    MultiplyOrPointer,
    Divide,
    Modulo,
    Increment,
    Decrement,

    // Bitwise
    BitwiseAndOrDereference,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,

    // Logical
    LogicalAnd,
    LogicalOr,
    LogicalNot,

    // Comparison
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,

    // Assignment
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    LeftShiftAssign,
    RightShiftAssign,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Keyword {
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Int,
    Long,
    Register,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Identifier,
    Keyword(Keyword),
    Operator(Operator),
    IntegerLiteral,
    FloatLiteral,
    CharLiteral,
    StringLiteral,
    EmptyLiteral,
    Punctuator(char),
    Comment,
    Unknown,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Char(char),
    Empty,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Value,
    pub position: Position,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ type: {:?}, value: {:?}, position: ({}, {}) }}",
            self.token_type, self.value, self.position.row, self.position.col
        )
    }
}

impl Token {
    pub fn new(token_type: TokenType, value: Value, row: usize, col: usize) -> Self {
        Token {
            token_type,
            value,
            position: Position { row, col },
        }
    }
}
