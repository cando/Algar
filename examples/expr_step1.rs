// data IntExpr = Val Int | Add Expr Expr
// data MulExpr = Mul IntExpr Intexpr
// type Expr = Either IntExpr MulExpr
// data Either a b = Inl a | Inr b

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

pub struct Expr(Either<IntExpr, MulExpr>);

pub enum Either<A, B> {
    Inl(A),
    Inr(B),
}

impl Expr {
    fn eval(expr: Expr) -> i32 {
        match expr.0 {
            Either::Inl(ie) => IntExpr::eval(ie),
            Either::Inr(me) => match me {
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
        // I can't add MulExpr inside IntExpr!!
        let expr = Expr(Either::Inl(IntExpr::Add(
            Box::new(IntExpr::Val(2)),
            Box::new(IntExpr::Val(3)),
        )));

        assert_eq!(5, Expr::eval(expr))
    }
}
