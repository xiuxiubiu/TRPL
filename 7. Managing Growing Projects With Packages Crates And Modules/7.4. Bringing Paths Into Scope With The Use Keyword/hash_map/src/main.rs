use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map);
}

use std::fmt;
use std::io;

#[allow(dead_code)]
fn funcion1() -> fmt::Result {
    Ok(())
}

#[allow(dead_code)]
fn funtion2() -> io::Result<()> {
    Ok(())
}

use std::fmt::Result;
use std::io::Result as IoResult;

#[allow(dead_code)]
fn funcion3() -> Result {
    Ok(())
}

#[allow(dead_code)]
fn funtion4() -> IoResult<()> {
    Ok(())
}
