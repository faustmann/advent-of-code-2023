use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
struct Game {
    _id: u32,
    winning_numbers: HashSet<u32>,
    selected_numbers: HashSet<u32>,
    amount: u32,
}
impl Game {
    fn get_right_numbers(&self) -> Vec<u32> {
        self.winning_numbers
            .intersection(&self.selected_numbers)
            .cloned()
            .collect()
    }
}

fn main() -> std::io::Result<()> {
    let path = Path::new("instances/4.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut games: Vec<Game> = vec![];
    for line in reader.lines() {
        let raw_line = line.unwrap();
        let mut raw_game_info = raw_line.split(":");
        let game_id: u32 = raw_game_info
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let mut all_numbers = raw_game_info.next().unwrap().split("|").map(|select_part| {
            HashSet::from_iter(
                select_part
                    .split(" ")
                    .filter(|s| !s.trim().is_empty())
                    .map(|sel_number| sel_number.trim().parse::<u32>().unwrap())
                    .into_iter(),
            )
        });

        let game = Game {
            _id: game_id,
            winning_numbers: all_numbers.next().unwrap(),
            selected_numbers: all_numbers.next().unwrap(),
            amount: 1,
        };
        games.push(game)
    }

    for game_idx in 0..games.len() {
        let wins = games[game_idx].get_right_numbers().len();
        for updater in game_idx + 1..game_idx + 1 + wins {
            games[updater].amount += games[game_idx].amount;
        }
    }

    let num_scratchcards: u32 = games.iter().map(|g| g.amount).sum();
    println!("{:?}", num_scratchcards);

    Ok(())
}
