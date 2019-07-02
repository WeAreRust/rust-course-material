// Test

#[test]
fn this_is_a_test() {
    unimplemented!();
}

// Scopes

#[test]
fn scopes() {
    // We can use a block expression to create a scope
    if true {
        //                          --- start of scope
        let x = "foo";

        assert_eq!(x, "foo");
        //                          --- end of scope
    }

    // assert_eq!(x, "foo"); // uncomment me...
}

// The `mut` keyword

#[test]
fn mut_variable() {
    let x = 3;
    assert_eq!(x, 3);

    // Increment x
    // x += 1; // uncomment me...
    assert_eq!(x, 4);
}

#[test]
fn mut_argument() {
    let x = 3;

    fn inc(mutable: u8) -> u8 {
        // mutable += 1; // uncomment me...
        // try mutating x...
        return mutable;
    }

    let y = inc(x);
    assert_eq!(y, 4);
}

#[test]
fn mut_closure() {
    let x = 3;

    // A closure
    let inc_x = || {
        // x += 1; // uncomment me...
    };

    inc_x();
    assert_eq!(x, 4);
}

// Ownership & borrowing in detail

#[test]
fn ownership_scope() {
    // We can create a scope with curly braces
    {
        let x = 3;

        assert_eq!(x, 3);
    }

    // assert_eq!(x, 3); // uncomment me...
}

#[test]
fn ownership_move() {
    fn take_three(word: String) {
        assert_eq!(word, "three");
    }

    let x = String::from("three");
    take_three(x);

    // assert_eq!(x, "three"); // uncomment me...
}

#[test]
fn ownership_borrow() {
    let mut x = 3;

    let borrow_a = &x;
    let borrow_b = &x;
    // x += 1; // uncomment me...

    assert_eq!(borrow_a, &3);
    assert_eq!(borrow_b, &3);
    assert_eq!(x, 4);
}

#[test]
fn ownership_mutable_borrow() {
    fn inc_mutable(number: &mut u8) {
        *number += 1;
    }

    let mut x = 3;
    inc_mutable(&mut x);
    assert_eq!(x, 4);

    let borrow = &mut x;

    // let borrow_b = &x; // uncomment me...
    // x += 1;  // uncomment me...

    inc_mutable(borrow);
    assert_eq!(x, 5);
}

#[test]
fn ownership_closure_borrow() {
    let mut message = String::from("hello");
    let check_message = || {
        // this closure immutably borrows message (i.e. &message)
        assert_eq!(&message, "hello");
    };

    // message.push_str(" everyone!"); // uncomment me...
    check_message();
    message.push_str(" world");
    assert_eq!(message, String::from("hello world"));
}

#[test]
fn ownership_closure_mutable_borrow() {
    let mut message = String::from("hello");
    let mut add_world = || {
        // this closure mutably borrows x (i.e. &mut x)
        message.push_str(" world");
    };

    // assert_eq!(message, "hello"); // uncomment me...
    add_world();
    assert_eq!(message, "hello world");
}
#[test]
fn ownership_closure_move() {
    let mut message = String::from("Hello");
    let mut finish_message = move || {
        // this closure takes ownership of message
        message.push_str(" world");
    };

    // assert_eq!(message, "Hello"); // uncomment me...
    finish_message();
    // assert_eq!(message, "Hello world"); // uncomment me...
}

#[test]
fn ownership_return_borrow() {
    // // uncomment me...
    // fn bad_function() -> &u8 {
    //     let three = 3;
    //     &three
    // }

    // assert_eq!(bad_function(), &3);
}

// Lifetimes

#[test]
fn explicit_lifetimes() {
    fn shortest_name<'a>(first: &'a str, second: &'a str) -> &'a str {
        if first.len() <= second.len() {
            first
        } else {
            second
        }
    }

    {   
        // -- 'ben start
        let ben = String::from("Ben");

        let name = {
            // -- 'tristan start
            let tristan = String::from("Tristan");  

            let shortest = shortest_name(&ben, &tristan);

            assert_eq!(shortest, "Ben");

            "" // replace me with shortest...

            // -- 'tristan end
        };                                           

        assert_eq!(name, "Ben");

        // -- 'ben end
    }
}

// Generics

#[test]
fn generic_function() {
    fn first_generic<T>(a: T, b: T) -> T {
        // try replacing this with b < a
        if false {
            return b;
        }

        a
    }

    assert_eq!(first_generic(1, 2), 1);
}

#[test]
fn generic_enum() {
    #[derive(Debug)]
    enum Maybe<T> {
        NotNull(T),
        Null,
    }

    let _a: Maybe<u8> = Maybe::NotNull(1);
    let _b: Maybe<&str> = Maybe::NotNull("foo");
    let _c: Maybe<()> = Maybe::Null;
}

// Option type

#[test]
fn option_compare() {
    // Look familiar?
    let null_a: Option<()> = Option::None;
    let null_b: Option<()> = None;

    assert_eq!(null_a, null_b);

    let some_a = Some("foo");
    let some_b = Some("foo");

    assert_eq!(some_a, some_b);
}

#[test]
fn option_function() {
    /**
     * Return the bigger number. Returns "None" if numbers are equal.
     */
    fn bigger(a: u8, b: u8) -> Option<u8> {
        if a > b {
            Some(a)
        } else if b > a {
            Some(b)
        } else {
            None
        }
    }
    /**
     * Return double the bigger number. Returns "None" if numbers are equal.
     */
    fn double_bigger(a: u8, b: u8) -> Option<u8> {
        Some(2 * bigger(a, b)?) // <--- look!
    }
    assert_eq!(double_bigger(2, 5), Some(10));
}

// Result type

type SimpleError = &'static str; // ignore this for now

#[test]
fn result_function() {
    fn bad_function() -> u8 {
        // do stuff that might result in an error...
        panic!("Oh no!!!");
    }

    let result = bad_function();

    assert_eq!(result, 0);

    // do we get here or not? Why don't we force errors to be handled?

    fn good_function() -> Result<u8, SimpleError> {
        // do stuff that might result in an error...
        Err("Oh no!!!")
    }

    let result = good_function();

    if result.is_ok() {
        // safely do stuff with result!
    }

    assert_eq!(result, Err("Oh no!!!"));
}

#[test]
fn result_function_simpler() {
    fn do_risky_stuff() -> Result<String, SimpleError> {
        Ok(String::from("foo"))
    }

    fn do_more_stuff() -> Result<String, SimpleError> {
        let mut msg = do_risky_stuff()?;
        msg.push_str("bar");
        Ok(msg)
    }

    assert_eq!(do_more_stuff(), Ok(String::from("foobar")));
}

#[test]
fn result_expect() {
    fn should_never_fail() -> Result<String, SimpleError> {
        Ok(String::from("foo"))
    }

    let msg = should_never_fail().expect("should_never_fail has failed!");
    assert_eq!(&msg, "foo");
}

#[test]
fn result_unwrap() {
    fn should_never_fail() -> Result<String, SimpleError> {
        Ok(String::from("foo"))
    }

    let msg = should_never_fail().unwrap(); // probably don't ever do this outside tests...
    assert_eq!(&msg, "foo");
}

// Modules

mod my_module {
    struct PrivateStruct {
        number: u8,
    }

    // try making this public
    fn private_func() -> PrivateStruct {
        PrivateStruct { number: 3 }
    }

    pub fn public_func() -> u8 {
        3
    }

    #[test]
    fn module_test() {
        assert_eq!(private_func().number, 3);
    }
}

#[test]
fn test_my_module() {
    assert_eq!(my_module::public_func(), 3);

    // or even
    use my_module::public_func;

    assert_eq!(public_func(), 3);
}

#[cfg(test)]
mod tests {
    // this module will only be compiled if we're running tests
}
