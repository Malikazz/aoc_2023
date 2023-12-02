use std::fs::read_to_string;

#[derive(Debug)]
pub struct CubeColors {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

#[derive(Debug)]
pub struct Game {
    pub round_cubes: Vec<CubeColors>,
    pub game_number: i32,
}

pub fn solve(input: Vec<String>) -> i32 {
    let mut games: Vec<Game> = Vec::new();
    let mut valid_games: i32 = 0;

    let cube_maxes = CubeColors {
        red: 12,
        green: 13,
        blue: 14,
    };
    
    games = parse_game(input);
    for game in games.iter(){
        println!("Checking game {:?} {:?}", game.game_number, game);
        let mut game_was_possible: bool = true;
        for round in game.round_cubes.iter(){
            println!("Checking round {:?}", round);
            if round.green > cube_maxes.green || round.red > cube_maxes.red || round.blue > cube_maxes.blue{
                println!("{:?} was not possible", round);
                game_was_possible = false;
            }
        }
        if game_was_possible {
            valid_games += game.game_number;
        }
    }
    valid_games
}

pub fn parse_game(input: Vec<String>) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    let mut count: i32 = 1;
    for line in input.iter() {
        // set game number
        let mut temp_game: Game = Game {
            round_cubes: Vec::new(),
            game_number: count,
        };

        // get rounds
        for round in line.split(";") {
            temp_game.round_cubes.push(parse_round(round));
        }
        games.push(temp_game);
        count = count +1;
    }
    games
}

pub fn parse_round(input: &str) -> CubeColors {
    // at this point we have something like 
    // Game 3: 8 green, 6 blue, 20 red;
    let red_indicies: Vec<_> = input.match_indices("red").collect();
    let blue_indicies: Vec<_> = input.match_indices("blue").collect();
    let green_indicies: Vec<_> = input.match_indices("green").collect();

    let mut red: i32 = 0;
    let mut green: i32 = 0;
    let mut blue: i32 = 0;
    
    if red_indicies.len() > 0 {
        red = input[red_indicies[0].0 - 3..red_indicies[0].0 -1].trim_start()
            .parse()
            .unwrap()
    }
    if blue_indicies.len() > 0 {
        blue = input[blue_indicies[0].0 - 3..blue_indicies[0].0 -1].trim_start()
            .parse()
            .unwrap()
    }
    if green_indicies.len() > 0 {
        green = input[green_indicies[0].0 - 3..green_indicies[0].0 -1].trim_start()
            .parse()
            .unwrap()
    }
    CubeColors {
        red,
        blue,
        green
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
    fn it_works() {
        let result = solve(vec![
            String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            String::from(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            ),
            String::from(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            ),
            String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        ]);
        assert_eq!(result, 8);
    }
}
