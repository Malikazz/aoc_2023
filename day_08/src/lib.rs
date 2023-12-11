use std::collections::{HashMap, VecDeque};

pub fn solve(input: Vec<String>) -> usize {
    let map: CamelMap = parse_input(input);
    let start_positions: Vec<String> = vec![
        String::from("AAA"),
        String::from("RLA"),
        String::from("QLA"),
        String::from("QFA"),
        String::from("RXA"),
        String::from("JSA"),
    ];
    let mut counts: Vec<usize> = Vec::new();

    for pos in start_positions.iter() {
        counts.push(search(pos, &map));
    }

    println!("{:?}", counts);
    lcm(&counts[0..])
}

pub fn search(start: &str, map: &CamelMap) -> usize {
    let mut cloned_map = map.clone();
    let mut current_position = String::from(start);
    let mut current_count = 0;

    while &current_position[2..3] != "Z" {
        print!(
            "Currently at {:?} {:?}",
            current_position,
            &map.step.get(&current_position)
        );

        let temp_index = cloned_map.left_right.pop_front().unwrap();
        cloned_map.left_right.push_back(temp_index);

        print!("moving to the {:?} ", temp_index);

        current_position = String::from(&map.step.get(&current_position).unwrap()[temp_index]);

        println!("Moved to {:?}", current_position);

        current_count = current_count + 1;
    }
    current_count
}
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[derive(Debug, Clone)]
pub struct CamelMap {
    pub start: String,
    pub left_right: VecDeque<usize>,
    pub step: HashMap<String, [String; 2]>,
}

pub fn parse_input(input: Vec<String>) -> CamelMap {
    let mut map: CamelMap = CamelMap {
        start: String::from(""),
        left_right: input[0]
            .chars()
            .map(|a| if a == 'L' { 0 } else { 1 })
            .collect::<VecDeque<usize>>(),
        step: HashMap::new(),
    };

    for line in input[2..].iter() {
        if map.start == "" {
            map.start = String::from(&line[0..3]);
        }
        map.step.insert(
            String::from(&line[0..3]),
            [String::from(&line[7..10]), String::from(&line[12..15])],
        );
    }
    map
}

// L/R can just be 0,1 and can index a structure

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_file_parser::read_lines;
    #[test]
    fn it_works() {
        let result = solve(read_lines("src/test_input"));
        assert_eq!(result, 6);
    }
}
