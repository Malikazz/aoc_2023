
pub fn solve(input: Vec<String>) -> i32{
    5 
}

pub fn expand_universe(input: Vec<String>){
     
}

pub fn parse_input(input: Vec<String>){
    let mut points: Vec<Point> = Vec::new();
    for (index, line) in input.iter().enumerate(){
        let matched_indexs = line.match_indices("#");
        for matches in matched_indexs{
           points.push(Point{y: index, x:matches.0});
        }
    } 
}

pub struct Point{
    pub x: usize,
    pub y: usize
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
