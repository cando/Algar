// // data Expr f = In (f (Expr f))
// // data AddF a = Val Int | Add a a
// // data MulF a = Mul a a
// // type AddExpr = Expr AddF
// // type AddMulExpr = Expr (AddF :+: MulF)
// // addExample :: Expr (MulF :+: AddF)
// // addExample = In (Inl (Mul (In (Inr (Val 1)))
// // (In (Inr (Val 2)))))

// use std::marker::PhantomData;

// use algar::Functor;

// pub trait Family {
//     type This<A>;
// }

// pub struct Expr<F: Family<This<F> = Expr<F>>> {
//     _in: F,
// }

// impl<F: Family<This<F> = Expr<F>>> Expr<F> {
//     pub fn e(expr: F) -> Self {
//         Self { _in: expr }
//     }
// }

// pub struct Val<A> {
//     value: i32,
//     _p: PhantomData<A>,
// }

// impl<T> Family for Val<T> {
//     type This<A> = Val<A>;
// }

// impl<A> Val<A> {
//     pub fn new(value: i32) -> Self {
//         Val {
//             value,
//             _p: PhantomData,
//         }
//     }
// }

// type ValExpr<A> = Expr<Val<A>>;

// pub struct Add<A> {
//     a: A,
//     b: A,
// }

// impl<T> Family for Add<T> {
//     type This<A> = Add<A>;
// }

// impl<A> Add<A> {
//     pub fn new(a: A, b: A) -> Self {
//         Add { a, b }
//     }
// }

// type AddExpr<A> = Expr<Add<A>>;

// impl<'a, A> Functor<'a> for Val<A> {
//     type Unwrapped = i32;
//     type Wrapped<B: 'a> = Val<B>;

//     fn fmap<F, B: 'a>(self, _f: F) -> Self::Wrapped<B>
//     where
//         F: Fn(Self::Unwrapped) -> B + 'a,
//     {
//         Val {
//             value: self.value,
//             _p: PhantomData,
//         }
//     }
// }

// impl<'a, A> Functor<'a> for Add<A> {
//     type Unwrapped = A;
//     type Wrapped<B: 'a> = Add<B>;

//     fn fmap<F, B: 'a>(self, f: F) -> Self::Wrapped<B>
//     where
//         F: Fn(Self::Unwrapped) -> B + 'a,
//     {
//         Add {
//             a: f(self.a),
//             b: f(self.b),
//         }
//     }
// }

// // def foldExpr[F[_]: Functor, T](eval: F[T] => T)(expr: Expr[F]): T =
// //     eval(expr.in.map(foldExpr(eval)))

// // fn foldExpr<U, F, A>(eval: U, expr: Expr<F>) -> A
// // where
// //     F: HKT<Inner = Expr<F>> + for<'a> Functor<'a>,
// //     U: Fn(F) -> A,
// // {
// //     eval(expr._in.fmap(foldExpr(eval)))
// // }

// // fn foldExpr<U, F, A: 'static>(eval: U) -> impl Fn(Expr<F>) -> A
// // where
// //     F: HKT<Inner = Expr<F>>
// //         + Functor<'static, Unwrapped = Expr<F>>
// //         + Functor<'static, Wrapped<A> = F>
// //         + 'static,
// //     U: Fn(Expr<F>) -> A + Copy + 'static,
// // {
// //     move |expr| eval(Expr::In(expr._in.fmap(foldExpr(eval))))
// // }
// // cata :: (Functor f) => (f a -> a) -> Expr f -> a
// // cata phi (In t) = phi (fmap (cata phi) t)

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_eval_expression() {
        let program = Expr::e(Val::new(1));
    }
}
