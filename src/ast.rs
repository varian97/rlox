use crate::token::{Literal, Token};

pub trait ExprVisitor<T> {
    fn visit_binary_expr(
        &mut self,
        left: &mut Box<Expr>,
        operator: &mut Token,
        right: &mut Box<Expr>,
    ) -> T;
    fn visit_grouping_expr(&mut self, expression: &mut Box<Expr>) -> T;
    fn visit_literal_expr(&mut self, value: &mut Literal) -> T;
    fn visit_unary_expr(&mut self, operator: &mut Token, right: &mut Box<Expr>) -> T;
}

pub trait ExprVisitable<T> {
    fn accept(&mut self, visitor: &mut impl ExprVisitor<T>) -> T;
}

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
        right: Box<Expr>,
    },
}

impl<T> ExprVisitable<T> for Expr {
    fn accept(&mut self, visitor: &mut impl ExprVisitor<T>) -> T {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
        }
    }
}
