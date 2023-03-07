use core::fmt;

fn main() {}

// This solution to expression problem is "Coproduct of Functors" (can be generalized to Free Monads???)
// (Another is tagless final encoding)
pub trait EvaluateInt {
    fn eval(&self) -> i32;
}

pub struct IntVal {
    value: i32,
}

impl EvaluateInt for IntVal {
    fn eval(&self) -> i32 {
        self.value
    }
}

pub struct Add<E> {
    lhs: E,
    rhs: E,
}

impl<E> EvaluateInt for Add<E>
where
    E: EvaluateInt,
{
    fn eval(&self) -> i32 {
        self.lhs.eval() + self.rhs.eval()
    }
}

pub enum Coproduct<A, B> {
    L(A),
    R(B),
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

pub type Op<E> = Coproduct<IntVal, Add<E>>;
pub struct Expr(Box<Op<Expr>>);

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

pub type Op2<E> = Coproduct<Mul<E>, Op<E>>;
pub struct Expr2(Box<Op2<Expr2>>);

impl EvaluateInt for Expr2 {
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

impl fmt::Display for Expr2 {
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
        let expr = Expr2(Box::new(Coproduct::R(Coproduct::R(Add {
            lhs: Expr2(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 3 })))),
            rhs: Expr2(Box::new(Coproduct::L(Mul {
                lhs: Expr2(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 42 })))),
                rhs: Expr2(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 2 })))),
            }))),
        }))));

        assert_eq!(expr.eval(), 87);
    }

    #[test]
    fn render_expression() {
        let expr = Expr2(Box::new(Coproduct::R(Coproduct::R(Add {
            lhs: Expr2(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 3 })))),
            rhs: Expr2(Box::new(Coproduct::L(Mul {
                lhs: Expr2(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 42 })))),
                rhs: Expr2(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 2 })))),
            }))),
        }))));

        assert_eq!(expr.to_string(), "(3 + (42 * 2))");
    }
}
