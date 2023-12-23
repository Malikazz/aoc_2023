use std::iter::repeat;

pub fn solve(input: Vec<String>) -> i32 {
    let input = parse_input(input); 

    5
}

pub fn expand_universe(mut input: Vec<String>) -> Vec<String>{
    let row_count = input[0].len();
    let mut vert_expansions: Vec<Vec<usize>> = Vec::new();
    for index in (0..input.len()).rev() {
        vert_expansions.push(
            input[index]
                .match_indices(".")
                .map(|x| x.0)
                .collect::<Vec<usize>>(),
        );
        // Handles horizontal expansion
        if vert_expansions.last().unwrap().len() == row_count{
            input.insert(index, repeat(".").take(row_count).collect::<String>());
        }
    }
    println!("vert expansions {:?}", vert_expansions);
    let vert_expansions: Vec<usize> = vert_expansions.iter().filter(|x| x.len() == row_count).map(|x| x[0]).collect::<Vec<usize>>();
    println!("vert expansions {:?}", vert_expansions);
    for line_index in (0..input.len()).rev(){
        for index in vert_expansions.iter().rev(){
            input[line_index].insert(*index, '.');
        }
    }
    for line in input.iter(){
        println!("{:?}", line);
    }
    input
}

pub fn parse_input(input: Vec<String>) -> Vec<String>{
    let mut points: Vec<Point> = Vec::new();
    for (index, line) in input.iter().enumerate() {
        let matched_indexs = line.match_indices("#");
        for matches in matched_indexs {
            points.push(Point {
                y: index,
                x: matches.0,
            });
        }
    }
    expand_universe(input)
}

pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_file_parser::read_lines;

    #[test]
    fn it_works() {
        let result = solve(read_lines("src/test_input"));
        assert_eq!(result, 374);
    }
}
