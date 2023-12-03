use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
pub struct EnginePart {
    pub start: i32,
    pub end: i32,
    pub line_number: usize,
    pub value: String,
}

#[derive(Debug)]
pub struct Symbols {
    pub index: i32,
    pub line_number: usize,
    pub value: String,
}

pub fn solve(input: Vec<String>) -> i32 {
    let parts: Vec<EnginePart> = find_all_parts(&input);
    let symbols: Vec<Symbols> = find_all_symbols(&input);
    let mut solve_value: i32 = 0;

    'outer: for part in parts.iter() {
        for symbol in symbols.iter() {
            if !(part.line_number.abs_diff(symbol.line_number) <= 1
                && (part.start.abs_diff(symbol.index) <= 1 || part.end.abs_diff(symbol.index) <= 1))
            {
                continue;
            } else {
                println!("Did not skip {:?} {:?}", part, symbol);
                solve_value = solve_value + part.value.parse::<i32>().unwrap();
                continue 'outer;
            }
        }
    }

    solve_value
}

pub fn solve_two(input: Vec<String>) {
    let parts: Vec<EnginePart> = find_all_parts(&input);
    let symbols: Vec<Symbols> = find_all_symbols(&input);
    let mut gears: Vec<EnginePart> = Vec::new();

    for symbol in symbols.iter() {
        if symbol.value == String::from("*") {
            let mut temp_parts: Vec<EnginePart> = Vec::new();
            for part in parts.iter() {
                if !(part.line_number.abs_diff(symbol.line_number) <= 1
                    && (part.start.abs_diff(symbol.index) <= 1
                        || part.end.abs_diff(symbol.index) <= 1))
                {
                    continue;
                } else {
                    temp_parts.push(EnginePart{start:part.start, end: part.end, line_number:part.line_number, value:String::from(&part.value)});
                }
            }
            gears.append(&mut temp_parts);
            println!("TempParts: {:?}", temp_parts);
        }
    }
}

pub fn find_all_parts(input: &Vec<String>) -> Vec<EnginePart> {
    let regex_numbers = Regex::new(r"\d+").unwrap();
    let mut parts: Vec<EnginePart> = Vec::new();
    let mut count: usize = 0;
    for line in input.iter() {
        for part in regex_numbers.find_iter(line) {
            parts.push(EnginePart {
                start: part.start() as i32,
                end: (part.end() as i32) - 1,
                line_number: count,
                value: String::from(&line[part.start()..part.end()]),
            })
        }
        count = count + 1;
    }
    parts
}

pub fn find_all_symbols(input: &Vec<String>) -> Vec<Symbols> {
    let regex_symbols = Regex::new(r"[^\d\s.]").unwrap();
    let mut symbols: Vec<Symbols> = Vec::new();
    let mut count: usize = 0;

    for line in input.iter() {
        for item in regex_symbols.find_iter(line) {
            symbols.push(Symbols {
                index: item.start() as i32,
                line_number: count,
                value: String::from(&line[item.start()..item.start() + 1]),
            });
        }
        count = count + 1;
    }
    symbols
}
pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve(vec![
            String::from("467..114.."),
            String::from("...*......"),
            String::from("..35..633."),
            String::from("......#..."),
            String::from("617*......"),
            String::from(".....+.58."),
            String::from("..592....."),
            String::from("......755."),
            String::from("...$.*...."),
            String::from(".664.598.."),
        ]);
        assert_eq!(result, 4361);
    }
}
