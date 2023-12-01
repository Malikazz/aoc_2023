use std::fs::read_to_string;
#[derive(Debug)]
pub struct NumberPostion {
    pub number: i32,
    pub postion: usize,
}

pub fn part_one(input: Vec<String>) -> i32 {
    let mut numbers_numbers: Vec<Vec<NumberPostion>> = Vec::new();
    for item in input.iter() {
        let mut numbers: Vec<NumberPostion> = Vec::new();
        numbers.append(&mut find_by_char(item));
        numbers.append(&mut find_by_str(item));
        numbers.sort_by(|a, b| a.postion.partial_cmp(&b.postion).unwrap());
        numbers_numbers.push(numbers);
    }
    let mut solve: i32 = 0;
    let mut count:usize = 0;
    for item in numbers_numbers.iter() {
        count = count + 1;
        solve = solve
            + format!("{:?}{:?}", item[0].number, item[item.len() - 1].number)
                .parse::<i32>()
                .unwrap();
    }
    solve
}

pub fn find_by_str(item: &String) -> Vec<NumberPostion> {
    let mut numbers: Vec<NumberPostion> = Vec::new();
    let string_numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for find in string_numbers.iter() {
        let v: Vec<_> = item.match_indices(find).collect();
        for thing in v.iter(){
            numbers.push(NumberPostion {
                postion: thing.0,
                number: str_to_number(find),
            })
        }
    }
    numbers
}

pub fn str_to_number(thing: &str) -> i32 {
    match thing {
        "one" => return 1,
        "two" => return 2,
        "three" => return 3,
        "four" => return 4,
        "five" => return 5,
        "six" => return 6,
        "seven" => return 7,
        "eight" => return 8,
        "nine" => return 9,
        _ => panic!("how i here"),
    }
}

pub fn find_by_char(item: &String) -> Vec<NumberPostion> {
    let mut numbers: Vec<NumberPostion> = Vec::new();
    let mut count: usize = 0;
    for letter in item.chars() {
        if char::is_digit(letter, 10) {
            numbers.push(NumberPostion {
                number: char::to_digit(letter, 10).unwrap() as i32,
                postion: count,
            });
        }
        count = count + 1;
    }
    numbers
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
    fn part_one_test() {
        let result = part_one(vec![
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet"),
        ]);
        assert_eq!(result, 142);
    }

    #[test]
    fn part_two_test() {
        let result = part_one(vec![
            String::from("two1nine"),
            String::from("eightwothree"),
            String::from("abcone2threexyz"),
            String::from("xtwone3four"),
            String::from("4nineeightseven2"),
            String::from("zoneight234"),
            String::from("7pqrstsixteen"),
        ]);
        assert_eq!(result, 281);
    }
    #[test]
    fn test_three() {
        let result = part_one(vec![
            String::from("onetwo"),
            String::from("threefour"),
            String::from("fivesix"),
            String::from("seveneight"),
            String::from("nine"),
        ]);
        assert_eq!(result, 279);
    }
    
    #[test]
    fn test_four() {
        let result = part_one(vec![
            String::from("12"),
            String::from("34"),
            String::from("56"),
            String::from("78"),
            String::from("9"),
        ]);
        assert_eq!(result, 279);
    }
    
    #[test]
    fn test_five() {
        let result = part_one(vec![
            String::from("1asdf2fasdf"),
            String::from("3asdf4asdf"),
            String::from("asdf5asdf6asdf"),
            String::from("asdf7fasdf8asf"),
            String::from("asdf9fasdf"),
        ]);
        assert_eq!(result, 279);
    }

    #[test]
    fn test_six() {
        let result = part_one(vec![
            String::from("one1asd2"),
            String::from("three3asdf4"),
            String::from("five5asdf6"),
            String::from("seven7asdf8"),
            String::from("nineasdf9asdf"),
        ]);
        assert_eq!(result, 279);
    }

    #[test]
    fn test_seven() {
        let result = part_one(vec![
           String::from("fouronevhnrz44"),
            String::from("eightg1"),
            String::from("4ninejfpd1jmmnnzjdtk5sjfttvgtdqspvmnhfbm"),
            String::from("78seven8"),
            String::from("6pcrrqgbzcspbd"),
            String::from("7sevenseven"),
            String::from("1threeeight66"),
            String::from("one1sevensskhdreight"),
            String::from("rninethree6"),
            String::from("eight45fourfgfive1")
        ]);
        assert_eq!(result, 602);
    }
    
    #[test]
    fn bug_test() {
        let result = part_one(vec![
            String::from("eight45fourfgfive1eight")
        ]);
        assert_eq!(result, 88);
    }
}
