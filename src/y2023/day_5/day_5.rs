use std::fs;

pub fn process_answer() {
    println!("The sum for day fives minimum location is: {}", part_1());
    // TODO: Fix part_2 brute force approach
    // println!("The sum for day fives overall minimum location is: {}", part_2());
}

fn part_1() -> i64 {
    let lines = fs::read_to_string("src/y2023/day_5/input.txt").unwrap();
    let input = lines.split("\n\n").collect::<Vec<&str>>();
    let seeds = input[0]
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let groups = input.iter()
        .skip(1)
        .map(|v| v.split('\n')
            .skip(1)
            .map(|v| v.split_whitespace())
            .map(|v| v.map(|v| v.parse::<i64>().unwrap()).collect::<Vec<i64>>())
            .collect::<Vec<Vec<i64>>>()
        )
        .collect::<Vec<Vec<Vec<i64>>>>();

    process_minimum_seed(seeds, groups)
}

fn process_minimum_seed(seeds: Vec<i64>, groups: Vec<Vec<Vec<i64>>>) -> i64 {
    seeds.iter().map(|v| find_min_seed(*v, &groups)).min().unwrap()
}

fn find_min_seed(seed: i64, groups: &Vec<Vec<Vec<i64>>>) -> i64 {
    let mut current_mapping: i64 = seed;

    for group in groups.iter() {
        for line in group {
            let dest_range = line[0];
            let src_range = line[1];
            let val_range = line[2];

            if (src_range <= current_mapping) && (current_mapping < src_range + val_range) {
                let idx = current_mapping - src_range;
                current_mapping = dest_range + idx;
                break;
            }
        }
    }
    current_mapping
}

fn part_2() -> i64 {
    let lines = fs::read_to_string("src/y2023/day_5/input.txt").unwrap();
    let input = lines.split("\n\n").collect::<Vec<&str>>();
    let seeds = input[0]
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let seed_pairs = seeds.chunks(2)
        .collect::<Vec<&[i64]>>();

    let groups = input.iter()
        .skip(1)
        .map(|v| v.split('\n')
            .skip(1)
            .map(|v| v.split_whitespace())
            .map(|v| v.map(|v| v.parse::<i64>().unwrap()).collect::<Vec<i64>>())
            .collect::<Vec<Vec<i64>>>()
        )
        .collect::<Vec<Vec<Vec<i64>>>>();

    seed_pairs.iter()
        .map(|v| (v[0]..v[1] + v[0]).into_iter().map(|x| find_min_seed(x, &groups)).min().unwrap())
        .min().unwrap()
}

#[cfg(test)]
mod test {
    use super::process_minimum_seed;

    #[test]
    fn should_find_correct_scratchcard_winning_values() {
        let seeds: Vec<i64> = vec!(79, 14, 55, 13);
        let groups: Vec<Vec<Vec<i64>>> = vec!(
            vec!(
                vec!(50, 98, 2),
                vec!(52, 50, 48)
            ),
            vec!(
                vec!(0, 15, 37),
                vec!(37, 52, 2),
                vec!(39, 0, 15)
            ),
            vec!(
                vec!(49, 53, 8),
                vec!(0, 11, 42),
                vec!(42, 0, 7),
                vec!(57, 7, 4)
            ),
            vec!(
                vec!(88, 18, 7),
                vec!(18, 25, 70)
            ),
            vec!(
                vec!(45, 77, 23),
                vec!(81, 45, 19),
                vec!(68, 64, 13)
            ),
            vec!(
                vec!(0, 69, 1),
                vec!(1, 0, 69)
            ),
            vec!(
                vec!(60, 56, 37),
                vec!(56, 93, 4)
            )
        );

        assert_eq!(process_minimum_seed(seeds, groups), 35);
    }
}
