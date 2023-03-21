#![allow(dead_code)]
use std::{fmt::Debug, marker::PhantomData};

fn main() {}

// This solution to expression problem is "Coproduct of Functors", with "real functors"
// https://www.cambridge.org/core/journals/journal-of-functional-programming/article/data-types-a-la-carte/14416CB20C4637164EA9F77097909409

pub trait HKT<U>: Debug {
    type Current;
    type Target;
}

pub trait Functor<B>: HKT<B> {
    fn fmap(&self, f: &dyn Fn(&Self::Current) -> B) -> Self::Target;
}

#[derive(Debug, PartialEq)]
pub struct IntVal<E> {
    value: i32,
    _p: PhantomData<E>,
}

impl<E> IntVal<E> {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            _p: PhantomData,
        }
    }
}

impl<U, E: Debug> HKT<U> for IntVal<E> {
    type Current = E;
    type Target = IntVal<U>;
}

impl<U: Debug, E: Debug> Functor<U> for IntVal<E> {
    fn fmap(&self, _f: &dyn Fn(&Self::Current) -> U) -> Self::Target {
        IntVal {
            value: self.value,
            _p: PhantomData,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Add<E> {
    lhs: E,
    rhs: E,
}

impl<E> Add<E> {
    pub fn new(lhs: E, rhs: E) -> Self {
        Self { lhs, rhs }
    }
}

impl<U: Debug, E: Debug> HKT<U> for Add<E> {
    type Current = E;

    type Target = Add<U>;
}

impl<U: Debug, E: Debug> Functor<U> for Add<E> {
    fn fmap(&self, f: &dyn Fn(&Self::Current) -> U) -> Self::Target {
        Add {
            lhs: f(&self.lhs),
            rhs: f(&self.rhs),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Coproduct<A, B, T> {
    L(A),
    R(B),
    P(PhantomData<T>),
}

impl<U: Debug, A, B, T: Debug> HKT<U> for Coproduct<A, B, T>
where
    A: Functor<T> + Functor<U>,
    B: Functor<T> + Functor<U>,
{
    type Current = T;
    type Target = Coproduct<<A as HKT<U>>::Target, <B as HKT<U>>::Target, U>;
}

impl<U: Debug, A, B, T: Debug> Functor<U> for Coproduct<A, B, T>
where
    A: Functor<T> + Functor<U> + HKT<U, Current = T>,
    B: Functor<T> + Functor<U> + HKT<U, Current = T>,
{
    fn fmap(&self, f: &dyn Fn(&Self::Current) -> U) -> Self::Target {
        match self {
            Coproduct::L(l) => Coproduct::L(l.fmap(f)),
            Coproduct::R(r) => Coproduct::R(r.fmap(f)),
            Coproduct::P(_) => panic!(),
        }
    }
}

pub type Op<E> = Coproduct<IntVal<E>, Add<E>, E>;

#[derive(Debug, PartialEq)]
pub struct Expr(Box<Op<Expr>>);

impl Expr {
    pub fn new(expr: Op<Expr>) -> Self {
        Self(Box::new(expr))
    }

    fn fold<W, A>(&self, eval: W) -> A
    where
        W: Fn(Op<A>) -> A + Copy,
        A: Debug,
    {
        let folded = (*self.0).fmap(&|e| e.fold::<W, A>(eval));
        eval(folded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_fold_expression() {
        let expr = Expr::new(Coproduct::R(Add::new(
            Expr::new(Coproduct::R(Add::new(
                Expr::new(Coproduct::L(IntVal::new(42))),
                Expr::new(Coproduct::L(IntVal::new(7))),
            ))),
            Expr::new(Coproduct::L(IntVal::new(2))),
        )));

        let int_eval = expr.fold(|e| match e {
            Coproduct::L(v) => v.value,
            Coproduct::R(s) => s.lhs + s.rhs,
            Coproduct::P(_) => panic!(),
        });
        assert_eq!(51, int_eval);

        let string_eval = expr.fold(|e| match e {
            Coproduct::L(v) => v.value.to_string(),
            Coproduct::R(s) => format!("({} + {})", s.lhs, s.rhs),
            Coproduct::P(_) => panic!(),
        });
        assert_eq!("((42 + 7) + 2)", string_eval);
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
