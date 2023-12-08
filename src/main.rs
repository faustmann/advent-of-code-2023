use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::str::Chars;
use std::vec;

#[derive(Debug)]
struct Hand<'a> {
    cards: Chars<'a>,
    bid: usize,
}

fn rank_cards(cards: Chars) {
    let mut sorted_cards: Vec<char> = cards.clone().collect();
    sorted_cards.sort();

    let grouped_cards: Vec<_> = sorted_cards
        .iter()
        .into_group_map_by(|&x| x)
        .into_iter()
        .map(|(k, v)| (k, v.len() as u32))
        .collect();
    println!("{:?}", grouped_cards)
}

fn main() -> std::io::Result<()> {
    let path = Path::new("instances/7_1_test.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut hands: Vec<Hand> = vec![];
    for line in reader.lines() {
        let raw_line = line.unwrap();
        let raw_line_split = raw_line.split(" ");
        let mut hand_infos = raw_line_split.into_iter();

        let hand = Hand {
            cards: hand_infos.next().unwrap().chars(),
            bid: hand_infos.next().unwrap().parse().unwrap(),
        };
        hands.push(hand);
    }

    println!("{:?}", hands);

    Ok(())
}
