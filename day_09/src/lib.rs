pub fn solve(input: Vec<String>) -> isize {
    let numbers: Vec<Vec<isize>> = parse_input(&input, false);
    let mut next_numbers: Vec<isize> = Vec::new();

    let mut next_numbers_rev: Vec<isize> = Vec::new();
    let mut numbers_rev: Vec<Vec<isize>> = parse_input(&input, true);

    for line in numbers.iter() {
        next_numbers.push(build_history(&line));
    }

    for line in numbers_rev.iter() {
        next_numbers_rev.push(build_history(&line));
    }

    println!("Part one {:?}", next_numbers.iter().sum::<isize>());
    println!("Part two {:?}", next_numbers_rev.iter().sum::<isize>());
    next_numbers.iter().sum()
}

pub fn build_history(numbers: &Vec<isize>) -> isize {
    let mut history: Vec<Vec<isize>> = Vec::new();
    history.push(numbers.clone());

    while history
        .last()
        .unwrap()
        .iter()
        .filter_map(|a| if a != &0 { Some(a) } else { None })
        .collect::<Vec<&isize>>()
        .len()
        > 0
    {
        history.push(find_differences(history.last().unwrap()));
    }
    //println!("Histories {:?}", history);
    history.iter().map(|a| a.last().unwrap()).sum()
}

pub fn find_differences(numbers: &Vec<isize>) -> Vec<isize> {
    let mut differences: Vec<isize> = Vec::new();

    for number in numbers.windows(2) {
        differences.push(number[1] - number[0]);
    }
    differences
}

pub fn parse_input(input: &Vec<String>, rev: bool) -> Vec<Vec<isize>> {
    let mut numbers_lists: Vec<Vec<isize>> = Vec::new();

        for line in input.iter() {
            numbers_lists.push(split_to_vec(&line, " ", rev));
    }

    numbers_lists
}

pub fn split_to_vec(input: &String, split_by: &str, rev: bool) -> Vec<isize> {
    let mut numbers: Vec<isize> = Vec::new();
    if rev {

    for part in input.split(split_by).collect::<Vec<&str>>().iter().rev() {
        numbers.push(part.parse::<isize>().unwrap());
    }
    }else {

    for part in input.split(split_by) {
        numbers.push(part.parse::<isize>().unwrap());
    }
    }
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_file_parser::read_lines;
    #[test]
    fn it_works() {
        let result = solve(read_lines("src/test_input"));
        assert_eq!(result, 114);
    }
}
