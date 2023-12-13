use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use atoi::atoi;
use std::collections::HashMap;
use crate::HandType::*;

#[derive(PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    const VALUES: [Self; 7] =
        [HighCard, OnePair, TwoPair, ThreeOfAKind, FullHouse, FourOfAKind, FiveOfAKind];
    fn get_priority(&self) -> usize {
        return Self::VALUES.iter().position(|a| a == self).unwrap();
    }
}

const CARD_ORDER: [u8; 13] = [
    b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'J', b'Q', b'K', b'A'
];

struct Hand {
    cards:  Vec<u8>,
    bid: i64,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut card_cnt: HashMap<&u8, i64> = HashMap::new();
        self.cards.iter().for_each(|card| {
            let tmp = card_cnt.get(card);
            let mut cnt = 1;
            if tmp.is_some() {
                cnt = *tmp.unwrap() + 1;
            }
            card_cnt.insert(card, cnt);
        });
        let max_cnt = card_cnt.clone().into_iter().
            max_by(|x, y| x.1.cmp(&y.1)).unwrap();
        card_cnt.remove(max_cnt.0);
        if max_cnt.1 > 2i64 {
            return if max_cnt.1 == 3i64 {
                if card_cnt.iter().any(|(x, y)| *y == 2i64) {
                    FullHouse
                } else {
                    ThreeOfAKind
                }
            } else if max_cnt.1 == 4i64 {
                FourOfAKind
            } else {
                FiveOfAKind
            };
        }

        if max_cnt.1 == 2i64 {
            if card_cnt.iter().any(|x| *x.1 == 2) {
                return TwoPair;
            }
            return OnePair;
        }

        return HighCard;
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut hands: Vec<_> = input.split("\n").map(|hand_str| {
        let (cards_str, bid_str) = hand_str.split_once(' ').unwrap();
        return Hand {
            cards: cards_str.as_bytes().to_vec(),
            bid: atoi(bid_str.as_bytes()).unwrap(),
        };
    }).collect();

    hands.sort_by(hands_cmp);


    println!("{:?}", hands.iter().enumerate().map(|(i, hand)| (i + 1) as i64 * hand.bid)
        .reduce(|x, y| x + y).unwrap())
}


fn hands_cmp(x: &Hand, y: &Hand) -> Ordering {
    let tmp = x.hand_type().get_priority().cmp(&y.hand_type().get_priority());
    if tmp != Equal {
        return tmp;
    }

    for (card_x, card_y) in x.cards.iter()
        .map(|a| CARD_ORDER.iter().position(|b| b == a).unwrap())
        .zip(y.cards.iter().map(|a| CARD_ORDER.iter().position(|b| b == a).unwrap())) {
        if card_x.cmp(&card_y) != Equal {
            return card_x.cmp(&card_y);
        }
    }

    return Equal;
}