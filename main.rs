use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();

    // TODO: replace the 0 below with ONE iterator chain over
    //   line.split_whitespace()  ->  parse each token as i32
    //                            ->  keep only the even values
    //                            ->  square them
    //                            ->  add them up
    // The `: i32` on `total` is what tells the final step which
    // integer type to produce - keep it.
    let total: i32 = line
        .split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .sum();

    println!("{}", total);
}
