use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;
    let mut max_hand = "";
    let mut max_bid = 0;
    let mut max_rank = 0;

    for line in reader.lines() {
        let new_line = line.unwrap();
        let hand: Vec<&str> = new_line.split(" ").collect();
        let rank = get_rank(hand[0].clone());
        if rank > max_rank || (rank == max_rank && compare_hands(hand[0], max_hand) > 0) {
            result += max_rank * max_bid;
            max_hand = hand[0].clone();
            max_rank = rank;
            let temp = hand[1].clone();
            max_bid = temp.parse::<usize>().unwrap();
        } else {
            result += rank * hand[1].parse::<usize>().unwrap();
        }
    }

    println!("{}", result);

    Ok(())
}  

fn get_rank(hand: &str) -> usize {
    let mut cards: Vec<i32> = vec![];
    for card in hand.chars() {
        cards.push(get_card_value(card));
    }

    cards.sort();
    let mut rank: usize = 0;
    
    // Five of a kind
    if cards[0] == cards[4] { rank = 7 as usize; }  
    // Four of a kind
    else if cards[0] == cards[3] || cards[1] == cards[4] { rank = 6 as usize; }
    // Full house
    else if (cards[0] == cards[2] && cards[3] == cards[4])  || (cards[0] == cards[1] && cards[2] == cards[4]) { rank = 5 as usize; }
    // Three of a kind
    else if cards [0] == cards[2] || cards[1] == cards[3] || cards[2] == cards[4] { rank = 4 as usize; }
    // two pair
    else if pair(cards.clone()) == 2 { rank = 3 as usize; } 
    // one pair
    else if pair(cards.clone()) == 1 { rank = 2 as usize; }
    // high card
    else { rank = 1 as usize; }
    return rank;
}

fn pair(cards: Vec<i32>) -> usize {
    let mut result = 0;
    for i in 0..cards.len() - 1 {
        if cards[i] == cards[i + 1] { result += 1; }
    }
    return result;
}

// return -1 if hand1 < hand2 0 if = and 1 if hand1 > hand2
fn compare_hands(hand1: &str, hand2: &str) -> i32 {
    let hand1_vec: Vec<char> = hand1.chars().collect();
    let hand2_vec: Vec<char> = hand2.chars().collect();
    for i in 0..hand1.len() {
        let card1 = get_card_value(hand1_vec[i]);
        let card2 = get_card_value(hand2_vec[i]);
        if card1 > card2 { return 1; }
        else if card1 < card2 { return -1; }
    }
    return 0;
}

fn get_card_value(card: char) -> i32 {
    let mut value = 0;
    match card {
        '2' => value = 2,
        '3' => value = 3,
        '4' => value = 4,
        '5' => value = 5,
        '6' => value = 6,
        '7' => value = 7,
        '8' => value = 8,
        '9' => value = 9,
        'T' => value = 10,
        'J' => value = 11,
        'Q' => value = 12,
        'K' => value = 13,
        'A' => value = 14,
        _ => panic!(),
    }
    return value;
}
