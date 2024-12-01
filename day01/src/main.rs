const PUZZLE_INPUT: &str = include_str!("../puzzle_input.txt");

fn main() {
    let mut part1_result: isize = 0;
    let mut part2_result: isize = 0;
    let mut left_list: Vec<isize> = Vec::with_capacity(1000);
    let mut right_list: Vec<isize> = Vec::with_capacity(1000);
    for line in PUZZLE_INPUT.lines() {
        if let Some((left, right)) = line.split_once(char::is_whitespace) {
            left_list.push(left.trim().parse().unwrap_or_default());
            right_list.push(right.trim().parse().unwrap_or_default());
        }
    }
    left_list.sort();
    right_list.sort();

    for (i, left) in left_list.iter().enumerate() {
        for (j, right) in right_list.iter().enumerate() {
            if i == j {
                part1_result += (left - right).abs();
            }
            if left == right {
                part2_result += right;
            }
        }
    }

    println!("{part1_result} {part2_result}");
}
