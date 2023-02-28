// data Expr f = In (f (Expr f))
// data AddF a = Val Int | Add a a
// data MulF a = Mul a a
// type AddExpr = Expr AddF
// type AddMulExpr = Expr (AddF :+: MulF)
// addExample :: Expr (MulF :+: AddF)
// addExample = In (Inl (Mul (In (Inr (Val 1)))
// (In (Inr (Val 2)))))

pub trait HKT {
    type Inner;
    type HKT<B>;
}

pub enum Expr<F>
where
    F: HKT<Inner = Expr<F>>,
{
    In(F),
}

pub enum AddF<A> {
    Val(i32),
    Add(A, A),
}

impl<A> HKT for AddF<A> {
    type Inner = Expr<AddF<A>>;
    type HKT<B> = AddF<B>;
}

pub enum MulF<A> {
    Mul(A, A),
}

impl<A> HKT for MulF<A> {
    type Inner = Expr<MulF<A>>;
    type HKT<B> = MulF<B>;
}

// cata :: (Functor f) => (f a -> a) -> Expr f -> a
// cata phi (In t) = phi (fmap (cata phi) t)

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_eval_expression() {
        let program = Expr::In(AddF::Add(
            Expr::In(AddF::<i32>::Val(2)),
            Expr::In(AddF::Val(4)),
        ));
    }
}
