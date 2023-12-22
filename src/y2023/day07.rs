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

impl Solver<usize, usize> for Solution {
    fn solve(&self, input: &str) -> (usize, usize) {
        (
            find_total_winnings(
                input,
                &classify_hand,
                vec![
                    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
                ],
            ),
            find_total_winnings(
                input,
                &classify_hand_with_jokers,
                vec![
                    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
                ],
            ),
        )
    }
}

fn find_total_winnings(
    input: &str,
    classify_fun: &dyn Fn(Vec<char>) -> HandType,
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
                classify_fun(hand.chars().collect_vec()),
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

fn classify_hand(hand: Vec<char>) -> HandType {
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in hand.into_iter() {
        *map.entry(c).or_insert(0) += 1;
    }

    let vec: Vec<i32> = map.values().sorted().rev().copied().collect();

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

fn classify_hand_with_jokers(hand: Vec<char>) -> HandType {
    hand.into_iter()
        .map(|c| match c {
            'J' => vec![
                'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
            ],
            _ => vec![c],
        })
        .multi_cartesian_product()
        .fold(HandType::HighCard, |best, hand| {
            classify_hand(hand).max(best)
        })
}
