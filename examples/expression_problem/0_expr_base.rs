// data Expr = Val Int | Add Expr Expr
// eval :: Expr -> Int
// eval (Val x) = x
// eval (Add l r) = eval l + eval r
#![allow(dead_code)]

pub enum Expr {
    Val(i32),
    Add(Box<Expr>, Box<Expr>),
}

fn eval(expr: Expr) -> i32 {
    match expr {
        Expr::Val(i) => i,
        Expr::Add(x, y) => eval(*x) + eval(*y),
    }
}

fn render(expr: Expr) -> String {
    match expr {
        Expr::Val(i) => i.to_string(),
        Expr::Add(x, y) => format!("{} + {}", render(*x), render(*y)),
    }
}

pub fn int_val<E: From<Expr>>(value: i32) -> E {
    E::from(Expr::Val(value))
}

pub fn add<E: From<Expr>>(lhs: Expr, rhs: Expr) -> E {
    E::from(Expr::Add(Box::new(lhs), Box::new(rhs)))
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_eval_expression() {
        let expr = add(int_val(1), int_val(2));

        assert_eq!(3, eval(expr))
    }

    #[test]
    fn simple_render_expression() {
        let expr = add(int_val(1), int_val(2));

        assert_eq!("1 + 2", render(expr))
    }
}
