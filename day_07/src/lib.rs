use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
pub struct Hand {
    pub hand: String,
    pub bet: i32,
    pub hand_type: HandType,
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
#[derive(Debug)]
pub struct LetterCounts {
    pub letter: char,
    pub count: i32
}

impl Hand {
    pub fn determine_hand(&self) -> HandType {
        let mut char_counts: HashMap<char, i32> = HashMap::new();
        for char in self.hand.chars() {
            if char_counts.contains_key(&char) {
                let current_count: &i32 = char_counts.get(&char).unwrap();
                char_counts.insert(char, current_count + 1);
            } else {
                char_counts.insert(char, 1);
            }
        }
        
        // find the highest not j value and add to it 


        let mut char_counts_vec = char_counts.iter().map(|(k, v)| LetterCounts{letter:*k, count:*v}).collect::<Vec<LetterCounts>>();

        char_counts_vec.sort_by(|a, b| b.count.cmp(&a.count));
        if let Some(j_count) = char_counts.get(&'J') {
            for _ in 0..*j_count {
                for index in 0..char_counts_vec.len() {
                    if char_counts_vec[index].count < 5 && char_counts_vec[index].letter != 'J' {
                        let temp = char_counts_vec[index].count +1;
                        char_counts_vec[index].count = temp;
                        break;
                    }
                }
            }
        }

        char_counts_vec.sort_by(|a, b| b.count.cmp(&a.count));
        for char in char_counts_vec.iter() {
            match char.count {
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
                    } else if char_counts.len() == 3 {
                        return HandType::TwoPair;
                    } else {
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
        let card_value = "AKQT987654321J";
        if &self.hand_type == &other.hand_type {
            for index in (0 as usize)..self.hand.len() {
                if card_value
                    .match_indices(&self.hand[index..index + 1])
                    .collect::<Vec<_>>()[0]
                    < card_value
                        .match_indices(&other.hand[index..index + 1])
                        .collect::<Vec<_>>()[0]
                {
                    return Some(Ordering::Greater);
                } else if card_value
                    .match_indices(&self.hand[index..index + 1])
                    .collect::<Vec<_>>()[0]
                    > card_value
                        .match_indices(&other.hand[index..index + 1])
                        .collect::<Vec<_>>()[0]
                {
                    return Some(Ordering::Less);
                }
            }
            return Some(Ordering::Equal);
        }
        if (self.hand_type as u8) < (other.hand_type as u8) {
            return Some(Ordering::Greater);
        } else {
            return Some(Ordering::Less);
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let card_value = "AKQT987654321J";
        if &self.hand_type == &other.hand_type {
            for index in (0 as usize)..self.hand.len() {
                if card_value
                    .match_indices(&self.hand[index..index + 1])
                    .collect::<Vec<_>>()[0]
                    < card_value
                        .match_indices(&other.hand[index..index + 1])
                        .collect::<Vec<_>>()[0]
                {
                    return Ordering::Greater;
                } else if card_value
                    .match_indices(&self.hand[index..index + 1])
                    .collect::<Vec<_>>()[0]
                    > card_value
                        .match_indices(&other.hand[index..index + 1])
                        .collect::<Vec<_>>()[0]
                {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        }
        if (self.hand_type as u8) < (other.hand_type as u8) {
            return Ordering::Greater;
        } else {
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

    hands.sort_by(|a, b| b.cmp(a));
    for hand in hands.iter(){
        println!("Sorted hands: {:?}\n", hand);
    }
    // sort hands
    for index in 0..hands.len() {
        println!("Hand: {:?} math {:?} * {:?}", hands[hands.len()-index -1], hands[hands.len()-index -1].bet, index + 1);
        score = hands[hands.len()-index -1].bet * (index + 1)as i32 + score;
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
            hand_type: HandType::HighCard,
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
        assert_eq!(result, 5905);
    }
    #[test]
    fn bug_one(){
        let hand:Hand = Hand{hand: String::from("J58JJ"), bet: 19, hand_type: HandType::HighCard};
        let result = hand.determine_hand();
        assert_eq!(result, HandType::FourOfAKind);
    }
    #[test]
    fn bug_one_two(){
        let hand:Hand = Hand{hand: String::from("JJJJJ"), bet: 19, hand_type: HandType::HighCard};
        let result = hand.determine_hand();
        assert_eq!(result, HandType::FiveOfAKind);
    }

    #[test]
    fn bug_one_three(){
        let result = solve(vec![String::from("JJJJJ 3"), String::from("AAAAA 6"), String::from("AAAAJ 8"), String::from("KKKK4 5")]);
        assert_eq!(result, 59);
    } 
}
