#[macro_use]
extern crate combine;

mod parser;

pub use parser::parse_expr;

#[derive(Clone, Debug, PartialEq)]
pub enum Variable {
    X,
    Y,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Const(f64),
    Var(Variable),
    Add { lhs: Box<Expr>, rhs: Box<Expr> },
    Sub { lhs: Box<Expr>, rhs: Box<Expr> },
}

impl Expr {
    /// Calculates this expression with variables being the given values.
    pub fn calc(&self, x: f64, y: f64) -> f64 {
        unimplemented!()
    }

    /// Simplifies the expression by merging parts only involve constants.
    ///
    /// It simplifies `1 + 2 + 3` to `6`, but it doesn't change `x + 2 + 3` because it's stored as
    /// `(x + 2) + 3` which means no sub-expression involve constants only.
    pub fn simplify(self) -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
mod calc_tests {
    // use crate::parser::parse_expr;

    // #[test]
    // fn calc_var() {
    //     assert_eq!(parse_expr("x").unwrap().calc(100., 200.), 100.);
    //     assert_eq!(parse_expr("y").unwrap().calc(100., 200.), 200.);
    // }

    // #[test]
    // fn calc_const() {
    //     assert_eq!(parse_expr("300").unwrap().calc(-1., 2.), 300.);
    // }

    // #[test]
    // fn calc_add() {
    //     assert_eq!(parse_expr("1 + 2").unwrap().calc(0., 0.), 3.);
    //     assert_eq!(parse_expr("x + y").unwrap().calc(3., 4.), 7.);
    //     assert_eq!(parse_expr("x + 2 + y").unwrap().calc(3., 4.), 9.);
    // }

    // #[test]
    // fn calc_sub() {
    //     assert_eq!(parse_expr("1 - 2").unwrap().calc(0., 0.), -1.);
    //     assert_eq!(parse_expr("y - x").unwrap().calc(100., 200.), 100.);
    //     assert_eq!(parse_expr("y - x + 1").unwrap().calc(100., 200.), 101.);
    //     assert_eq!(parse_expr("y - x - 1").unwrap().calc(100., 200.), 99.);
    //     assert_eq!(parse_expr("1 - (x - y)").unwrap().calc(100., 200.), 101.);
    // }
}

#[cfg(test)]
mod simplify_tests {
    // use crate::parser::parse_expr;

    // #[test]
    // fn simplify_single() {
    //     assert_eq!(
    //         parse_expr("x").unwrap().simplify(),
    //         parse_expr("x").unwrap(),
    //     );
    //     assert_eq!(
    //         parse_expr("y").unwrap().simplify(),
    //         parse_expr("y").unwrap(),
    //     );
    //     assert_eq!(
    //         parse_expr("1").unwrap().simplify(),
    //         parse_expr("1").unwrap(),
    //     );
    // }

    // #[test]
    // fn simplify_add() {
    //     assert_eq!(
    //         parse_expr("1 + 2").unwrap().simplify(),
    //         parse_expr("3").unwrap()
    //     );
    //     assert_eq!(
    //         parse_expr("1 + 2 + 3").unwrap().simplify(),
    //         parse_expr("6").unwrap(),
    //     );
    //     assert_eq!(
    //         parse_expr("1 + 2 + y").unwrap().simplify(),
    //         parse_expr("3 + y").unwrap(),
    //     );
    //     assert_eq!(
    //         parse_expr("x + 2 + 3").unwrap().simplify(),
    //         parse_expr("x + 2 + 3").unwrap(),
    //     );
    // }

    // #[test]
    // fn simplify_sub() {
    //     assert_eq!(
    //         parse_expr("1 - 2").unwrap().simplify(),
    //         parse_expr("-1").unwrap()
    //     );
    //     assert_eq!(
    //         parse_expr("1 - 2 + 3").unwrap().simplify(),
    //         parse_expr("2").unwrap(),
    //     );
    //     assert_eq!(
    //         parse_expr("1 - 2 - y").unwrap().simplify(),
    //         parse_expr("-1 - y").unwrap(),
    //     );
    //     assert_eq!(
    //         parse_expr("x - 2 - 3").unwrap().simplify(),
    //         parse_expr("x - 2 - 3").unwrap(),
    //     );
    // }
}
