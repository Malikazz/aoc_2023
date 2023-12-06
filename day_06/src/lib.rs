use regex::Regex;

#[derive(Debug)]
pub struct BoatRace {
    pub race_length: usize,
    pub race_distance: usize,
}

pub fn solve(input: Vec<String>) -> i32 {
    let boat_races = parse_race_info(input);
    println!("{:?}", boat_races);
    55555
}

pub fn calculate_race(input: BoatRace){

}


pub fn parse_race_info(input: Vec<String>) -> Vec<BoatRace> {
    let mut boat_races: Vec<BoatRace> = Vec::new();
    let regex: Regex = Regex::new(r"\d+").unwrap();

    for races in input.windows(2).step_by(2) {
        let line_one = regex
            .find_iter(&races[0][0..])
            .filter_map(|d| Some(d.as_str()))
            .collect::<Vec<&str>>();
        let line_two = regex
            .find_iter(&races[1][0..])
            .filter_map(|d| Some(d.as_str()))
            .collect::<Vec<&str>>();
        for item in 0..line_one.len() {
            boat_races.push(BoatRace {
                race_length: line_one[item].parse::<usize>().unwrap(),
                race_distance: line_two[item].parse::<usize>().unwrap(),
            });
        }
    }

    boat_races
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_file_parser::read_lines;
    #[test]
    fn it_works() {
        let result = solve(read_lines("day_06/src/test_input"));
        assert_eq!(result, 4);
    }
}
