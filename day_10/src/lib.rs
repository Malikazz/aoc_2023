use std::collections::HashMap;

pub fn solve(input: Vec<String>) -> i32 {
    let maze: HashMap<Point, PipeNode> = parse_maze(input);
    let start: &PipeNode = maze.iter().filter(|a| a.1.is_start());
    println!("{:?}", maze);
    5
}

pub fn parse_maze(input: Vec<String>) -> HashMap<Point, PipeNode>{
    let mut maze: HashMap<Point, PipeNode> = HashMap::new();

    for (y_index, line) in input.iter().enumerate(){
        for (x_index, char) in line.chars().enumerate(){
        let point = Point{x:x_index, y:y_index};
            let char = String::from(char);
           maze.insert(point, PipeNode{ value: String::from(&char), visited: false, position: point, node_type:PipeNode::node_type_from_string(char) }); 
        }
    }
    maze
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct PipeNode {
    pub value: String,
    pub visited: bool,
    pub position: Point,
    pub node_type: NodeType
}

#[derive(Debug)]
pub enum NodeType{
    VerticalPipe,
    HorizontalPipe,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start
}

impl PipeNode{
    pub fn node_type_from_string(char:String) -> NodeType{
        match &char[0..] {
            "|" => NodeType::VerticalPipe,
            "-" => NodeType::HorizontalPipe,
            "L" => NodeType::NorthEast,
            "J" => NodeType::NorthWest,
            "7" => NodeType::SouthWest,
            "F" => NodeType::SouthEast,
            "." => NodeType::Ground,
            "S" => NodeType::Start,
            _ => panic!("from_char NodeType has failed to match")
        }
    }

    pub fn is_start(&self) -> bool{
        matches!(&self.node_type, NodeType::Start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_file_parser::read_lines;

    #[test]
    fn it_works() {
        let result = solve(read_lines("src/test_input"));
        assert_eq!(result, 8);
    }
}
