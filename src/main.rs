pub mod day2;
fn main() {
    let input = include_str!("../input/day2.txt");
    let part1_result = day2::part1(input.to_string());
    println!("Part 1: {}", part1_result);
    let part2_result = day2::part2(input.to_string());
    println!("Part 2: {}", part2_result);
}
