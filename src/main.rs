fn main() {
    println!("{}", f(3.0));
}

fn f(x: f32) -> f32 {
    3.0 * x.powi(2) - 4.0 * x + 5.0
}
