use std::collections::HashMap;
use std::fs;

pub fn process_answer() {
    println!("The sum for day sevens total winning hands is: {}", part_1());
    println!("The sum for day sevens advanced total winning hands is: {}", part_2());
}

fn part_1() -> i32 {
    let input = fs::read_to_string("src/y2023/day_7/input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let card_values: HashMap<char, i32> = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);

    process_hands(lines, &card_values, false)
}

fn process_hands(lines: Vec<&str>, card_values: &HashMap<char, i32>, is_advanced_case: bool) -> i32 {
    let mut hands: Vec<(Vec<char>, i32, i32)> = lines.iter()
        .map(|v| v.split_whitespace())
        .map(|v| v.collect::<Vec<&str>>())
        .map(|v| {
            let value_of_hand = match is_advanced_case {
                true => find_advanced_rank_value_of_hand(v[0], card_values),
                false => find_simple_rank_value_of_hand(v[0])
            };
            (v[0].chars().collect::<Vec<char>>(), v[1].parse::<i32>().unwrap(), value_of_hand)
        })
        .collect::<Vec<(Vec<char>, i32, i32)>>();

    hands.sort_unstable_by(|a, b| {
        if a.2 == b.2 {
            for i in 0..a.0.len() {
                let card_a = card_values.get(&a.0[i]).unwrap();
                let card_b = card_values.get(&b.0[i]).unwrap();
                if card_a != card_b {
                    return card_a.cmp(card_b);
                }
            }
        }
        a.2.cmp(&b.2)
    });

    hands.into_iter().enumerate()
        .map(|(i, v)| (i + 1) as i32 * v.1)
        .sum()
}

fn part_2() -> i32 {
    let input = fs::read_to_string("src/y2023/day_7/input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let card_values: HashMap<char, i32> = HashMap::from([
        ('J', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);

    process_hands(lines, &card_values, true)
}

fn find_simple_rank_value_of_hand(hand: &str) -> i32 {
    let sets = hand.chars()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

    find_set_value(sets)
}

fn find_advanced_rank_value_of_hand(hand: &str, card_values: &HashMap<char, i32>) -> i32 {
    let mut hand_chars = hand.chars().collect::<Vec<char>>();
    hand_chars.sort_by(|a, b| {
        card_values.get(b).unwrap().cmp(card_values.get(a).unwrap())
    });

    let sets = hand_chars.iter()
        .fold(HashMap::new(), |mut acc: HashMap<char, i32>, c| {
            if *c == 'J' {
                let most_common_char: char = acc.iter()
                    .max_by(|a, b| a.1.cmp(b.1))
                    .map(|(a, _)| *a)
                    .unwrap_or_else(|| 'A');
                *acc.entry(most_common_char).or_insert(0) += 1;
            } else {
                *acc.entry(*c).or_insert(0) += 1;
            }
            acc
        });

    find_set_value(sets)
}

fn find_set_value(sets: HashMap<char, i32>) -> i32 {
    if sets.len() == 1 {
        return 7; // Five of a kind
    } else if sets.len() == 2 {
        if sets.iter().any(|(_, y)| *y > 3) {
            return 6; // Four of a kind
        } else if sets.iter().any(|(_, y)| *y > 2) {
            return 5; // Full house
        }
    } else if sets.len() == 3 {
        if sets.iter().any(|(_, y)| *y > 2) {
            return 4; // Three of a kind
        } else if sets.iter().filter(|(_, y)| **y > 1).count() == 2 {
            return 3; // Two pair
        }
    } else if sets.len() == 4 && sets.iter().filter(|(_, y)| **y > 1).count() == 1 {
        return 2; // Single pair
    }
    1
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::{find_advanced_rank_value_of_hand, find_simple_rank_value_of_hand, process_hands};

    #[test]
    fn should_find_individual_correct_standard_hand_values() {
        assert_eq!(find_simple_rank_value_of_hand("KK677"), 3);
        assert_eq!(find_simple_rank_value_of_hand("KTJJT"), 3);
        assert_eq!(find_simple_rank_value_of_hand("32T3K"), 2);
        assert_eq!(find_simple_rank_value_of_hand("AAAAA"), 7);
        assert_eq!(find_simple_rank_value_of_hand("AAAAK"), 6);
        assert_eq!(find_simple_rank_value_of_hand("AAAKK"), 5);
    }

    #[test]
    fn should_find_correct_hand_rankings() {
        let card_values: HashMap<char, i32> = HashMap::from([
            ('2', 2),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9),
            ('T', 10),
            ('J', 11),
            ('Q', 12),
            ('K', 13),
            ('A', 14),
        ]);

        let lines: Vec<&str> = vec!(
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483"
        );

        assert_eq!(process_hands(lines, &card_values, false), 6440);
    }

    #[test]
    fn should_find_individual_correct_advanced_hand_values() {
        let card_values: HashMap<char, i32> = HashMap::from([
            ('J', 1),
            ('2', 2),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9),
            ('T', 10),
            ('Q', 11),
            ('K', 12),
            ('A', 13),
        ]);

        assert_eq!(find_advanced_rank_value_of_hand("KK677", &card_values), 3);
        assert_eq!(find_advanced_rank_value_of_hand("KTJJT", &card_values), 6);
        assert_eq!(find_advanced_rank_value_of_hand("32T3K", &card_values), 2);
        assert_eq!(find_advanced_rank_value_of_hand("JJJJJ", &card_values), 7);
        assert_eq!(find_advanced_rank_value_of_hand("AAAAK", &card_values), 6);
        assert_eq!(find_advanced_rank_value_of_hand("AAAKK", &card_values), 5);
    }
}
