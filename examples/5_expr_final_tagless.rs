fn main() {}

// This solution to expression problem is "Final tagless encoding"
// https://okmij.org/ftp/tagless-final/JFP.pdf
pub trait Expr {
    type Repr<T>;
    fn i_val(v: i32) -> Self::Repr<i32>;
    fn add(lhs: Self::Repr<i32>, rhs: Self::Repr<i32>) -> Self::Repr<i32>;
}

pub struct EvaluateInt {}

impl Expr for EvaluateInt {
    type Repr<T> = i32;
    fn i_val(v: i32) -> i32 {
        v
    }

    fn add(lhs: i32, rhs: i32) -> i32 {
        lhs + rhs
    }
}

// ---------------------------------------------
// We can  easily add new operations
pub struct Render {}

impl Expr for Render {
    type Repr<T> = String;
    fn i_val(v: i32) -> String {
        v.to_string()
    }

    fn add(lhs: String, rhs: String) -> String {
        format!("({} + {})", lhs, rhs)
    }
}

// ---------------------------------------
// We can easily add another expression term!

pub trait MulExpr: Expr {
    fn mul(lhs: Self::Repr<i32>, rhs: Self::Repr<i32>) -> Self::Repr<i32>;
}

impl MulExpr for EvaluateInt {
    fn mul(lhs: i32, rhs: i32) -> i32 {
        lhs * rhs
    }
}

impl MulExpr for Render {
    fn mul(lhs: String, rhs: String) -> String {
        format!("({} + {})", lhs, rhs)
    }
}

// ----------------------------------------------------------------------------------------
// Final tagless typechecks on operations (i can't mix bool and int)

pub trait BoolExpr: Expr {
    fn b(value: bool) -> Self::Repr<bool>;
    fn and(lhs: Self::Repr<bool>, rhs: Self::Repr<bool>) -> Self::Repr<bool>;
}

impl BoolExpr for Render {
    fn b(value: bool) -> Self::Repr<bool> {
        format!("[{}]", value)
    }

    fn and(lhs: String, rhs: String) -> String {
        format!("({} AND {})", lhs, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_simple_expr<E>() -> E::Repr<i32>
    where
        E: Expr,
    {
        return E::add(E::i_val(2), E::i_val(3));
    }

    fn make_complex_expr<E>() -> E::Repr<i32>
    where
        E: Expr,
    {
        return E::add(E::add(E::i_val(2), E::i_val(3)), E::i_val(3));
    }

    fn make_complex_mul_expr<E>() -> E::Repr<i32>
    where
        E: MulExpr,
    {
        return E::mul(
            E::add(E::add(E::i_val(2), E::i_val(3)), E::i_val(3)),
            E::mul(
                E::add(E::add(E::i_val(2), E::i_val(3)), E::i_val(3)),
                E::i_val(12),
            ),
        );
    }

    fn make_complex_bool_expr<E>() -> E::Repr<bool>
    where
        E: BoolExpr,
    {
        return E::and(E::b(false), E::b(true));
    }

    // fn does_not_compile<E>() -> E::Repr<bool>
    // where
    //     E: BoolExpr,
    // {
    //     return E::and(E::i_val(12), E::b(true));
    // }

    #[test]
    fn simple_expression() {
        let expr = make_simple_expr::<EvaluateInt>();

        assert_eq!(5, expr);
    }
    #[test]
    fn complex_expression() {
        let expr = make_complex_expr::<Render>();

        assert_eq!("((2 + 3) + 3)", expr);
    }

    #[test]
    fn handle_mul_expression() {
        let expr = make_complex_mul_expr::<Render>();

        assert_eq!("(((2 + 3) + 3) + (((2 + 3) + 3) + 12))", expr);
    }

    #[test]
    fn handle_bool_expression() {
        let expr = make_complex_bool_expr::<Render>();

        assert_eq!("([false] AND [true])", expr);
    }
}
