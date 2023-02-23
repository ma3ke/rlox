use std::fmt::Display;

use crate::token::{Literal, Token, TokenType};

trait Nary {
    fn interpret(&self);
    fn resolve(&self);
    fn analyze(&self);
}

type WrappedExpr = Box<Expr>;

#[derive(Debug)]
pub(crate) enum Expr {
    Literal {
        value: Literal,
    },
    Variable {
        name: Token,
    },
    Assign {
        name: Token,
        value: WrappedExpr,
    },
    Logical {
        left: WrappedExpr,
        operator: Token,
        right: WrappedExpr,
    },
    Unary {
        operator: Token,
        right: WrappedExpr,
    },
    Binary {
        left: WrappedExpr,
        operator: Token,
        right: WrappedExpr,
    },
    Grouping {
        expression: WrappedExpr,
    },
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal { value } => write!(f, "{value}"),
            Expr::Variable { name } => write!(f, "{name}"),
            Expr::Assign { name, value } => write!(f, "{name} = {value}"),
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let op = match operator.token_type() {
                    TokenType::Or => "or",
                    TokenType::And => "and",
                    _ => unreachable!(),
                };
                write!(f, "{left} {op} {right}")
            }
            Expr::Unary { operator, right } => write!(f, "({} {right})", operator.lexeme()),
            Expr::Binary {
                left,
                operator,
                right,
            } => write!(f, "({left} {} {right})", operator.lexeme()),
            Expr::Grouping { expression } => write!(f, "{expression}"),
        }
    }
}

type WrappedStmt = Box<Stmt>;

#[derive(Debug)]
pub(crate) enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Expression {
        expression: WrappedExpr,
    },
    If {
        condition: Expr,
        then_branch: WrappedStmt,
        else_branch: Option<WrappedStmt>,
    },
    Print {
        expression: WrappedExpr,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Block { statements } => write!(
                f,
                "{{ {} }}",
                statements
                    .iter()
                    .map(|stmt| stmt.to_string())
                    .collect::<Vec<_>>()
                    .join("  ")
            ),
            Stmt::Expression { expression } => write!(f, "{expression}"),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                write!(f, "if ({condition}) {then_branch}")?;
                if let Some(else_branch) = else_branch {
                    write!(f, " else {else_branch}")?;
                };
                Ok(())
            }
            Stmt::Print { expression } => write!(f, "print {expression}"),
            Stmt::Var {
                name,
                initializer: Some(init),
            } => write!(f, "var {name} = {init}"),
            Stmt::Var {
                name,
                initializer: None,
            } => write!(f, "var {name}"),
        }
    }
}
