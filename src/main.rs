pub mod day1;
fn main() {
    let input = include_str!("../input/day1.txt");
    let result = day1::part1(input.to_string());
    println!("Result: {}", result);
}
