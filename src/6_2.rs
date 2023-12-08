use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::vec;

fn calc_button_pressing_range_length(time_dist_pair: (&usize, &usize)) -> usize {
    let time = *time_dist_pair.0 as f64;
    let distance = *time_dist_pair.1 as f64;
    let root_value = (time.powf(2.0) - 4.0 * distance).sqrt();

    let mut start_range_float = (time - root_value) / 2.0;
    let mut end_range_float = (time + root_value) / 2.0;

    if start_range_float.ceil() == start_range_float {
        start_range_float += 1.0;
    }
    if end_range_float.floor() == end_range_float {
        end_range_float -= 1.0;
    }
    let start_range = start_range_float.ceil() as usize;
    let end_range = end_range_float.floor() as usize;

    end_range - start_range + 1
}

fn main() -> std::io::Result<()> {
    let path = Path::new("instances/6.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut parsed_numers: Vec<_> = vec![];
    for line in reader.lines() {
        let line_numbers: Vec<_> = vec![line
            .unwrap()
            .split(" ")
            .map(|p| p.trim())
            .filter(|p| !p.is_empty() && p.chars().next().unwrap().is_ascii_digit())
            .collect::<String>()
            .parse::<usize>()
            .unwrap()];

        parsed_numers.push(line_numbers);
    }

    let mut parsed_numbers_iter = parsed_numers.iter();
    let times = parsed_numbers_iter.next().unwrap();
    let distances = parsed_numbers_iter.next().unwrap();
    let time_dist_pairs: Vec<_> = times.iter().zip(distances.iter()).collect();

    let faster_time_points_of_all_races: Vec<_> = time_dist_pairs
        .iter()
        .map(|p| calc_button_pressing_range_length(p.clone()))
        .collect();

    println!(
        "{:?}",
        faster_time_points_of_all_races
            .iter()
            .fold(1, |acc, v| v * acc)
    );
    Ok(())
}
