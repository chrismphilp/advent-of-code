use std::fs;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_1() {
    let mut sum = 0;

    for line in fs::read_to_string("src/day_1/input.txt").unwrap().lines() {
        let curr = line.to_string();
        let i = find_calibration_value(curr);
        sum += i;
    }

    println!("The sum for day ones simple calibration is: {}", sum);
}

pub fn part_2() {
    let mut sum = 0;

    for line in fs::read_to_string("src/day_1/input.txt").unwrap().lines() {
        let curr = line.to_string();
        let i = find_advanced_forward_calibration_value(curr.clone()) * 10 + find_advanced_backward_calibration_value(curr.clone());
        sum += i;
    }

    println!("The sum for day ones advanced calibration is: {}", sum);
}

fn find_calibration_value(line: String) -> i32 {
    let mut lhs = line.chars()
        .filter(|v| v.is_digit(10))
        .next()
        .map(|v| v.to_string())
        .unwrap();

    let rhs = line.chars()
        .rev()
        .filter(|v| v.is_digit(10))
        .next()
        .map(|v| v.to_string())
        .unwrap();

    lhs.push_str(&rhs);

    lhs.parse::<i32>().unwrap()
}

fn find_advanced_forward_calibration_value(line: String) -> i32 {
    let char_vec: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < char_vec.len() {
        if char_vec[i].is_digit(10) {
            return char_vec[i].to_digit(10).unwrap() as i32;
        } else {
            for (j, d) in DIGITS.iter().enumerate() {
                let chars: Vec<char> = d.chars().collect();
                if char_vec[i..].starts_with(&chars) {
                    return (j as i32) + 1
                }
            }
        }

        i += 1
    }
    0
}

fn find_advanced_backward_calibration_value(line: String) -> i32 {
    let char_vec: Vec<char> = line.chars().collect();
    let mut i: i32 = char_vec.len() as i32 - 1;

    while i >= 0 {
        if char_vec[i as usize].is_digit(10) {
            return char_vec[i as usize].to_digit(10).unwrap() as i32;
        } else {
            for (j, d) in DIGITS.iter().enumerate() {
                let chars: Vec<char> = d.chars().collect();
                if char_vec[(i as usize)..].starts_with(&chars) {
                    return j as i32 + 1
                }
            }
        }

        i -= 1
    }
    0
}

#[cfg(test)]
mod test {
    use super::find_advanced_forward_calibration_value;
    use super::find_advanced_backward_calibration_value;

    #[test]
    fn should_find_correct_forward_values() {
        assert_eq!(find_advanced_forward_calibration_value(String::from("5")), 5);
        assert_eq!(find_advanced_forward_calibration_value(String::from("8onethree7")), 8);
        assert_eq!(find_advanced_forward_calibration_value(String::from("onethree7")), 1);
        assert_eq!(find_advanced_forward_calibration_value(String::from("onthree7")), 3);
        assert_eq!(find_advanced_forward_calibration_value(String::from("4onthree7")), 4);
    }

    #[test]
    fn should_find_correct_backward_values() {
        assert_eq!(find_advanced_backward_calibration_value(String::from("5")), 5);
        assert_eq!(find_advanced_backward_calibration_value(String::from("8onethree7")), 7);
        assert_eq!(find_advanced_backward_calibration_value(String::from("onethree")), 3);
        assert_eq!(find_advanced_backward_calibration_value(String::from("1onthre")), 1);
        assert_eq!(find_advanced_backward_calibration_value(String::from("onesix827")), 7);
        assert_eq!(find_advanced_backward_calibration_value(String::from("onesixaaa")), 6);
    }
}
