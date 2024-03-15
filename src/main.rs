mod odd_calculator;
use crate::odd_calculator::calculate_odds;
fn main() {
    println!("{}", calculate_odds(10, 10, false, false));
    println!("Hello, world!");
}
