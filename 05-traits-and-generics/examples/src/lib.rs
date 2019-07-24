//! Rust Workshop 5
//! Generics Types and Traits

#![allow(dead_code, unused_imports, unused_variables)]

//
// Generic Functions
//

#[test]
fn generic_function() {
    // Fn with generic type T
    fn do_foo<T>(x: T) -> T {
        x
    }

    // do_foo::<i32>
    let a = do_foo(10);
    assert_eq!(a, 10);

    // do_foo::<&str>
    let b = do_foo("abc");
    assert_eq!(b, "abc");
}

#[test]
fn generic_enum() {
    // Enum with generic type T
    enum Foo<T> {
        Baz(T),
    }

    // Foo<i32>
    let a = Foo::Baz(10);

    // Foo<&str>
    let b = Foo::Baz("abc");
}

#[test]
fn generic_struct() {
    // Struct with generic type T
    struct Foo<T> {
        x: T,
    }

    // Foo<i32>
    let a = Foo { x: 10 };
    assert_eq!(a.x, 10);

    // Foo<&str>
    let b = Foo { x: "abc" };
    assert_eq!(b.x, "abc");
}

#[test]
fn generic_struct_multiple_generics() {
    // Struct with generic type Bar and Baz
    struct Foo<Bar, Baz> {
        x: Bar,
        y: Baz,
    }

    // Foo<i32, String>
    let a = Foo {
        x: 10,
        y: String::from("abc"),
    };
    assert_eq!(a.x, 10);
    assert_eq!(a.y, "abc");
}

#[test]
fn impl_for_generic_struct() {
    struct Foo<T> {
        x: T,
    }

    impl<T> Foo<T> {
        fn get_x(&self) -> &T {
            &self.x
        }
    }

    // Foo<i32>
    let a = Foo { x: 10 };
    assert_eq!(a.get_x(), &10);

    // Foo<String>
    let b = Foo {
        x: String::from("abc"),
    };
    assert_eq!(b.get_x(), "abc");
}

#[test]
fn unused_generics() {
    // Have to be used when defining a type.
    struct Foo<T> {
        x: u32,
        y: T, // <- comment me
    }
}

#[test]
fn generics_without_inference() {
    // The compiler has to know what type T is
    fn do_foo<T>() -> &'static str {
        "abc"
    }

    // assert_eq!(do_foo(), "abc"); // <- uncomment me
    assert_eq!(do_foo::<i32>(), "abc");

    // Turbofish ::<>
    // Awesome... but rare
}

#[test]
fn generics_with_defaults() {
    struct Foo<T>(T);

    // let x: Foo = Foo(10);   // <- uncomment me

    struct FooWithDefault<T = u32>(T);

    let y: FooWithDefault = FooWithDefault(10);
}

//
// Trait Bounds
//

#[test]
fn trait_bounds_single() {
    use std::fmt::Debug;

    // fn show<T>(a: T) {   // <- uncomment me
    fn show<T: Debug>(a: T) {
        // <- comment me
        println!("{:?}", a);
    }
}

#[test]
fn trait_bounds_multiple() {
    use std::fmt::Debug;

    // fn show_ne<T: PartialEq>(a: T, b: T) {  // uncomment me
    fn show_ne<T: Debug + PartialEq>(a: T, b: T) {
        // comment me
        if a != b {
            println!("{:?} != {:?}", a, b);
        }
    }
}

#[test]
fn trait_bounds_where_syntax() {
    use std::fmt::Debug;

    fn show_ne<T>(a: T, b: T)
    where
        T: Debug,
        T: PartialEq,
    {
        if a != b {
            println!("{:?} != {:?}", a, b);
        }
    }
}

#[test]
fn trait_bounds_more_where_syntax() {
    use std::fmt::Debug;

    // Just for demonstration
    // Mainly seen with Associated Types
    fn print_vec<T>(a: Vec<T>)
    where
        Vec<T>: Debug + PartialEq,
    {
        println!("{:?}", a);
    }
}

#[test]
fn trait_bounds_lifetimes() {
    // Recall something similar from Ben's talk (Week 2)
    fn first<'a>(x: &'a u32, y: &'a u32) -> &'a u32 {
        if x < y {
            return x;
        }
        y
    }

    assert_eq!(first(&10, &11), &10);
    // assert_eq!(first(&-1, &-2), &-2);    // <- uncomment me
}

#[test]
fn trait_bounds_lifetimes_and_generics() {
    // We usually put the lifetime before the generic
    fn first<'a, T>(x: &'a T, y: &'a T) -> &'a T
    where
        T: Eq + Ord,
    {
        if x < y {
            return x;
        }
        y
    }

    assert_eq!(first(&10, &11), &10);
    assert_eq!(first(&-1, &-2), &-2);
}

#[test]
fn trait_bounds_impl_uniform() {
    use std::fmt::Debug;

    struct Foo<T: Debug>(T);

    // comment me
    // impl Foo<i32> {
    //     fn show_inner(&self) {
    //         println!("{:?}", self.0);
    //     }
    // }

    // let x = Foo(10); // <- comment me

    impl<T: Debug> Foo<T> {
        fn show_inner(&self) {
            println!("{:?}", self.0);
        }
    }

    #[derive(Debug)]
    struct MyType;

    let x = Foo(MyType);
    x.show_inner();
}

#[test]
fn trait_bounds_impl_distinct() {
    use std::fmt::Debug;

    struct Foo<T>(T);

    // impl Foo<i32> {
    //     fn show_inner(&self) {
    //         println!("{:?}", self.0);
    //     }
    // }

    impl<T: Debug> Foo<T> {
        fn show_inner(&self) {
            println!("{:?}", self.0);
        }
    }

    // #[derive(Debug)]
    struct MyType;

    let x = Foo(MyType);
    // x.show_inner(); // uncomment me
}

//
// Common and custom traits
//
// - Debug
// - Default & Display
// - PartialEq & Eq
// - PartialOrd & Ord
// - Clone & Copy
// - From & Into
//

#[test]
fn custom_trait() {
    trait HasLength {
        fn my_len(&self) -> usize;
    }

    struct Foo;

    impl HasLength for Foo {
        fn my_len(&self) -> usize {
            0
        }
    }

    let x = Foo;
    assert_eq!(x.my_len(), 0);
}

#[test]
fn custom_trait_default() {
    trait HasLength {
        fn my_len(&self) -> usize;

        fn is_empty(&self) -> bool {
            self.my_len() == 0
        }
    }

    struct Foo;

    impl HasLength for Foo {
        fn my_len(&self) -> usize {
            0
        }
    }

    let x = Foo;
    assert!(x.is_empty());
}

#[test]
fn default_trait_functions_impl_over() {
    trait HasLength {
        fn my_len(&self) -> usize;

        fn is_empty(&self) -> bool {
            self.my_len() == 0
        }
    }

    struct Foo;

    impl HasLength for Foo {
        fn my_len(&self) -> usize {
            0
        }

        // Override HasLength::is_empty
        fn is_empty(&self) -> bool {
            false
        }
    }

    let x = Foo;
    assert!(!x.is_empty());
}

#[test]
fn impl_from() {
    struct Foo {
        s: String,
    }

    impl From<String> for Foo {
        fn from(s: String) -> Self {
            Foo { s }
        }
    }

    let s = String::from("bar");
    let a = Foo::from(s);
    assert_eq!(a.s, "bar");
}

#[test]
fn impl_from_generic() {
    struct Foo {
        s: String,
    }

    impl<S: Into<String>> From<S> for Foo {
        fn from(data: S) -> Self {
            Foo { s: data.into() }
        }
    }

    let a = Foo::from("bar");
    assert_eq!(a.s, "bar");
}

#[test]
fn powerful_use_case_for_functions() {
    fn concat<S, T>(a: S, b: T) -> String
    where
        S: Into<String>,
        T: Into<String>,
    {
        a.into() + &b.into()
    }

    assert_eq!(concat("a", "b"), String::from("ab"));
    assert_eq!(concat(String::from("a"), "b"), "ab");

    // assert_eq!(concat(10, "a"), "10a");  // uncomment me (1st)

    // uncomment me (2nd)
    // impl From<u32> for String {
    //     fn from(v: u32) -> Self {
    //         v.to_string()
    //     }
    // }

    // uncomment me (3rd)
    // Getting around the orphan rule.
    // struct Vec {
    //     inner: RawVec
    // }

    // impl From<Foo> for String {
    //     fn from(foo: Foo) -> Self {
    //         format!("{}", foo.0)
    //     }
    // }

    // assert_eq!(concat(Foo(10), "a"), "10a");
}

//
// Trait Trait Bounds
//

#[test]
fn trait_trait_bounds() {
    // This pattern we've already seen with the docs
    // for PartialEq & Eq, and Clone & Copy

    trait MaybeFoo {
        fn maybe_foo(&self) -> Option<bool>;
    }

    trait Foo: MaybeFoo {
        fn foo(&self) -> bool {
            self.maybe_foo().unwrap()
        }
    };

    struct MyStruct;

    // comment this block
    impl MaybeFoo for MyStruct {
        fn maybe_foo(&self) -> Option<bool> {
            Some(true)
        }
    }

    impl Foo for MyStruct {}
}

//
// Deriving trait impls
//

#[test]
fn derive_debug() {
    #[derive(Debug)] // <- comment this
    struct A;

    #[derive(Debug)] // procedural macro (custom derive)
    struct B {
        foo: A,
    }
}

#[test]
fn derive_multiple() {
    #[derive(Debug, PartialEq)]
    struct A;

    #[derive(Debug, PartialEq)]
    struct B {
        foo: A,
    }

    assert_eq!(B { foo: A }, B { foo: A })
}

//
// Generic traits and Associated Types
//
// - Want to show you traits with associated type for workshop
// - To do that I need to show you generic traits
// - This is considered Adv Rust (by The Book)

#[test]
fn generic_trait_partialeq() {
    // TODO: Jump to PartialEq docs.

    #[derive(Debug)]
    struct Foo;

    // Rhs = Right Hand Side
    impl PartialEq for Foo {
        fn eq(&self, other: &Self) -> bool {
            true
        }
    }

    let x = Foo;
    let y = Foo;
    assert_eq!(x, y);

    #[derive(Debug)]
    struct Bar;

    // 2nd impl of PartialEq for Foo
    impl PartialEq<Bar> for Foo {
        // uncomment me (2nd)
        fn eq(&self, other: &Bar) -> bool {
            true
        }
    }

    let z = Bar;
    assert_eq!(x, z); // uncomment me (1st)
}

#[test]
fn associated_types_add() {
    // TODO: Jump to Add docs.

    trait MyAdd {
        // Can have defaults (behind feature flag)
        // Can also specify trait bounds
        type Output;

        fn my_add(&self, other: &Self) -> Self::Output;
    }

    #[derive(Debug, PartialEq)]
    struct Foo(i32);

    impl MyAdd for Foo {
        type Output = Self;

        fn my_add(&self, other: &Self) -> Self::Output {
            Foo(self.0 + other.0)
        }
    }

    let x = Foo(10);
    let y = Foo(20);
    assert_eq!(x.my_add(&y), Foo(30));

    // TODO: Jump to Rust by Example link
    // https://doc.rust-lang.org/rust-by-example/generics/assoc_items/the_problem.html
}
