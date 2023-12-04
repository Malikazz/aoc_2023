use std::fs::read_to_string;

#[derive(Debug)]
pub struct Card{
    pub number: i32,
    pub player_numbers: Vec<i32>,
    pub game_numbers: Vec<i32>,
    pub points: i32
}


pub fn solve(input: Vec<String>) -> i32{
    let mut cards: Vec<Card> = parse_card(input);
    for card in cards.iter_mut(){
        for number in card.player_numbers.iter(){
            if card.game_numbers.contains(number){
                if card.points > 0 {
                    card.points = card.points * 2;
                }else {
                    card.points = 1;
                }
            }
        }
    }
    for card in cards.iter(){
      println!("{:?}", card);
    }
    cards.iter().map(| card| card.points).sum()
}

pub fn parse_card(input: Vec<String>) -> Vec<Card>{
    let mut cards: Vec<Card> = Vec::new();
    let mut counter: i32 = 1;
    
    for line in input.iter(){
        let mut ticket_lines: Vec<Vec<i32>> = Vec::new();
        for part in line.split("|"){
            ticket_lines.push(parse_ticket_line(part));        
        }
        cards.push(Card{number:counter, player_numbers: ticket_lines[0].to_owned(), game_numbers: ticket_lines[1].to_owned(), points: 0 });
        counter = counter +1;
    }
    cards
}

pub fn parse_ticket_line(mut line: &str) -> Vec<i32>{
    let mut ticket_line: Vec<i32> = Vec::new();
    // skip over card part
    
    let mut line = &line.replace("  ", " ")[..];

    if line.contains(":"){
        line = line.split(":").nth(1).unwrap().trim();
    } else{
        line = line.trim();
    }
    for number in line.split(" "){
        ticket_line.push(number.parse().unwrap());
    }
    ticket_line
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
    fn day_04_base_test() {
        let result = solve(vec![String::from("")]);
        assert_eq!(result, 4);
    }
}
