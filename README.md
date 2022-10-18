# crust
C + Rust
CRust

Rust with C-like syntax

Example:
```rust
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
fn funcwithargs(a: i32, b: i32) -> () {
    println!("{} + {} = {}", a, b, a+b);
}

pub fn testfunc() -> () {
    let test: i32 = 5;
    println!("I am a test function with int {}", test);
    funcwithargs(1, 2);
}
```
