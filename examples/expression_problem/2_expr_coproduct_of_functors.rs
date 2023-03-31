use core::fmt;

fn main() {}

// This solution to expression problem is "Coproduct of Functors", but in a Rust-like way
// https://www.cambridge.org/core/journals/journal-of-functional-programming/article/data-types-a-la-carte/14416CB20C4637164EA9F77097909409

pub struct IntVal {
    value: i32,
}

pub struct Add<E> {
    lhs: E,
    rhs: E,
}

pub enum Coproduct<A, B> {
    L(A),
    R(B),
}

pub type Op<E> = Coproduct<IntVal, Add<E>>;

pub struct Expr(Box<Op<Expr>>);

pub trait EvaluateInt {
    fn eval(&self) -> i32;
}

impl EvaluateInt for IntVal {
    fn eval(&self) -> i32 {
        self.value
    }
}

impl<E> EvaluateInt for Add<E>
where
    E: EvaluateInt,
{
    fn eval(&self) -> i32 {
        self.lhs.eval() + self.rhs.eval()
    }
}
impl<A, B> EvaluateInt for Coproduct<A, B>
where
    A: EvaluateInt,
    B: EvaluateInt,
{
    fn eval(&self) -> i32 {
        match self {
            Coproduct::L(a) => a.eval(),
            Coproduct::R(b) => b.eval(),
        }
    }
}

impl EvaluateInt for Expr {
    fn eval(&self) -> i32 {
        self.0.eval()
    }
}

// ---------------------------------------
// We can now add another expression term!

pub struct Mul<E> {
    lhs: E,
    rhs: E,
}

impl<E> EvaluateInt for Mul<E>
where
    E: EvaluateInt,
{
    fn eval(&self) -> i32 {
        self.lhs.eval() * self.rhs.eval()
    }
}

pub type OpMul<E> = Coproduct<Mul<E>, Op<E>>;
pub struct MulExpr(Box<OpMul<MulExpr>>);

impl EvaluateInt for MulExpr {
    fn eval(&self) -> i32 {
        self.0.eval()
    }
}

// ---------------------------------------------
// And we can also easily add new operations

impl fmt::Display for IntVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<E> fmt::Display for Add<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} + {})", self.lhs, self.rhs)
    }
}

impl<E> fmt::Display for Mul<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} * {})", self.lhs, self.rhs)
    }
}

impl<A, B> fmt::Display for Coproduct<A, B>
where
    A: fmt::Display,
    B: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Coproduct::L(l) => write!(f, "{}", l),
            Coproduct::R(r) => write!(f, "{}", r),
        }
    }
}

impl fmt::Display for MulExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_eval_expression() {
        let expr = Expr(Box::new(Coproduct::R(Add {
            lhs: Expr(Box::new(Coproduct::L(IntVal { value: 1 }))),
            rhs: Expr(Box::new(Coproduct::L(IntVal { value: 2 }))),
        })));

        assert_eq!(expr.eval(), 3);
    }

    #[test]
    fn mul_expression() {
        let expr = MulExpr(Box::new(Coproduct::R(Coproduct::R(Add {
            lhs: MulExpr(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 3 })))),
            rhs: MulExpr(Box::new(Coproduct::L(Mul {
                lhs: MulExpr(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 42 })))),
                rhs: MulExpr(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 2 })))),
            }))),
        }))));

        assert_eq!(expr.eval(), 87);
    }

    #[test]
    fn render_expression() {
        let expr = MulExpr(Box::new(Coproduct::R(Coproduct::R(Add {
            lhs: MulExpr(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 3 })))),
            rhs: MulExpr(Box::new(Coproduct::L(Mul {
                lhs: MulExpr(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 42 })))),
                rhs: MulExpr(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 2 })))),
            }))),
        }))));

        assert_eq!(expr.to_string(), "(3 + (42 * 2))");
    }
}
