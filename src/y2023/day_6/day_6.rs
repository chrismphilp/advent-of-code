use std::fs;

pub fn process_answer() {
    println!("The sum for day six winning boats is: {}", part_1());
    println!("The total for day six single boat race is: {}", part_2());
}

fn part_1() -> i64 {
    let input = fs::read_to_string("src/y2023/day_6/input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let times = lines[0].split_whitespace().skip(1).map(|v| v.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let record_distances = lines[1].split_whitespace().skip(1).map(|v| v.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    find_winning_boats(times, record_distances)
}

fn find_winning_boats(times: Vec<i64>, records: Vec<i64>) -> i64 {
    times.iter().enumerate()
        .map(|(i, v)| find_number_of_times_boat_beat_record(*v, records[i]))
        .filter(|v| *v > 0)
        .product()
}

fn find_number_of_times_boat_beat_record(time: i64, record: i64) -> i64 {
    let mut winning_counts = 0;

    for curr in 0..time {
        let button_time = time - curr;
        let distance_travelled = button_time * (time - button_time);

        if distance_travelled > record {
            winning_counts += 1;
        }
    }
    winning_counts
}

fn part_2() -> i64 {
    let input = fs::read_to_string("src/y2023/day_6/input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let time = lines[0].split_whitespace().skip(1).map(String::from).reduce(|a, b| format!("{}{}", a, b)).map(|v| v.parse::<i64>().unwrap()).unwrap();
    let record_distance = lines[1].split_whitespace().skip(1).map(String::from).reduce(|a, b| format!("{}{}", a, b)).map(|v| v.parse::<i64>().unwrap()).unwrap();

    find_number_of_times_boat_beat_record(time, record_distance)
}

#[cfg(test)]
mod test {
    use super::{find_number_of_times_boat_beat_record, find_winning_boats};

    #[test]
    fn should_find_correct_record_beating_boats() {
        let times = vec!(7, 15, 30);
        let distances = vec!(9, 40, 200);
        assert_eq!(find_winning_boats(times, distances), 288);
    }

    #[test]
    fn should_find_correct_single_boat_record() {
        assert_eq!(find_number_of_times_boat_beat_record(71530, 940200), 71503);
    }
}
