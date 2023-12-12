use std::fs;

pub fn process_answer() {
    println!("The sum for day eights  is: {}", part_1());
}

fn part_1() -> i32 {
    let input = fs::read_to_string("src/y2023/day_7/input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    0
}

fn find_number_of_steps() -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::find_number_of_steps;

    #[test]
    fn should_find_individual_correct_standard_hand_values() {
        assert_eq!(find_number_of_steps(), 3);
    }
}
