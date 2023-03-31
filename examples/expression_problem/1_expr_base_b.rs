// data IntExpr = Val Int | Add Expr Expr
// data MulExpr = Mul IntExpr Intexpr
// type Expr = Either IntExpr MulExpr
// data Either a b = Inl a | Inr b
#![allow(dead_code)]

pub enum IntExpr {
    Val(i32),
    Add(Box<IntExpr>, Box<IntExpr>),
}

impl IntExpr {
    fn eval(expr: IntExpr) -> i32 {
        match expr {
            IntExpr::Val(i) => i,
            IntExpr::Add(x, y) => Self::eval(*x) + Self::eval(*y),
        }
    }
}

pub enum MulExpr {
    Mul(Box<IntExpr>, Box<IntExpr>),
}

pub struct Expr(Coproduct<IntExpr, MulExpr>);

pub enum Coproduct<A, B> {
    L(A),
    R(B),
}

impl Expr {
    fn eval(expr: Expr) -> i32 {
        match expr.0 {
            Coproduct::L(ie) => IntExpr::eval(ie),
            Coproduct::R(me) => match me {
                MulExpr::Mul(x, y) => IntExpr::eval(*x) * IntExpr::eval(*y),
            },
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_eval_expression() {
        let expr = Expr(Coproduct::L(IntExpr::Add(
            Box::new(IntExpr::Val(2)),
            Box::new(IntExpr::Val(3)),
        )));

        // I can't add MulExpr inside IntExpr!! ((1 * 2) + 3)
        // let mul_expr = Expr(Coproduct::L(IntExpr::Add(
        //     Box::new(MulExpr::Mul(IntExpr::Val(1), IntExpr::Val(2))),
        //     Box::new(IntExpr::Val(3)),
        // )));

        assert_eq!(5, Expr::eval(expr))
    }
}
