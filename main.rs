use std::io::{self, BufRead};

fn parse_two(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    // TODO: parse BOTH `a` and `b` as i32 and return Ok of their sum.
    // Use `?` on each parse so a bad input returns its Err to main
    // instead of panicking.
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let a = lines.next().unwrap().unwrap();
    let b = lines.next().unwrap().unwrap();
    match parse_two(&a, &b) {
        Ok(n) => println!("sum: {}", n),
        Err(_) => println!("error: invalid input"),
    }
}
