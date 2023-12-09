use std::collections::{HashMap, HashSet};

fn process(contents: &str) -> u32 {
    let mut instances: HashMap<u32, u32> = HashMap::new();
    let total_cards = contents.lines().count() as u32;
    for line in contents.lines() {
        if let Some((card_part, rest)) = line.split_once(':') {
            let card = card_part
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            if let Some((winning_str, hand_str)) = rest.split_once('|') {
                let winning = winning_str
                    .split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<HashSet<u32>>();
                let hand = hand_str
                    .split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<HashSet<u32>>();
                let count = winning.intersection(&hand).count() as u32;

                for i in (card + 1)..(card + count + 1) {
                    let mut copies = 0;
                    copies += if instances.contains_key(&i) {
                        instances.get(&i).unwrap()
                    } else {
                        &0
                    };
                    copies += 1;
                    copies += if instances.contains_key(&card) {
                        instances.get(&card).unwrap()
                    } else {
                        &0
                    };
                    instances.insert(i, copies);
                }
            }
        }
    }
    instances.values().sum::<u32>() + total_cards
}

pub fn solve() -> (i32, i32) {
    let s = String::from_utf8_lossy(include_bytes!("../inputs/input04.txt"));
    process(&s);
    (0, 0)
}
