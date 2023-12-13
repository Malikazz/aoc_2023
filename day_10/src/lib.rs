use std::{
    cmp::{max, min},
    collections::HashMap,
};

pub fn solve(input: Vec<String>) -> usize {
    let maze: HashMap<Point, PipeNode> = parse_maze(input);
    let mut visited_points: Vec<Point> = Vec::new();
    let mut s_points: Vec<Point> = Vec::new();

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

    for node in maze.iter() {
        if check_is_surrounded(&node.1, &maze, &visited_points) {
            s_points.push(Point {
                x: node.1.position.x,
                y: node.1.position.y,
            });
        }
    }

    for y in 0..140 {
        for x in 0..140 {
            print!("{}", maze.get(&Point { x, y }).unwrap().value);
        }
        print!("\n");
    }
    print!("\n");
    for y in 0..140 {
        for x in 0..140 {
            if s_points.contains(&Point { x, y }) {
                print!("\x1b[93mI\x1b[0m");
            } else if visited_points.contains(&Point { x, y }){
                print!("\x1b[92m{}\x1b[0m", maze.get(&Point { x, y }).unwrap().value);
            }else {
                print!("{}", maze.get(&Point { x, y }).unwrap().value);
            }
        }
        print!("\n");
    }

    println!("Part one: {:?}", visited_points.len() / 2);
    println!("Part two: {:?}", s_points.len());
    visited_points.len() / 2
}

pub fn check_is_surrounded(
    node: &PipeNode,
    nodes: &HashMap<Point, PipeNode>,
    visited_points: &Vec<Point>,
) -> bool {
    if visited_points.contains(&node.position) {
        return false;
    }

    // does move left hit a wall or node?
    let mut left = max(node.position.x as isize - 1, 0) as usize;
    let mut right = min(node.position.x + 1, 139);
    let mut down = min(node.position.y + 1, 139);
    let mut up = max(node.position.y as isize - 1, 0) as usize;

    loop {
        if visited_points.contains(
            &nodes
                .get(&Point {
                    x: left,
                    y: node.position.y,
                })
                .unwrap()
                .position,
        ) {
            break;
        }
        if left == 0 {
            return false;
        } else {
            left = left - 1;
        }
    }

    loop {
        if visited_points.contains(
            &nodes
                .get(&Point {
                    x: right,
                    y: node.position.y,
                })
                .unwrap()
                .position,
        ) {
            break;
        }
        if right == 139 {
            return false;
        } else {
            right = right + 1;
        }
    }

    loop {
        if visited_points.contains(
            &nodes
                .get(&Point {
                    x: node.position.x,
                    y: down,
                })
                .unwrap()
                .position,
        ) {
            break;
        }
        if down == 139 {
            return false;
        } else {
            down = down + 1;
        }
    }

    loop {
        if visited_points.contains(
            &nodes
                .get(&Point {
                    x: node.position.x,
                    y: up,
                })
                .unwrap()
                .position,
        ) {
            break;
        }
        if up == 0 {
            return false;
        } else {
            up = up - 1;
        }
    }

    true
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
