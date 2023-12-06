use std::cmp::min;
#[derive(Debug)]
pub struct Card {
    pub number: i32,
    pub player_numbers: Vec<i32>,
    pub game_numbers: Vec<i32>,
    pub points: i32,
    pub multiplier: i32,
}

pub fn solve(input: Vec<String>) -> i32 {
    let mut cards: Vec<Card> = parse_card(input);
    for card in cards.iter_mut() {
        for number in card.player_numbers.iter() {
            if card.game_numbers.contains(number) {
                if card.points > 0 {
                    card.points = card.points * 2;
                } else {
                    card.points = 1;
                }
            }
        }
    }
    for card in cards.iter() {
        println!("{:?}", card);
    }
    cards.iter().map(|card| card.points).sum()
}

pub fn solve_two(input: Vec<String>) -> i32 {
    let mut cards: Vec<Card> = parse_card(input);
    for card in cards.iter_mut() {
        for number in card.player_numbers.iter() {
            if card.game_numbers.contains(number) {
                card.points = card.points + 1;
            }
        }
    }
    for index in 0..cards.len() {
        let point_offset = min(
            (cards[index].points + index as i32 + 1) as usize,
            cards.len(),
        );
        if index + 1 >= cards.len() {
            break;
        }
        for _ in 0..cards[index].multiplier {
            for card in cards[index + 1..point_offset].iter_mut() {
                card.multiplier = card.multiplier + 1;
            }
        }
    }

    for card in cards.iter() {
        println!("{:?}", card);
    }

    cards.iter().map(|card| card.multiplier).sum()
}

pub fn parse_card(input: Vec<String>) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    let mut counter: i32 = 1;

    for line in input.iter() {
        let mut ticket_lines: Vec<Vec<i32>> = Vec::new();
        for part in line.split("|") {
            ticket_lines.push(parse_ticket_line(part));
        }
        cards.push(Card {
            number: counter,
            player_numbers: ticket_lines[0].to_owned(),
            game_numbers: ticket_lines[1].to_owned(),
            points: 0,
            multiplier: 1,
        });
        counter = counter + 1;
    }
    cards
}

pub fn parse_ticket_line(line: &str) -> Vec<i32> {
    let mut ticket_line: Vec<i32> = Vec::new();
    // skip over card part

    let mut line = &line.replace("  ", " ")[..];

    if line.contains(":") {
        line = line.split(":").nth(1).unwrap().trim();
    } else {
        line = line.trim();
    }
    for number in line.split(" ") {
        ticket_line.push(number.parse().unwrap());
    }
    ticket_line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_04_base_test() {
        let result = solve_two(vec![
            String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        ]);
        assert_eq!(result, 30);
    }
}
