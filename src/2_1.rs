use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

struct BallAmount {
    blue: u32,
    green: u32,
    red: u32,
}

fn main() -> std::io::Result<()> {
    let path = Path::new("instances/2.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let constraint = BallAmount {
        blue: 14,
        green: 13,
        red: 12,
    };

    let mut sum = 0;
    for line in reader.lines() {
        let unwrapped_line = line.unwrap();
        let mut game_split = unwrapped_line.split(": ");
        let game_id: u32 = game_split
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let game_part_str = game_split.next().unwrap();
        let mut summand = game_id;
        for reveal_str in game_part_str.split("; ") {
            for revealed_ball_raw in reveal_str.split(", ") {
                let mut revealed_ball = revealed_ball_raw.split(" ");
                let amount_str = revealed_ball.next().unwrap();
                let amount: u32 = amount_str.parse().unwrap();
                let color = revealed_ball.next().unwrap();

                match color {
                    "blue" => {
                        if amount > constraint.blue {
                            summand = 0;
                        }
                    }
                    "green" => {
                        if amount > constraint.green {
                            summand = 0;
                        }
                    }
                    "red" => {
                        if amount > constraint.red {
                            summand = 0;
                        }
                    }
                    _ => print!("color {color}"),
                }
            }
        }
        sum += summand;
    }
    println!("{:?}", sum);

    Ok(())
}
