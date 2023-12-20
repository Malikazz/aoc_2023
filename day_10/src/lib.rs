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
        if let Some(temp) = move_next(&current_node, &maze, &visited_points) {
            current_node = temp;
            visited_points.push(Point {
                x: current_node.position.x,
                y: current_node.position.y,
            });
        } else {
            break;
        }
    }

    println!("Part one: {:?}", visited_points.len() / 2);
    println!("Part two: {:?}", solve_two(visited_points));
    5
}

pub fn solve_two(visited_nodes: Vec<Point>) -> f64 {
    // Shoelace
    let mut sum1: f64 = 0.0;
    let mut sum2: f64 = 0.0;

    for item in visited_nodes[0..visited_nodes.len()].windows(2) {
        sum1 = sum1 + (item[0].x * item[1].y) as f64;
        sum2 = sum2 + (item[0].y * item[1].x) as f64;
    }
    
    sum1 = sum1 + (visited_nodes[visited_nodes.len() -1].x * visited_nodes[0].y) as f64;
    sum2 = sum2 + (visited_nodes[0].x * visited_nodes[visited_nodes.len() -1].y) as f64;

    let area = f64::abs(sum1 - sum2) / 2.0;
   
    println!("area {:?}", area);
    println!("visted nodes len {:?}", visited_nodes.len());
    // pick therom
    // A = I + B/2 -1
    // area = I + visited_nodes.len() /2 - 1
    // area +1 = I + visited_nodes.len() /2 
    // area * 2 + 2 = 2I + visited_nodes.len()
    // area * 2 + 2 - visited_nodes.len() = 2I
    // (area * 2 + 2 - visited_nodes.len()) /2 = I
    area - 0.5 * visited_nodes.len() as f64 + 1.0
    
}

pub fn move_next<'a>(
    node: &'a PipeNode,
    nodes: &'a HashMap<Point, PipeNode>,
    visited_points: &Vec<Point>,
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
            if !visited_points.contains(&one) {
                return Some(nodes.get(&one).unwrap());
            } else if !visited_points.contains(&two) {
                return Some(nodes.get(&two).unwrap());
            } else {
                return None;
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
            if !visited_points.contains(&one) {
                return Some(nodes.get(&one).unwrap());
            } else if !visited_points.contains(&two) {
                return Some(nodes.get(&two).unwrap());
            } else {
                return None;
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

            if !visited_points.contains(&one) {
                return Some(nodes.get(&one).unwrap());
            } else if !visited_points.contains(&two) {
                return Some(nodes.get(&two).unwrap());
            } else {
                return None;
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

            if !visited_points.contains(&one) {
                return Some(nodes.get(&one).unwrap());
            } else if !visited_points.contains(&two) {
                return Some(nodes.get(&two).unwrap());
            } else {
                return None;
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
            if !visited_points.contains(&one) {
                return Some(nodes.get(&one).unwrap());
            } else if !visited_points.contains(&two) {
                return Some(nodes.get(&two).unwrap());
            } else {
                return None;
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
            if !visited_points.contains(&one) {
                return Some(nodes.get(&one).unwrap());
            } else if !visited_points.contains(&two) {
                return Some(nodes.get(&two).unwrap());
            } else {
                return None;
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
