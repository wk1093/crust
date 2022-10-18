# crust
C + Rust
CRust

Rust with C-like syntax

Example:
```rust
// sample.crs
void funcwithargs(i32 a, i32 b) {
    println!("{} + {} = {}", a, b, a+b);
}

pub void testfunc() {
    const i32 test = 5;
    println!("I am a test function with int {}", test);
    funcwithargs(1, 2);
}
```
Will be converted to
```rust
// sample.rs
fn funcwithargs(a: i32, b: i32) -> () {
    println!("{} + {} = {}", a, b, a+b);
}

pub fn testfunc() -> () {
    let test: i32 = 5;
    println!("I am a test function with int {}", test);
    funcwithargs(1, 2);
}
```

CRust files can be imported with the macro `crust::i!(FILE);`
You could import this example with `crust::i!(sample);`

Here is a basic Cargo file for a project that uses crust

```toml
[package]
name = "crustest"
version = "0.1.0"
edition = "2021"
build = "crust_build.rs"

[build-dependencies]
crust = { git = "https://github.com/wk1093/crust" }

[dependencies]
crust = { git = "https://github.com/wk1093/crust" }
```

And Here is the build script

```rust
fn main() {
    crust::build();
}
```

And a basic main.rs using this sample library

```rust
crust::i!(sample);

fn main() {
    println!("Hello, world!");
    sample::testfunc();
}
```
