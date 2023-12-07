use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::Card::{Eight, Five, Four, Nine, Seven, Six, Three, Two};
use crate::HandType::HighCard;

#[derive(Debug,Hash,Eq,PartialEq,Ord,PartialOrd)]
enum Card {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, T, J, Q, K, A
}

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd)]
enum HandType {
    HighCard, OnePair, TwoPair, Three, FullHouse, Four, Five
}

fn main() {
    let mut hands = BufReader::new(File::open("day7.txt").expect("what could be happen?"))
        .lines()
        .map(|x| x.expect("do it"))
        .map(|x| {
            let mut splits = x.split(' ');
            let hand = splits.next().expect("no bytes?").bytes().map(|b| {
                match b {
                    b'2' => Two, b'3' => Three, b'4' => Four, b'5' => Five, b'6' => Six, b'7' => Seven,
                    b'8' => Eight, b'9' => Nine, b'T' => Card::T, b'J' => Card::J, b'Q' => Card::Q, b'K' => Card::K, b'A' => Card::A,
                    _ => panic!("unexpected byte {}", b)
                }
            }).collect::<Vec<Card>>();
            let bid = splits.next().expect("try again").parse::<u32>().expect("hmm...");
            (hand, bid)
        })
        .map(|(hand, bid)| {
            let hand_type = hand.iter().fold(HashMap::new(), |mut agg, x| {
                if let Some(val) = agg.get_mut(x) {
                    *val += 1;
                } else {
                    agg.insert(x, 1);
                }
                agg
            }).into_iter().fold(HandType::HighCard, |agg, (k, v)| {
                match (v, &agg) {
                    (2, HandType::HighCard) => HandType::OnePair,
                    (2, HandType::OnePair) => HandType::TwoPair,
                    (2, HandType::Three) => HandType::FullHouse,
                    (3, HandType::HighCard) => HandType::Three,
                    (3, HandType::OnePair) => HandType::FullHouse,
                    (4, _) => HandType::Four,
                    (5, _) => HandType::Five,
                    _ => agg
                }
            });
            (hand_type, hand, bid)
        }).collect::<Vec<(HandType, Vec<Card>, u32)>>();

    hands.sort_by(|(lhs_hand_type, lhs_hand, _), (rhs_hand_type, rhs_hand, _)| {
        let hand_type_cmp = lhs_hand_type.cmp(rhs_hand_type);
        match hand_type_cmp {
            Ordering::Equal => {
                lhs_hand.cmp(rhs_hand)
            }
            _ => hand_type_cmp
        }
    });

    let res = hands.into_iter().enumerate().fold(0, |acc, (i, (_, _, bid))| {
        acc + (i + 1) * bid as usize
    });

    println!("{}", res);
}
