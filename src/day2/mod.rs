struct Game {
    id: i32,
    games: Vec<Round>,
}

struct Round {
    blue: i32,
    red: i32,
    green: i32,
}

// CONSTANTS FOR GAMES: 12 red, 13 green, 14 blue
//
const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

pub fn part1(input: String) -> i32 {
    // First we parse the line
    let games: Vec<Game> = input.lines().map(|line| parse_line(line)).collect();
    let mut sum = 0;
    for game in games {
        sum += game.id;
    }
    sum
}

fn parse_line(line: &str) -> Game {
    // We deconstruct the line into the following variables:
    //Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // id, and a vector of games
    let game_id = line.split(":").next().unwrap();
    let id = game_id.split(" ").last().unwrap();
    // We then parse the games into a vector of GAMES
    let games: Vec<&str> = line
        .split(":")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .split(";")
        .map(|game| game.trim())
        .collect();
    let mut game_rounds: Vec<Round> = vec![];
    for game in games {
        let rounds: Vec<&str> = game.split(",").collect();
        for round in rounds {
            // We get the number of blue, red and green
            let colors: Vec<&str> = round.split(",").collect();
            let mut round = Round {
                blue: 0,
                red: 0,
                green: 0,
            };
            // We now get the number of each color
            colors.iter().for_each(|color| {
                let color_values = color.trim().split(" ").collect::<Vec<&str>>();
                let color = color_values[0].parse::<i32>().unwrap();
                match color_values.last().unwrap().trim() {
                    "blue" => round.blue = color,
                    "red" => round.red = color,
                    "green" => round.green = color,
                    _ => (),
                }
            });
            game_rounds.push(round);
        }
    }
    let mut game = Game {
        id: id.parse::<i32>().unwrap(),
        games: game_rounds,
    };
    // We iterate through the games and if the numbers are higher than our constants we return 0,
    // otherwise we return the id of the game
    game.games.iter().for_each(|round| {
        if round.blue > BLUE || round.red > RED || round.green > GREEN {
            game.id = 0;
        }
    });
    game
}

pub fn part2(input: String) -> i32 {
    let games: Vec<Game> = input.lines().map(|line| parse_line(line)).collect();
    let mut maximums: Vec<(i32, i32, i32)> = vec![];
    for game in games {
        let mut temp = (0, 0, 0);
        for round in game.games {
            if round.blue > temp.0 {
                temp.0 = round.blue;
            }
            if round.red > temp.1 {
                temp.1 = round.red;
            }
            if round.green > temp.2 {
                temp.2 = round.green;
            }
        }
        maximums.push(temp);
    }
    let mut result = 0;
    for colors in maximums {
        result += colors.0 * colors.1 * colors.2;
    }
    result
}
