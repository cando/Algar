use std::marker::PhantomData;

fn main() {}

pub trait HKT<U> {
    type Current;
    type Target;
}

pub trait Functor<B>: HKT<B> {
    fn fmap(self, f: &dyn Fn(Self::Current) -> B) -> Self::Target;
}

pub trait Expressable {}

// This solution to expression problem is "Coproduct of Functors" (can be generalized to Free Monads???)
impl Expressable for IntVal {}
impl<A> Expressable for Add<A> {}
impl<'a, A, B, T> Expressable for Coproduct<'a, A, B, T> {}

#[derive(Debug, PartialEq)]
pub struct IntVal {
    value: i32,
}

impl<U> HKT<U> for IntVal {
    type Current = U;
    type Target = IntVal;
}

impl<B> Functor<B> for IntVal {
    fn fmap(self, _f: &dyn Fn(Self::Current) -> B) -> Self::Target {
        self
    }
}

#[derive(Debug, PartialEq)]
pub struct Add<E> {
    lhs: E,
    rhs: E,
}

impl<U, E> HKT<U> for Add<E> {
    type Current = E;

    type Target = Add<U>;
}

impl<U, E> Functor<U> for Add<E> {
    fn fmap(self, f: &dyn Fn(Self::Current) -> U) -> Self::Target {
        Add {
            lhs: f(self.lhs),
            rhs: f(self.rhs),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Coproduct<'a, A, B, T> {
    L(A),
    R(B),
    P(PhantomData<&'a T>),
}

impl<'a, U: 'a, A, B, T: 'a> HKT<U> for Coproduct<'a, A, B, T>
where
    A: Functor<T> + Functor<U>,
    B: Functor<T> + Functor<U>,
{
    type Current = T;
    type Target = Coproduct<'a, <A as HKT<U>>::Target, <B as HKT<U>>::Target, U>;
}

impl<'a, U: 'a, A, B, T: 'a> Functor<U> for Coproduct<'a, A, B, T>
where
    A: Functor<T> + Functor<U> + HKT<U, Current = T>,
    B: Functor<T> + Functor<U> + HKT<U, Current = T>,
{
    fn fmap(self, f: &dyn Fn(Self::Current) -> U) -> Self::Target {
        match self {
            Coproduct::L(l) => Coproduct::L(l.fmap(f)),
            Coproduct::R(r) => Coproduct::R(r.fmap(f)),
            Coproduct::P(_) => todo!(),
        }
    }
}

pub type Op<E> = Coproduct<'static, IntVal, Add<E>, E>;

#[derive(Debug, PartialEq)]
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

impl<'a, A, B, T> EvaluateInt for Coproduct<'a, A, B, T>
where
    A: EvaluateInt + Functor<T>,
    B: EvaluateInt + Functor<T>,
{
    fn eval(&self) -> i32 {
        match self {
            Coproduct::L(a) => a.eval(),
            Coproduct::R(b) => b.eval(),
            Coproduct::P(_) => todo!(),
        }
    }
}

impl<'a> EvaluateInt for Expr {
    fn eval(&self) -> i32 {
        self.0.eval()
    }
}

impl Expr {
    fn fold<W>(self, eval: W) -> Expr
    where
        W: Fn(&dyn EvaluateInt) -> Expr + Copy,
    {
        let expr = *self.0;

        eval(&expr.fmap(&|e: Expr| e.fold::<W>(eval)))
    }
}

// ---------------------------------------
// We can now add another expression term!

// pub struct Mul<E> {
//     lhs: E,
//     rhs: E,
// }

// pub type Op2<E> = Coproduct<'static, Mul<E>, Op<E>, E>;
// pub struct Expr2(Box<Op2<Expr2>>);

// ---------------------------------------------
// And we can also easily add new operations

// impl fmt::Display for IntVal {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.value)
//     }
// }

// impl<E> fmt::Display for Add<E>
// where
//     E: fmt::Display,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({} + {})", self.lhs, self.rhs)
//     }
// }

// impl<E> fmt::Display for Mul<E>
// where
//     E: fmt::Display,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({} * {})", self.lhs, self.rhs)
//     }
// }

// impl<A, B> fmt::Display for Coproduct<A, B>
// where
//     A: fmt::Display,
//     B: fmt::Display,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Coproduct::L(l) => write!(f, "{}", l),
//             Coproduct::R(r) => write!(f, "{}", r),
//         }
//     }
// }

// impl fmt::Display for Expr2 {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_fold_expression() {
        let expr = Expr(Box::new(Coproduct::R(Add {
            lhs: Expr(Box::new(Coproduct::L(IntVal { value: 1 }))),
            rhs: Expr(Box::new(Coproduct::L(IntVal { value: 2 }))),
        })));

        let a = expr.fold(|e| Expr(Box::new(Coproduct::L(IntVal { value: e.eval() }))));
        assert_eq!(Expr(Box::new(Coproduct::L(IntVal { value: 3 }))), a)
    }

    // #[test]
    // fn mul_expression() {
    //     let expr = Expr2(Box::new(Coproduct::R(Coproduct::R(Add {
    //         lhs: Expr2(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 3 })))),
    //         rhs: Expr2(Box::new(Coproduct::L(Mul {
    //             lhs: Expr2(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 42 })))),
    //             rhs: Expr2(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 2 })))),
    //         }))),
    //     }))));

    //     assert_eq!(expr.eval(), 87);
    // }

    // #[test]
    // fn render_expression() {
    //     let expr = Expr2(Box::new(Coproduct::R(Coproduct::R(Add {
    //         lhs: Expr2(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 3 })))),
    //         rhs: Expr2(Box::new(Coproduct::L(Mul {
    //             lhs: Expr2(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 42 })))),
    //             rhs: Expr2(Box::new(Coproduct::R(Coproduct::L(IntVal { value: 2 })))),
    //         }))),
    //     }))));

    //     assert_eq!(expr.to_string(), "(3 + (42 * 2))");
    // }
}
