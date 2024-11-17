use std::fmt::Result;
use std::io::Result as IoResult;

fn foo1() -> Result {
    return Ok(());
}

fn foo2() -> IoResult<()> {
    return Ok(());
}

fn main() {
    let _ = foo1();
    let _ = foo2();
}
