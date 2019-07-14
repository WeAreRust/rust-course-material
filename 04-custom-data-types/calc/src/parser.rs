use crate::{Expr, Variable};
use combine::parser::{
    char::{char, digit, spaces},
    choice::{choice, optional},
    combinator::attempt,
    range::recognize,
    repeat::{chainl1, skip_many, skip_many1},
    Parser,
};

pub fn parse_expr(input: &str) -> Option<Expr> {
    expr()
        .parse(input.trim())
        .ok()
        .and_then(|(expr, remaining)| match remaining {
            "" => Some(expr),
            _ => None,
        })
}

parser! {
    fn expr['a]()(&'a str) -> Expr {
        expr_inner()
    }
}

fn expr_inner<'a>() -> impl Parser<Input = &'a str, Output = Expr> {
    let op = choice((char('+'), char('-'))).map(|c| {
        move |lhs, rhs| {
            let lhs = Box::new(lhs);
            let rhs = Box::new(rhs);
            match c {
                '+' => Expr::Add { lhs, rhs },
                '-' => Expr::Sub { lhs, rhs },
                _ => unreachable!(),
            }
        }
    });
    chainl1(
        item(),
        attempt((spaces(), op, spaces())).map(|(_, op, _)| op),
    )
}

fn item<'a>() -> impl Parser<Input = &'a str, Output = Expr> {
    choice((
        char('x').map(|_| Expr::Var(Variable::X)),
        char('y').map(|_| Expr::Var(Variable::Y)),
        (char('('), spaces(), expr(), spaces(), char(')')).map(|(_, _, expr, _, _)| expr),
        recognize((
            optional(char('-')),
            skip_many1(digit()),
            optional((char('.'), skip_many(digit()))),
        ))
        .map(|num: &str| Expr::Const(num.parse().unwrap())),
    ))
}

#[cfg(test)]
mod tests {
    use super::parse_expr;
    use crate::{Expr, Variable};

    macro_rules! expr {
        (x) => { Expr::Var(Variable::X) };
        (y) => { Expr::Var(Variable::Y) };
        ($n:literal) => { Expr::Const($n) };
        (($($lhs:tt)+) + ($($rhs:tt)+)) => {
            Expr::Add {
                lhs: Box::new(expr!($($lhs)+)),
                rhs: Box::new(expr!($($rhs)+)),
            }
        };
        (($($lhs:tt)+) - ($($rhs:tt)+)) => {
            Expr::Sub {
                lhs: Box::new(expr!($($lhs)+)),
                rhs: Box::new(expr!($($rhs)+)),
            }
        };
    }

    #[test]
    fn expr() {
        assert_eq!(parse_expr("x"), Some(expr!(x)));
        assert_eq!(parse_expr("y"), Some(expr!(y)));
        assert_eq!(parse_expr(" y "), Some(expr!(y)));
        assert_eq!(parse_expr("3.14159"), Some(expr!(3.14159)));
        assert_eq!(parse_expr("-3.14159"), Some(expr!(-3.14159)));
        assert_eq!(parse_expr("(0)"), Some(expr!(0.)));
        assert_eq!(parse_expr("( 0 )"), Some(expr!(0.)));
        assert_eq!(parse_expr("1+2"), Some(expr!((1.) + (2.))));
        assert_eq!(parse_expr("1 + 2"), Some(expr!((1.) + (2.))));
        assert_eq!(parse_expr("1 - 2"), Some(expr!((1.) - (2.))));
        assert_eq!(parse_expr("(1 - 2)"), Some(expr!((1.) - (2.))));
        assert_eq!(parse_expr("( 1 - 2 )"), Some(expr!((1.) - (2.))));
        assert_eq!(parse_expr("1 + 2 + 3"), Some(expr!(((1.) + (2.)) + (3.))));
        assert_eq!(parse_expr("1 - 2 + 3"), Some(expr!(((1.) - (2.)) + (3.))));
        assert_eq!(parse_expr("1 - (2 + 3)"), Some(expr!((1.) - ((2.) + (3.)))));
        assert_eq!(parse_expr("x + y - 1"), Some(expr!(((x) + (y)) - (1.0))));
    }
}
