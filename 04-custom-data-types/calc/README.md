# Workshop

In this workshop, we are going to implement a simple expression calculator.

* update local clone of `rust-course-material`
* `cd 04-custom-data-types/calc`

## 1. Implement `Expr::calc`

* open `src/lib.rs`
* remove `unimplemented!()` inside `Expr::calc` and implement it based on the document
* uncomment all tests in `mod calc_tests`
* execute `cargo test` and ensure all tests pass

## 2. Implement `Expr::simplify`

* remove `unimplemented!()` inside `Expr::simplify` and implement it based on the document
* uncomment all tests in `mod simplify_tests`
* execute `cargo test` and ensure all tests pass

## 3. Add multiplication support

* add a new variant `Mul` in `Expr`
* update implementation of `Expr::calc` and `Expr::simplify`
* (there is no test currently)

## 4. (extra challenge) Add multiplication to parser

Note:
* the code is in `src/parser.rs`
* [documentation of `combine`](https://docs.rs/combine/3.8.1/combine/) may be helpful
* it may require an extra level to handle precedence correctly
* don't forget to add some tests
* if you are interested in `macro_rules`,
 [this page](https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html) may be useful
