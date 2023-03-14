fn main() {}

// This solution to expression problem is "Object algebras"
// https://www.cs.utexas.edu/~wcook/Drafts/2012/ecoop2012.pdf
pub trait Expr<E> {
    fn i_val(&self, v: i32) -> E;
    fn add(&self, lhs: E, rhs: E) -> E;
}

pub struct EvaluateInt {}

impl Expr<i32> for EvaluateInt {
    fn i_val(&self, v: i32) -> i32 {
        v
    }

    fn add(&self, lhs: i32, rhs: i32) -> i32 {
        lhs + rhs
    }
}

// ---------------------------------------------
// We can  easily add new operations
pub struct Render {}

impl Expr<String> for Render {
    fn i_val(&self, v: i32) -> String {
        v.to_string()
    }

    fn add(&self, lhs: String, rhs: String) -> String {
        format!("({} + {})", lhs, rhs)
    }
}

// ---------------------------------------
// We can easily add another expression term!

pub trait MulExpr<E>: Expr<E> {
    fn mul(&self, lhs: E, rhs: E) -> E;
}

impl MulExpr<i32> for EvaluateInt {
    fn mul(&self, lhs: i32, rhs: i32) -> i32 {
        lhs * rhs
    }
}

impl MulExpr<String> for Render {
    fn mul(&self, lhs: String, rhs: String) -> String {
        format!("({} + {})", lhs, rhs)
    }
}

// But if we add another "type" to our abstraction?

pub trait BoolExpr<E>: Expr<E> {
    fn b(&self, value: bool) -> E;
    fn and(&self, lhs: E, rhs: E) -> E;
}

impl BoolExpr<String> for Render {
    fn b(&self, value: bool) -> String {
        format!("[{}]", value)
    }

    fn and(&self, lhs: String, rhs: String) -> String {
        format!("({} AND {})", lhs, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_simple_expr<E>(expr: &dyn Expr<E>) -> E {
        return expr.add(expr.i_val(2), expr.i_val(3));
    }

    fn make_complex_expr<E>(expr: &dyn Expr<E>) -> E {
        return expr.add(expr.add(expr.i_val(2), expr.i_val(3)), expr.i_val(3));
    }

    fn make_complex_mul_expr<E>(expr: &dyn MulExpr<E>) -> E {
        return expr.mul(
            expr.add(
                expr.mul(expr.add(expr.i_val(1), expr.i_val(5)), expr.i_val(7)),
                expr.i_val(3),
            ),
            expr.i_val(3),
        );
    }

    // DOH! It compiles, but we are mixing boolean and integers!
    fn make_mixed_expr<E>(expr: &dyn BoolExpr<E>) -> E {
        return expr.add(expr.add(expr.b(false), expr.i_val(3)), expr.i_val(3));
    }

    #[test]
    fn simple_expression() {
        let expr = make_simple_expr(&EvaluateInt {});

        assert_eq!(5, expr);
    }
    #[test]
    fn complex_expression() {
        let expr = make_complex_expr(&Render {});

        assert_eq!("((2 + 3) + 3)", expr);
    }

    #[test]
    fn handle_mul_expression() {
        let expr = make_complex_mul_expr(&Render {});

        assert_eq!("((((1 + 5) + 7) + 3) + 3)", expr);
    }

    #[test]
    fn handle_mixed_wrong_expression_doh() {
        let expr = make_mixed_expr(&Render {});

        // What does this mean???? Let's try to improve it...Final tagless encoding!
        assert_eq!("(([false] + 3) + 3)", expr);
    }
}
