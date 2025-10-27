#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Option<String>,
    pub body: Block,
}

impl Function {
    pub fn new(name: impl Into<String>, params: Vec<Param>, ret_ty: Option<String>, body: Block) -> Self {
        Self {
            name: name.into(),
            params,
            ret_ty,
            body,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Option<String>,
}

impl Param {
    pub fn new(name: impl Into<String>, ty: Option<String>) -> Self {
        Self {
            name: name.into(),
            ty,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    // Variable declarations and assignments
    Let { name: String, expr: Expr },
    Assignment { name: String, expr: Expr },

    // Control flow
    If {
        cond: Box<Expr>,
        then_block: Block,
        elif_blocks: Vec<(Expr, Block)>,
        else_block: Option<Block>,
    },
    For {
        var: String,
        iterable: Expr,
        body: Block,
    },
    While {
        cond: Expr,
        body: Block,
    },
    Break,
    Continue,
    Return(Option<Expr>),

    // Function definitions
    Function(Function),

    // Expressions as statements
    Expr(Expr),

    // Module imports
    Use {
        module: String,
        alias: Option<String>,
    },

    // Blocks (for grouping)
    Block(Block),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    // Literals
    Literal(Literal),

    // Variables and access
    Identifier(String),
    Member {
        object: Box<Expr>,
        field: String,
    },

    // Function calls
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },

    // Binary operations
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    // Unary operations
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    // Control flow expressions
    If {
        cond: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },

    // Range expressions
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
    },

    // String interpolation
    FString {
        parts: Vec<FStringPart>,
    },

    // Async operations
    Await(Box<Expr>),
    Spawn(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FStringPart {
    Text(String),
    Expr(Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Comparison
    Eq,
    Ne,
    Lt,
    Gt,
    LtEq,
    GtEq,

    // Logical
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::String(a), Literal::String(b)) => a == b,
            (Literal::Bool(a), Literal::Bool(b)) => a == b,
            (Literal::Number(a), Literal::Number(b)) => a.to_bits() == b.to_bits(), // Compare f64 by bits
            _ => false,
        }
    }
}

impl Eq for Literal {}

use std::hash::{Hash, Hasher};

impl Hash for Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Literal::String(s) => {
                0u8.hash(state);
                s.hash(state);
            }
            Literal::Number(n) => {
                1u8.hash(state);
                n.to_bits().hash(state);
            }
            Literal::Bool(b) => {
                2u8.hash(state);
                b.hash(state);
            }
        }
    }
}
