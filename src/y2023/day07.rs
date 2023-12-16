use crate::Solver;
use hashbrown::HashMap;
use itertools::Itertools;
pub struct Solution;

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

// #[derive(Debug, Clone)]
// struct Hand {
//     // hand: String,
//     hand_type: HandType,
//     sort_key: (usize, usize, usize, usize, usize),
//     bid: usize,
// }

impl Solver for Solution {
    fn solve(&self, input: &String) -> (String, String) {
        let p1 = find_total_winnings(
            input,
            &classify_hand,
            vec![
                'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
            ],
        );

        (p1.to_string(), String::new())
    }
}

fn find_total_winnings(
    input: &str,
    classify_fun: &dyn Fn(&str) -> HandType,
    card_order: Vec<char>,
) -> usize {
    let sort_keys = card_order
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (*c, i))
        .collect::<HashMap<char, usize>>();

    let mut hands = input
        .trim()
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| {
            (
                classify_fun(hand),
                sort_key(hand, &sort_keys),
                bid.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    hands.sort();

    (1..=hands.len())
        .zip(hands)
        .fold(0, |acc, (rank, (_, _, bid))| rank * bid + acc)
}

fn sort_key(hand: &str, sort_keys: &HashMap<char, usize>) -> (usize, usize, usize, usize, usize) {
    hand.chars()
        .map(|card| *sort_keys.get(&card).unwrap())
        .collect_tuple()
        .unwrap()
}

fn classify_hand(hand: &str) -> HandType {
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in hand.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    let vec = map
        .values()
        .sorted()
        .rev()
        .map(|n| *n)
        .collect::<Vec<i32>>();

    if let Some((5,)) = vec.iter().next_tuple() {
        HandType::FiveOfAKind
    } else if let Some((4, 1)) = vec.iter().next_tuple() {
        HandType::FourOfAKind
    } else if let Some((3, 2)) = vec.iter().next_tuple() {
        HandType::FullHouse
    } else if let Some((3, 1, 1)) = vec.iter().next_tuple() {
        HandType::ThreeOfAKind
    } else if let Some((2, 1, 1, 1)) = vec.iter().next_tuple() {
        HandType::OnePair
    } else if let Some((2, 2, 1)) = vec.iter().next_tuple() {
        HandType::TwoPair
    } else if let Some((1, 1, 1, 1, 1)) = vec.iter().next_tuple() {
        HandType::HighCard
    } else {
        unreachable!()
    }
}
