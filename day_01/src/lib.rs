use std::fs::read_to_string;
#[derive(Debug)]
struct Numbers {
    first: i32,
    last: i32,
}

struct NumberPostion {
    number: i32,
    postion: usize,
}

pub fn part_one(input: Vec<String>) -> i32 {
    let mut numbers: Vec<Numbers> = Vec::new();

    for item in input.iter() {
        let mut number: Numbers = Numbers {
            first: -1,
            last: -1,
        };
        for letter in item.chars() {
            if char::is_digit(letter, 10) {
                if number.first == -1 {
                    number.first = char::to_digit(letter, 10).unwrap() as i32;
                    number.last = char::to_digit(letter, 10).unwrap() as i32;
                } else {
                    number.last = char::to_digit(letter, 10).unwrap() as i32;
                }
            }
        }
        numbers.push(number);
    }

    let mut solve: i32 = 0;
    for item in numbers.iter() {
        solve = solve
            + format!("{:?}{:?}", item.first, item.last)
                .parse::<i32>()
                .unwrap();
    }
    solve
}

pub fn find_by_char(item: String) {
    for letter in item.chars() {
        if char::is_digit(letter, 10) {
            if number.first == -1 {
                number.first = char::to_digit(letter, 10).unwrap() as i32;
                number.last = char::to_digit(letter, 10).unwrap() as i32;
            } else {
                number.last = char::to_digit(letter, 10).unwrap() as i32;
            }
        }
    }
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
}
