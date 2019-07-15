// ====================================================================
// Tuple
// ====================================================================

#[test]
fn tuple_instantiate_destruct() {
    // Instantiating tuple
    let foo = (vec![1i32, 2, 3], String::from("hello"), &true);

    // Destructing
    let (a, b, c) = foo;
    assert_eq!(a, vec![1, 2, 3]);
    assert_eq!(b, String::from("hello"));
    assert_eq!(c, &true);

    // Destructing by default moves the value,
    // so we cannot destruct it again.
    // let (a, b, c) = foo;
}

#[test]
fn tuple_destruct_reference() {
    // Instantiate it again
    let foo = (vec![1i32, 2, 3], String::from("hello"), &true);

    // Destructing as reference
    let (a, b, c) = &foo;
    assert_eq!(a, &vec![1, 2, 3]);
    assert_eq!(b, &String::from("hello"));
    assert_eq!(c, &&true);

    // Destructing again as reference
    let (_a, _b, _c) = &foo;
}

#[test]
fn tuple_destruct_mut_reference() {
    // Instantiate it again but mutable
    let mut foo = (vec![1i32, 2, 3], String::from("hello"), &true);

    // Destructing as mutable reference
    let (a, b, c) = &mut foo;

    // Mutate the content via the reference
    a.push(4);
    b.push_str(" world");
    *c = &false;

    // ... and check the result
    assert_eq!(a, &vec![1, 2, 3, 4]);
    assert_eq!(b, &String::from("hello world"));
    assert_eq!(c, &&false);

    assert_eq!(
        foo,
        (vec![1, 2, 3, 4], String::from("hello world"), &false),
    );
}

#[test]
fn tuple_dot_notation_move() {
    // Instantiate it again
    let foo = (vec![1i32, 2, 3], String::from("hello"), &true);

    // Move the first value
    let a = foo.0;
    assert_eq!(a, vec![1, 2, 3]);

    // You cannot move it again
    // let a = foo.0;

    // But you can still move others
    let (_, _b, _c) = foo;
}

#[test]
fn tuple_dot_notation_reference() {
    // Instantiate it again
    let foo: (Vec<i32>, String, &bool) = (vec![1i32, 2, 3], String::from("hello"), &true);

    // Borrow the first value
    let a = &foo.0;
    assert_eq!(a, &vec![1, 2, 3]);

    // Or you can just use it directly
    assert_eq!(foo.0.len(), 3);

    // Move all values
    let (_a, _b, _c) = foo;
}

#[test]
fn tuple_dot_notation_mut_reference() {
    // Instantiate it again but mutable
    let mut foo: (Vec<i32>, String, &bool) = (vec![1i32, 2, 3], String::from("hello"), &true);

    // Borrow the first value mutably
    let a = &mut foo.0;
    a.push(4);
    // ... and push to the first value directly
    foo.0.push(5);

    assert_eq!(&foo.0, &vec![1, 2, 3, 4, 5]);
}

#[test]
fn tuple_unit_type() {
    // An empty tuple is unit type
    // it has only a single possible value
    let foo = ();
    assert_eq!(foo, ());

    // Function returning nothing actually returns a unit
    fn function_returns_nothing() {}
    assert_eq!(function_returns_nothing(), ());
}

// ====================================================================
// Struct
// ====================================================================

#[test]
fn struct_instantiate_all() {
    // Define a struct
    struct Foo {
        a: String,
        b: Vec<i32>,
    }

    // Instantiate via providing all values
    let foo1 = Foo {
        a: String::from("hello"),
        b: vec![1, 2, 3],
    };

    // Instantiate with shorthand
    let a = String::from("hello");
    let b = vec![1, 2, 3];
    let foo2 = Foo { a, b };

    assert_eq!(foo1.a, foo2.a);
    assert_eq!(foo1.b, foo2.b);
}

#[test]
fn struct_instantiate_from_others() {
    // Define a struct again
    struct Struct {
        a: String,
        b: Vec<i32>,
    }
    // Instantiate one
    let foo = Struct {
        a: String::from("hello"),
        b: vec![1, 2, 3],
    };

    // Instantiate another from the previous
    let bar = Struct {
        b: vec![1, 2],
        ..foo
    };

    // `foo.a` has been moved so not accessible
    // assert_eq!(foo.a, String::from("hello"));
    assert_eq!(foo.b, vec![1, 2, 3]);

    // `foo.a` is moved to `bar.a` here
    assert_eq!(bar.a, String::from("hello"));
    assert_eq!(bar.b, vec![1, 2]);
}

#[test]
fn struct_instantiate_default() {
    // Define a struct again with Default
    #[derive(Default)]
    struct Foo {
        a: String,
        b: Vec<i32>,
    }

    // Instantiate with default value
    let foo = Foo::default();
    assert_eq!(foo.a, String::new());
    assert_eq!(foo.b, vec![]);

    // Instantiate with some values from default
    let foo = Foo {
        a: String::from("hello"),
        ..Default::default()
    };
    assert_eq!(foo.a, String::from("hello"));
    assert_eq!(foo.b, vec![]);
}

#[test]
fn struct_field_access_move() {
    // Define and instantiate a struct again
    struct Foo {
        a: String,
        b: Vec<i32>,
    }
    let foo = Foo {
        a: String::from("hello"),
        b: vec![1, 2, 3],
    };

    // Move the field
    let a = foo.a;
    assert_eq!(a, String::from("hello"));

    // `foo.a` is moved to `a` above
    // assert_eq!(foo.a, String::from("hello"));

    // ... but `foo.b` is not moved
    assert_eq!(foo.b, vec![1, 2, 3]);
}

#[test]
fn struct_field_access_reference() {
    // Define and instantiate a struct again
    struct Foo {
        a: String,
        b: Vec<i32>,
    }
    let foo = Foo {
        a: String::from("hello"),
        b: vec![1, 2, 3],
    };

    // Reference `foo.a` instead of move
    let a = &foo.a;
    let b = &foo.b;
    assert_eq!(a, &String::from("hello"));
    assert_eq!(b, &vec![1, 2, 3]);
    // ... so you can still access it after
    assert_eq!(foo.a, String::from("hello"));
}

#[test]
fn struct_field_access_mut_reference() {
    // Define and instantiate a struct again but mut
    struct Foo {
        a: String,
        b: Vec<i32>,
    }
    let mut foo = Foo {
        a: String::from("hello"),
        b: vec![1, 2, 3],
    };

    // Borrow field mutably
    let a = &mut foo.a;
    a.push_str(" world");
    assert_eq!(foo.a, String::from("hello world"));

    let b = &mut foo.b;
    b.push(4);
    assert_eq!(foo.b, vec![1, 2, 3, 4]);
}

#[test]
fn struct_destruct_move() {
    // Define and instantiate a struct again
    struct Foo {
        a: String,
        b: Vec<i32>,
    }
    let foo = Foo {
        a: String::from("hello"),
        b: vec![1, 2, 3],
    };

    // Destruct the struct
    let Foo { a, b } = foo;
    assert_eq!(a, String::from("hello"));
    assert_eq!(b, vec![1, 2, 3]);

    // The value is moved above
    // assert_eq!(foo.a, String::from("hello"));
    // assert_eq!(foo.b, vec![1, 2, 3]);
}

#[test]
fn struct_destruct_reference() {
    // Define and instantiate a struct again
    struct Foo {
        a: String,
        b: Vec<i32>,
    }
    let foo = Foo {
        a: String::from("hello"),
        b: vec![1, 2, 3],
    };

    // Destruct the struct via borrow
    let Foo { a, b } = &foo;
    assert_eq!(a, &String::from("hello"));
    assert_eq!(b, &vec![1, 2, 3]);

    // You can still access the fields
    assert_eq!(foo.a, String::from("hello"));
    assert_eq!(foo.b, vec![1, 2, 3]);
}

#[test]
fn struct_tuple() {
    // Define a tuple struct
    struct Foo(String, Vec<i32>);
    // ... and instantiate it
    let foo = Foo(String::from("hello"), vec![1, 2, 3]);

    assert_eq!(foo.0, String::from("hello"));
    assert_eq!(foo.1, vec![1, 2, 3]);
}

#[test]
fn struct_unit_like() {
    // Define a unit-like struct
    struct Foo;
    // ... and instantiate it
    let _foo = Foo;
}

// ====================================================================
// Method & associated function
// ====================================================================

#[test]
fn method_reference() {
    // Define a struct
    struct Foo {
        a: String,
    }

    // Add an `impl` block for the struct
    impl Foo {
        // Define a method taking reference of the instance
        fn get_a(&self) -> &str {
            &self.a
        }
    }

    // Instantiate the struct
    let foo = Foo {
        a: String::from("hello"),
    };

    // Invoke the method
    assert_eq!(foo.get_a(), "hello");
}

#[test]
fn method_mut_reference() {
    // Define a struct again
    struct Foo {
        a: String,
    }

    // Add an `impl` block for the struct again
    impl Foo {
        // Define a method taking mutable reference of the instance
        fn append_to_a(&mut self, s: &str) {
            self.a.push_str(s);
        }
    }

    // Instantiate the struct with mut
    let mut foo = Foo {
        a: String::from("hello"),
    };

    // Invoke the method
    foo.append_to_a(" world");
    assert_eq!(foo.a, String::from("hello world"));
}

#[test]
fn method_ownership() {
    // Define a struct again
    struct Foo {
        a: String,
    }

    // Add an `impl` block for the struct again
    impl Foo {
        // Define a method taking ownership of the instance
        fn take_a(self) -> String {
            self.a
        }
    }

    // Instantiate the struct
    let foo = Foo {
        a: String::from("hello"),
    };

    // Invoke the method
    let a = foo.take_a();
    assert_eq!(a, String::from("hello"));

    // `foo` has been moved above
    // assert_eq!(foo.a, String::from("hello"));
}

#[test]
fn associated_function_basic() {
    // Define a struct again
    struct Foo {
        a: String,
    }

    // Add an `impl` block for the struct again
    impl Foo {
        // Define a function without taking an instance
        fn new(a: String) -> Self {
            Foo { a }
        }
    }

    // Use the function to create an instance
    let foo = Foo::new(String::from("hello"));
    assert_eq!(foo.a, String::from("hello"));
}

#[test]
fn associated_function_opt() {
    // Define a struct again
    struct Foo {
        a: String,
    }

    // Add an `impl` block for the struct again
    impl Foo {
        // Define a function returning `Option<Self>`
        fn new(a: String) -> Option<Self> {
            if a.is_empty() {
                None
            } else {
                Some(Foo { a })
            }
        }
    }

    // Use the function to create an instance
    let foo = Foo::new(String::from("hello"));
    assert_eq!(foo.unwrap().a, String::from("hello"));
}

// ====================================================================
// Privacy
// ====================================================================

#[test]
fn privacy_basics() {
    // Declare a `mod`
    mod inner {
        // Define a public struct
        pub struct Foo {
            a: String,
        }

        // Define a public function
        pub fn create_foo() -> Foo {
            Foo {
                a: String::from("hello"),
            }
        }

        impl Foo {
            // Define a public method
            pub fn get_a(&self) -> &str {
                &self.a
            }
        }
    }

    // `Foo::a` is private so we cannot instantiate
    // let foo = inner::Foo { a: String::from("hello") };

    // ... but we can create it via a `pub` function
    let foo = inner::create_foo();

    // ... however we still cannot access the inner
    // assert_eq!(foo.a, String::from("hello"));

    // ... but we can do so via a `pub` method
    assert_eq!(foo.get_a(), "hello");
}

// ====================================================================
// Enum
// ====================================================================

#[test]
fn enum_instantiate() {
    // Define an enum
    enum Foo {
        // ... has a variant with no data
        Unit,
        // ... and a variant with unnamed field
        Unnamed(String),
        // ... and a variant with named field
        NamedFields { _a: Vec<i32> },
    }

    // An instance can have one of the types
    let _foo1 = Foo::Unit;
    let _foo2 = Foo::Unnamed(String::from("hello"));
    let _foo3 = Foo::NamedFields { _a: vec![1, 2, 3] };
}

#[test]
fn enum_method() {
    // Define an enum again
    enum Foo {
        Unit,
        Unnamed(String),
        NamedFields { a: Vec<char> },
    }

    impl Foo {
        // Define a method of enum
        fn stringify(&self) -> String {
            match self {
                Foo::Unit => "unit".into(),
                Foo::Unnamed(s) => s.clone(),
                Foo::NamedFields { a } => a.iter().collect(),
            }
        }
    }

    assert_eq!(Foo::Unit.stringify(), "unit");
    assert_eq!(Foo::Unnamed("hello".into()).stringify(), "hello");
    assert_eq!(Foo::NamedFields { a: vec!['a', 'b'] }.stringify(), "ab");
}

// ====================================================================
// Pattern matching
// ====================================================================

#[test]
fn match_enum_move() {
    // Define an enum again
    enum Foo {
        _Unit,
        _Unnamed(String),
        NamedFields { a: Vec<char> },
    }

    // Instantiate it
    let foo = Foo::NamedFields { a: vec!['a'] };
    // ... and use match to destruct
    match foo {
        // Destruct the variant with variable binding
        Foo::NamedFields { a } => assert_eq!(a, vec!['a']),
        // Since we know we are not handling other variants,
        // just mark it unreachable here
        Foo::_Unit | Foo::_Unnamed(_) => unreachable!(),
    }

    // Cannot destruct it again because its moved above
    // match foo {
    //     _ => {}
    // }
}

#[test]
fn match_enum_reference() {
    // Define an enum again
    enum Foo {
        _Unit,
        _Unnamed(String),
        NamedFields { a: Vec<char> },
    }

    let foo = Foo::NamedFields { a: vec!['a'] };
    // ... and use match to destruct as reference
    match &foo {
        // The binding variable is now a reference
        Foo::NamedFields { a } => assert_eq!(a, &vec!['a']),
        _ => unreachable!(),
    }

    // It can be destruct again
    match &foo {
        _ => {}
    }
}

#[test]
fn match_enum_mut_reference() {
    // Define an enum again
    enum Foo {
        _Unit,
        _Unnamed(String),
        NamedFields { a: Vec<char> },
    }

    let mut foo = Foo::NamedFields { a: vec!['a'] };
    // ... and use match to destruct as reference
    match &mut foo {
        // The binding variable is now a mutable reference
        Foo::NamedFields { a } => a.push('b'),
        _ => unreachable!(),
    }

    // It can be destruct again
    match &foo {
        // The binding variable is now a reference
        Foo::NamedFields { a } => assert_eq!(a, &vec!['a', 'b']),
        _ => unreachable!(),
    }
}

#[test]
fn match_number() {
    let a = 10u8;
    match a {
        0 => unreachable!("not zero"),
        1..=9 => unreachable!("not single digit number"),
        10 => {}
        11..=255 => unreachable!("too large"),
    }
}

#[test]
fn match_string() {
    let a = "hello";
    match a {
        "hello" => {}
        "world" => unreachable!("not world"),
        _ => unreachable!("wrong!"),
    }
}

#[test]
fn match_omitting_and_guard() {
    // Define a struct
    struct Foo {
        a: String,
        b: Vec<i32>,
        c: bool,
    }

    // Instantiate it
    let foo = Foo {
        a: "hello".into(),
        b: vec![1, 2, 3],
        c: false,
    };

    // Do matching on it
    match &foo {
        // `..` can be used to omit irrelevant fields
        Foo { c: true, .. } => unreachable!("c is false"),
        // `if` can be used to guard a pattern
        Foo { a, b, .. } if a == "hello" && b == &[1, 2, 3] => {}
        _ => unreachable!("wrong!"),
    }
}

#[test]
fn match_nested() {
    // Define a struct
    struct Foo {
        a: (String, bool),
    }
    // ... and instantiate it
    let foo = Foo {
        a: ("hello".into(), true),
    };

    match &foo {
        // Pattern can be nested, and
        // `@` can be used to bind a sub-pattern with a variable
        Foo { a: tuple @ (_, true) } => {
            assert_eq!(tuple, &("hello".into(), true))
        }
        _ => unreachable!(),
    }
}

// ====================================================================
// Pattern elsewhere
// ====================================================================

#[test]
fn pattern_if_let() {
    let foo = Some(1);
    // Pattern matching can also be used with `if` via `if let`
    if let Some(foo) = foo {
        assert_eq!(foo, 1);
    } else {
        unreachable!();
    }
}

#[test]
fn pattern_while_let() {
    // Construct a linked-list
    struct ListNode {
        value: char,
        next: Option<Box<ListNode>>,
    }
    let mut foo = Some(Box::new(ListNode {
        value: 'a',
        next: Some(Box::new(ListNode {
            value: 'b',
            next: None,
        })),
    }));

    // Use `while let` to loop through the linked list
    let mut s = String::new();
    while let Some(node) = foo {
        s.push(node.value);
        foo = node.next;
    }
    assert_eq!(s, "ab");
}

#[test]
fn pattern_for() {
    let list = &[
        (1, 10),
        (3, 8),
        (5, 6),
    ];
    // `for` also can use pattern
    for (a, b) in list {
        assert_eq!(a + b, 11);
    }
}

#[test]
fn pattern_function_params() {
    struct Point(i32, i32);
    // Function params can also use pattern
    fn transpose(Point(x, y): Point) -> Point {
        Point(y, x)
    }

    let a = Point(1, 2);
    let b = transpose(a);
    assert_eq!(b.0, 2);
    assert_eq!(b.1, 1);
}

#[test]
fn pattern_legacy_ref() {
    let mut foo = (String::from("hello"), vec![1, 2, 3]);

    // Use `ref` to borrow
    let (ref a, ref b) = foo;
    assert_eq!(a, &String::from("hello"));
    assert_eq!(b, &vec![1, 2, 3]);

    // Use `ref mut` to mutably borrow
    let (ref mut a, _) = foo;
    a.push_str(" world");
    assert_eq!(foo.0, String::from("hello world"));
}