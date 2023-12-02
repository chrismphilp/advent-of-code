use std::fs;

pub fn process_answer() {
    println!("The sum for day twos simple cube game is: {}", part_1());
    println!("The sum for day twos minimum cube game is: {}", part_2());
}

fn part_1() -> i32 {
    fs::read_to_string("src/day_2/input.txt")
        .unwrap()
        .lines()
        .map(|v| process_invalid_games(v))
        .sum()
}

fn part_2() -> i32 {
    fs::read_to_string("src/day_2/input.txt")
        .unwrap()
        .lines()
        .map(|v| process_power_of_games(v))
        .sum()
}

fn process_invalid_games(line: &str) -> i32 {
    let split_line = line.split(":").collect::<Vec<&str>>();
    let game_id: i32 = split_line[0].trim().split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
    let rhs = split_line[1].trim().split(";").collect::<Vec<&str>>();

    for game in rhs {
        let cube_choices = game.trim().split(" ").collect::<Vec<&str>>();
        let mut i = 0;

        while i < cube_choices.len() {
            let numeric_value = cube_choices[i].parse::<i32>().unwrap();

            match cube_choices[i + 1] {
                "red" | "red," => {
                    if numeric_value > 12 {
                        return 0;
                    }
                }
                "green" | "green," => {
                    if numeric_value > 13 {
                        return 0;
                    }
                }
                "blue" | "blue," => {
                    if numeric_value > 14 {
                        return 0;
                    }
                }
                _ => {}
            }
            i += 2
        };
    }
    game_id
}

fn process_power_of_games(line: &str) -> i32 {
    let split_line = line.split(":").collect::<Vec<&str>>();
    let rhs = split_line[1].trim().split(";").collect::<Vec<&str>>();

    let mut min_red: i32 = 1;
    let mut min_green: i32 = 1;
    let mut min_blue: i32 = 1;

    for game in rhs {
        let cube_choices = game.trim().split(" ").collect::<Vec<&str>>();
        let mut i = 0;

        while i < cube_choices.len() {
            let numeric_value = cube_choices[i].parse::<i32>().unwrap();

            match cube_choices[i + 1] {
                "red" | "red," => {
                    if min_red < numeric_value {
                        min_red = numeric_value
                    }
                }
                "green" | "green," => {
                    if min_green < numeric_value {
                        min_green = numeric_value
                    }
                }
                "blue" | "blue," => {
                    if min_blue < numeric_value {
                        min_blue = numeric_value
                    }
                }
                _ => {}
            }
            i += 2
        };
    }
    min_red * min_green * min_blue
}

#[cfg(test)]
mod test {
    use crate::day_2::day_2::{process_invalid_games, process_power_of_games};

    #[test]
    fn should_find_correct_invalid_games() {
        assert_eq!(process_invalid_games(&String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")), 0);
        assert_eq!(process_invalid_games(&String::from("Game 15: 3 green, 1 blue, 5 red; 2 red; 1 red, 4 green")), 15);
        assert_eq!(process_invalid_games(&String::from("Game 100: 6 green, 15 red, 12 blue; 9 red; 16 red; 17 red, 3 blue, 7 green")), 0);
    }

    #[test]
    fn should_find_correct_power_of_games() {
        assert_eq!(process_power_of_games(&String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")), 48);
        assert_eq!(process_power_of_games(&String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")), 12);
        assert_eq!(process_power_of_games(&String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")), 1560);
        assert_eq!(process_power_of_games(&String::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red")), 630);
        assert_eq!(process_power_of_games(&String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")), 36);
    }
}
