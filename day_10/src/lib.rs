use std::collections::HashMap;

pub fn solve(input: Vec<String>) -> usize {
    let maze: HashMap<Point, PipeNode> = parse_maze(input);
    let mut visited_points: Vec<Point> = Vec::new();
    let start: &PipeNode = maze
        .iter()
        .filter(|a| a.1.is_start())
        .map(|a| a.1)
        .collect::<Vec<&PipeNode>>()[0];
    let mut current_node = start;

    loop {
        println!("Current node is {:?}", current_node);

        if let Some(temp) = move_next(&current_node, &maze, &visited_points) {
            current_node = temp;
            visited_points.push(Point{x:current_node.position.x, y:current_node.position.y});
        } else {
            break;
        }
    }
    visited_points.len() / 2
}

pub fn move_next<'a>(
    node: &'a PipeNode,
    nodes: &'a HashMap<Point, PipeNode>,
    visited_points: &Vec<Point>
) -> Option<&'a PipeNode> {
    match &node.node_type {
        NodeType::VerticalPipe => {
            let one = Point {
                x: node.position.x,
                y: node.position.y - 1,
            };
            let two = Point {
                x: node.position.x,
                y: node.position.y + 1,
            };
            if !visited_points.contains(&one){
               return Some(nodes.get(&one).unwrap()); 
            } else if !visited_points.contains(&two){
                return Some(nodes.get(&two).unwrap());
            }else {
                return None
            }
        }
        NodeType::HorizontalPipe => {
            let one = Point {
                x: node.position.x - 1,
                y: node.position.y,
            };
            let two = Point {
                x: node.position.x + 1,
                y: node.position.y,
            };
            if !visited_points.contains(&one){
               return Some(nodes.get(&one).unwrap()); 
            } else if !visited_points.contains(&two){
                return Some(nodes.get(&two).unwrap());
            }else {
                return None
            }
        }
        NodeType::NorthEast => {
            let one = Point {
                x: node.position.x,
                y: node.position.y - 1,
            };
            let two = Point {
                x: node.position.x + 1,
                y: node.position.y,
            };
            
            if !visited_points.contains(&one){
               return Some(nodes.get(&one).unwrap()); 
            } else if !visited_points.contains(&two){
                return Some(nodes.get(&two).unwrap());
            }else {
                return None
            }

        }
        NodeType::NorthWest => {
            let one = Point {
                x: node.position.x,
                y: node.position.y - 1,
            };
            let two = Point {
                x: node.position.x - 1,
                y: node.position.y,
            };
            
            if !visited_points.contains(&one){
               return Some(nodes.get(&one).unwrap()); 
            } else if !visited_points.contains(&two){
                return Some(nodes.get(&two).unwrap());
            }else {
                return None
            }

        }
        NodeType::SouthWest => {
            let one = Point {
                x: node.position.x - 1,
                y: node.position.y,
            };
            let two = Point {
                x: node.position.x,
                y: node.position.y + 1,
            };
            if !visited_points.contains(&one){
               return Some(nodes.get(&one).unwrap()); 
            } else if !visited_points.contains(&two){
                return Some(nodes.get(&two).unwrap());
            }else {
                return None
            }
        }
        NodeType::SouthEast => {
            let one = Point {
                x: node.position.x + 1,
                y: node.position.y,
            };
            let two = Point {
                x: node.position.x,
                y: node.position.y + 1,
            };
            if !visited_points.contains(&one){
               return Some(nodes.get(&one).unwrap()); 
            } else if !visited_points.contains(&two){
                return Some(nodes.get(&two).unwrap());
            }else {
                return None
            }
        }
        NodeType::Ground => panic!("your trying to move from a ground tile"),
        NodeType::Start => {
            // need to check if move it wants to make is valid
            let one = Point {
                x: node.position.x,
                y: node.position.y + 1,
            };
            let node_one = nodes.get(&one).unwrap();
            return Some(node_one);
        }
    }
}

pub fn parse_maze(input: Vec<String>) -> HashMap<Point, PipeNode> {
    let mut maze: HashMap<Point, PipeNode> = HashMap::new();

    for (y_index, line) in input.iter().enumerate() {
        for (x_index, char) in line.chars().enumerate() {
            let point = Point {
                x: x_index,
                y: y_index,
            };
            let char = String::from(char);
            maze.insert(
                point,
                PipeNode {
                    value: String::from(&char),
                    visited: false,
                    position: point,
                    node_type: PipeNode::node_type_from_string(char),
                },
            );
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
    pub node_type: NodeType,
}

#[derive(Debug, Clone, Copy)]
pub enum NodeType {
    VerticalPipe,
    HorizontalPipe,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl PipeNode {
    pub fn node_type_from_string(char: String) -> NodeType {
        match &char[0..] {
            "|" => NodeType::VerticalPipe,
            "-" => NodeType::HorizontalPipe,
            "L" => NodeType::NorthEast,
            "J" => NodeType::NorthWest,
            "7" => NodeType::SouthWest,
            "F" => NodeType::SouthEast,
            "." => NodeType::Ground,
            "S" => NodeType::Start,
            _ => panic!("from_char NodeType has failed to match"),
        }
    }

    pub fn is_start(&self) -> bool {
        matches!(&self.node_type, NodeType::Start)
    }
    pub fn visited(&mut self){
        self.visited = true;
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
