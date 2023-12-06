pub fn part1(input: String) -> u32 {
    let numbers: Vec<u32> = input.lines().map(|line| parse_line(line)).collect();
    let sum = numbers.iter().sum::<u32>();
    sum
}

// pub fn part2(input: string) -> u32 {
//     12
// }

fn parse_line(line: &str) -> u32 {
    let line = replace_digits(line);
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    match digits.as_slice() {
        [first] => *first * 10 + *first,
        digits => {
            let first = digits.first().unwrap();
            let last = digits.last().unwrap();
            first * 10 + last
        }
    }
}

fn replace_digits(line: &str) -> String {
    let digit_words = [
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let mut line = line.to_string();
    let mut i = 0;

    while i < line.len() {
        let mut replaced = false;
        for &(word, digit) in &digit_words {
            if line[i..].starts_with(word) {
                line.replace_range(i..i + word.len() - 1, digit);
                i += digit.len();
                replaced = true;
                break;
            }
        }
        if !replaced {
            i += 1;
        }
    }
    line
}
