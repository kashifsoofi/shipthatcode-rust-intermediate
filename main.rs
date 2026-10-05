use std::io::{self, BufRead};

mod geometry {
    pub fn circle_area(radius: f64) -> f64 {
        3.14 * radius * radius
    }

    pub fn square_area(side: f64) -> f64 {
        side * side
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let kind: String = lines.next().unwrap().unwrap();
    let dim: f64 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let area = if kind.trim() == "circle" {
        geometry::circle_area(dim)
    } else {
        geometry::square_area(dim)
    };
    println!("{:.2}", area);
}
