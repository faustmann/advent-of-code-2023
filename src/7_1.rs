use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::vec;

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
}
impl Hand {
    fn rank_cards(&self) -> usize {
        let mut sorted_cards: Vec<char> = self.cards.clone();
        sorted_cards.sort();

        let grouped_cards: Vec<_> = sorted_cards
            .iter()
            .into_group_map_by(|&x| x)
            .into_iter()
            .map(|(k, v)| (k, v.len() as u32))
            .collect();

        if grouped_cards
            .iter()
            .any(|card_type_occurences| card_type_occurences.1 == 5)
        {
            return 7;
        }
        if grouped_cards
            .iter()
            .any(|card_type_occurences| card_type_occurences.1 == 4)
        {
            return 6;
        }
        if grouped_cards
            .iter()
            .any(|card_type_occurences| card_type_occurences.1 == 3)
            && grouped_cards
                .iter()
                .any(|card_type_occurences| card_type_occurences.1 == 2)
        {
            return 5;
        }
        if grouped_cards
            .iter()
            .any(|card_type_occurences| card_type_occurences.1 == 3)
        {
            return 4;
        }
        if grouped_cards
            .iter()
            .filter(|card_type_occurences| card_type_occurences.1 == 2)
            .collect::<Vec<_>>()
            .len()
            == 2
        {
            return 3;
        }
        if grouped_cards
            .iter()
            .filter(|card_type_occurences| card_type_occurences.1 == 2)
            .collect::<Vec<_>>()
            .len()
            == 1
        {
            return 2;
        }
        return 1;
    }
}

fn order_hands(hand_a: &Hand, hand_b: &Hand) -> Ordering {
    let rank_a = hand_a.rank_cards();
    let rank_b = hand_b.rank_cards();

    if rank_a != rank_b {
        return rank_a.cmp(&rank_b);
    }

    let card_type_rank = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ]);

    let ranked_cards_a = hand_a
        .cards
        .iter()
        .map(|c| card_type_rank.get(c).unwrap())
        .collect::<Vec<_>>();
    let ranked_cards_b = hand_b
        .cards
        .iter()
        .map(|c| card_type_rank.get(c).unwrap())
        .collect::<Vec<_>>();

    for idx in 0..ranked_cards_a.len() {
        if ranked_cards_a[idx] != ranked_cards_b[idx] {
            return ranked_cards_a[idx].cmp(ranked_cards_b[idx]);
        }
    }
    return Ordering::Equal;
}

fn main() -> std::io::Result<()> {
    let path = Path::new("instances/7.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut hands: Vec<Hand> = vec![];
    for line in reader.lines() {
        let raw_line = line.unwrap();
        let mut hand_infos = raw_line.split(" ");

        let hand = Hand {
            cards: hand_infos.next().unwrap().chars().collect(),
            bid: hand_infos.next().unwrap().parse().unwrap(),
        };
        hands.push(hand);
    }

    hands.sort_by(|a, b| order_hands(&a, &b));
    println!(
        "{:?}",
        hands
            .iter()
            .enumerate()
            .map(|(rank, c)| { (rank + 1) * c.bid })
            .sum::<usize>()
    );
    Ok(())
}
