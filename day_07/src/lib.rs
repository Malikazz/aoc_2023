use std::{collections::HashMap, cmp::Ordering};

#[derive(Debug)]
pub struct Hand {
    pub hand: String,
    pub bet: i32,
    pub hand_type: HandType
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHose,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    pub fn determine_hand(&self) -> HandType {
        let mut char_counts: HashMap<char, u8> = HashMap::new();
        for char in self.hand.chars() {
            if char_counts.contains_key(&char) {
                let current_count: &u8 = char_counts.get(&char).unwrap();
                char_counts.insert(char, current_count + 1);
            }else{
                char_counts.insert(char, 1);
            }
        }

        let mut char_counts_vec = char_counts.values().collect::<Vec<&u8>>();
        char_counts_vec.sort_by(|a,b | b.cmp(a));
        for char in char_counts_vec.iter() {
            match char {
                5 => return HandType::FiveOfAKind,
                4 => return HandType::FourOfAKind,
                3 => {
                    if char_counts.len() == 2 {
                        return HandType::FullHose;
                    } else {
                        return HandType::ThreeOfAKind;
                    }
                }
                2 => {
                    if char_counts.len() == 2 {
                        return HandType::FullHose;
                    } else if char_counts.len() == 3{
                        return HandType::TwoPair;
                    }else {
                        return HandType::OnePair;
                    }
                }
                1 => return HandType::HighCard,
                _ => panic!("determine hands failed"),
            }
        }
        panic!("determine hand failed with no elements?")
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let card_value = "AKQJT987654321";
        if &self.hand_type == &other.hand_type {
            for index in (0 as usize)..self.hand.len(){
                if card_value.match_indices(&self.hand[index..index+1]).collect::<Vec<_>>()[0] < card_value.match_indices(&other.hand[index..index+1]).collect::<Vec<_>>()[0]{
                    return Some(Ordering::Greater)                    
                } else if card_value .match_indices(&self.hand[index..index+1]).collect::<Vec<_>>()[0] > card_value.match_indices(&other.hand[index..index+1]).collect::<Vec<_>>()[0]{
                    return Some(Ordering::Less)
                }
            }
            return Some(Ordering::Equal)
        }
        if (self.hand_type as u8) < (other.hand_type as u8) {
            return Some(Ordering::Greater);
        }else {
            return Some(Ordering::Less);
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
         let card_value = "AKQJT987654321";
        if &self.hand_type == &other.hand_type {
            println!("Ord same hand type {:?}, {:?}",&self.hand_type, other.hand_type );
            for index in (0 as usize)..self.hand.len(){
                println!("Checking index {:?} of self {:?}", index, &self.hand[index..index+1] );
                println!("Checking index {:?} of other {:?}", index, &other.hand[index..index+1] );
                if card_value .match_indices(&self.hand[index..index+1]).collect::<Vec<_>>()[0] < card_value.match_indices(&other.hand[index..index+1]).collect::<Vec<_>>()[0]{
                    return Ordering::Greater;
                } else if card_value .match_indices(&self.hand[index..index+1]).collect::<Vec<_>>()[0] > card_value.match_indices(&other.hand[index..index+1]).collect::<Vec<_>>()[0]{
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        }
        if (self.hand_type as u8) < (other.hand_type as u8) {
            return Ordering::Greater;
        }else {
            return Ordering::Less;
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for Hand {}

pub fn solve(input: Vec<String>) -> i32 {
    let mut hands: Vec<Hand> = parse_hand(input);
    let mut score: i32 = 0;
    
    println!("Orignal hands: {:?}\n", hands);
    hands.sort_by(|a,b| b.cmp(a));
    println!("Sorted hands: {:?}\n", hands);
    // sort hands
    for index in 0..hands.len(){
       score = (hands[index].bet * (hands.len() as i32 - index as i32)) + score;
    }
    score
}

pub fn parse_hand(input: Vec<String>) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();

    for line in input.iter() {
        let trimmed_line = line.trim();
        let part = trimmed_line.split(" ").collect::<Vec<&str>>();
        hands.push(Hand {
            hand: String::from(part[0]),
            bet: part[1].parse::<i32>().unwrap(),
            hand_type: HandType::HighCard
        });
    }

    for hand in hands.iter_mut() {
        hand.hand_type = hand.determine_hand();
    }
    hands
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_file_parser::read_lines;
    #[test]
    fn it_works() {
        let result = solve(read_lines("src/test_input"));
        assert_eq!(result, 6440);
    }
}
