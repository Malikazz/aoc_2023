use std::collections::{HashMap, VecDeque};

pub fn solve(input: Vec<String>) -> i32 {
    let mut map: CamelMap = parse_input(input);
    let mut current_position = String::from(&map.start);
    let mut current_count = 0;

    println!("Begining processing with {:?}", map);

    while current_position != String::from("ZZZ") {
        print!("Currently at {:?} ", current_position);
        
        let temp_index = map.left_right.pop_front().unwrap();
        map.left_right.push_back(temp_index);
        
        print!("moving to the {:?} ", if temp_index == 0 { "left" } else { "right" });

        current_position =
        String::from(&map.step.get(&current_position).unwrap()[map.left_right[temp_index]]);

        println!("Moved to {:?}", current_position);

        current_count = current_count + 1;
    }
    current_count
}

#[derive(Debug)]
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
