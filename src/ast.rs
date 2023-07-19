use crate::token::{Token, Literal};

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>
    }
}

pub trait Visitor<T> {
    fn accept(&mut self, expr: Expr) -> T {
        match expr {
            Expr::Binary{ left, operator, right } => self.visit_binary_expr(left, operator, right),
            Expr::Grouping{ expression } => self.visit_grouping_expr(expression),
            Expr::Literal{ value } => self.visit_literal_expr(value),
            Expr::Unary{ operator, right } => self.visit_unary_expr(operator, right)
        }
    }

    fn visit_binary_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> T;
    fn visit_grouping_expr(&mut self, expression: Box<Expr>) -> T;
    fn visit_literal_expr(&mut self, value: Literal) -> T;
    fn visit_unary_expr(&mut self, operator: Token, right: Box<Expr>) -> T;
}