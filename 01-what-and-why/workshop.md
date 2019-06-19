# Workshop 1

## 1. Install Rust

Head to https://www.rust-lang.org/tools/install for instructions, or run:

```sh
curl https://sh.rustup.rs -sSf | sh
```

This will install `rustup`, the Rust toolchain manager. This tool installs and upgrades all the tools you need to build Rust code locally.

Check that the Rust compiler is installed correctly with:

```
rustc --version
```

## 2. Create a new Cargo project

This command creates a new `hello` directory, initialises a directory structure for the project, and creates a source file that just outputs a hello world.

```sh
cargo new hello
```

## 3. Build and run your new project

There's no need to explicitly build your project, `cargo` will automatically build your project when you run `cargo run`.

```sh
cd hello
cargo run
```

If you do want to build but not run, you can use `cargo build`. You can also tell it to just do a typecheck (which is sometimes faster) using `cargo check`

## 4. Create a unit test

Unit testing is built into Rust. Create a test inside `main.rs`:

```rust
#[test]
fn test_passes() {
  assert_eq!(1 + 1, 2);
}
```

Then run:

```sh
cargo test
```

Now change the assertion so that it fails.

### What's the `!` symbol after the function name mean?

The `!` indicates that it's not actually a function, but rather a macro. The compiler runs some Rust code at compile time which expands `assert_eq!(...)` into some other code. Macros allow Rust to do some pretty powerful things at compile-time to avoid run-time cost.

## 5. Install a crate, read the docs

Open `Cargo.toml` and add this line to the `[dependencies]` section:

```
rand = "0.6.5"
```

That specifies the name of the crate we want and the version. You can see the crate's listing on crates io: https://crates.io/crates/rand

Let's take a look at the documentation for the `rand` crate:

```
cargo doc --open
```

Cargo will automatically detect that in order to generate documentation for your project, it first needs to fetch the `rand` dependency.

Once it's done generating the docs for your project, those docs should open in your browser (that's the `--open` flag).

## 6. Generate and display a random number

Using the `rand` documentation, try to generate a random number and print it to the screen.

## 7a. (Extra challenge) Generate and output some JSON:

Install the serde crate (the go to crate for serialization and deserialization in Rust), and the JSON implementation by adding these to your dependencies:

```
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

Using `serde`, generate some JSON from a data structure:

```rust
#[derive(Serialize, Deserialize)]
struct Bookmark {
    id: u32,
    title: String,
    url: String,
}

fn main() {
    let bookmark = Bookmark {
        id: 1,
        title: "Cat pics".into(),
        url: "https://www.canva.com/photos/search/cats/".into(),
    };

    // TODO: Print the JSON representation of the above data using serde
}
```

The `#[derive(Serialize, Deserialize)]` is another compile time macro (it's actually two - one for Serialize and one for Deserialize). These macros examine the struct at compile time and generate a bunch of code to do the serializing/deserializing.

## 7b. Parse some JSON

Parse some JSON back into the data structure and print the title to the console:

```rust
let json = "{ \"id\": 2, \"title\": \"Dog pics\", \"url\": \"...\" }";

// TODO: Print just the title
```
