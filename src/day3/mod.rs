use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
struct Num {
    line: usize,
    col: usize,
    num: i32,
}

pub fn part1(input: String) -> i32 {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let numbers = input
        .lines()
        .enumerate()
        .map(|(i, line)| parse_line(i, line));
    let mut total = 0;
    for line in numbers {
        for num in line {
            if !check_num(&num, &lines) {
                total += num.num;
            }
        }
    }
    total
}

fn check_num(num: &Num, matrix: &[Vec<char>]) -> bool {
    let token_set = ['#', '*', '+', '$']; // Define the tokens to check for
    let row_len = matrix.len();
    let col_len = matrix[0].len();

    for row_offset in num.line.saturating_sub(1)..=num.line + 1 {
        if row_offset >= row_len {
            continue;
        }
        for col_offset in num.col.saturating_sub(1)..=num.col + num.num.to_string().len() {
            if col_offset >= col_len
                || (row_offset == num.line
                    && col_offset >= num.col
                    && col_offset < num.col + num.num.to_string().len())
            {
                continue;
            }

            if token_set.contains(&matrix[row_offset][col_offset]) {
                return false;
            }
        }
    }

    true
}
fn parse_line(i: usize, line: &str) -> Vec<Num> {
    let re = Regex::new(r"\d+").unwrap();
    // Make a vecor of the numbers and their positions
    let numbers = re
        .find_iter(line)
        .filter_map(|mat| {
            let start_idx = mat.start();
            mat.as_str().parse::<i32>().ok().map(|num| (start_idx, num))
        })
        .collect::<Vec<(usize, i32)>>()
        .iter()
        .map(|(start_idx, num)| Num {
            line: i,
            col: *start_idx,
            num: *num,
        })
        .collect::<Vec<Num>>();
    numbers
}

pub fn part2(input: String) -> i32 {
    32
}
