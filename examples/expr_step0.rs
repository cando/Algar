// data Expr = Val Int | Add Expr Expr
// eval :: Expr -> Int
// eval (Val x) = x
// eval (Add l r) = eval l + eval r

pub enum Expr {
    Val(i32),
    Add(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn eval(expr: Expr) -> i32 {
        match expr {
            Expr::Val(i) => i,
            Expr::Add(x, y) => Self::eval(*x) + Self::eval(*y),
        }
    }

    fn render(expr: Expr) -> String {
        match expr {
            Expr::Val(i) => i.to_string(),
            Expr::Add(x, y) => format!("{} + {}", Self::render(*x), Self::render(*y)),
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_eval_expression() {
        let expr = Expr::Add(
            Box::new(Expr::Add(Box::new(Expr::Val(2)), Box::new(Expr::Val(1)))),
            Box::new(Expr::Val(5)),
        );

        assert_eq!(8, Expr::eval(expr))
    }

    #[test]
    fn simple_render_expression() {
        let expr = Expr::Add(
            Box::new(Expr::Add(Box::new(Expr::Val(2)), Box::new(Expr::Val(1)))),
            Box::new(Expr::Val(5)),
        );

        assert_eq!("2 + 1 + 5", Expr::render(expr))
    }
}
