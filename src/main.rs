use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::vec;

struct AlmanacEntry {
    _from: String,
    _to: String,
    mapper: Option<Vec<Vec<usize>>>,
}
impl AlmanacEntry {
    fn do_mapping(&mut self, v: usize) -> usize {
        for raw_map in self.mapper.as_mut().unwrap() {
            if raw_map[1] <= v && v < raw_map[1] + raw_map[2] {
                return v - raw_map[1] + raw_map[0];
            }
        }
        return v;
    }
}

fn main() -> std::io::Result<()> {
    let path = Path::new("instances/5.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut almanac_entries: Vec<AlmanacEntry> = vec![];
    let mut cur_almanac_entry: Option<AlmanacEntry> = None;
    let mut cur_partial_num_range_info: Vec<Vec<usize>> = vec![];
    let mut seeds: Vec<usize> = vec![];
    for line in reader.lines() {
        let raw_line = line?;

        if seeds.is_empty() {
            seeds = raw_line
                .split(": ")
                .last()
                .unwrap()
                .split(" ")
                .map(|num| num.parse::<usize>().unwrap())
                .collect();
            continue;
        } else if raw_line.is_empty() {
            if cur_almanac_entry.is_some() {
                let mut almanac_entry = cur_almanac_entry.unwrap();
                almanac_entry.mapper = Some(cur_partial_num_range_info.clone());
                almanac_entries.push(almanac_entry);
                cur_almanac_entry = None;
                cur_partial_num_range_info.clear();
            }
        } else {
            if raw_line.chars().next().unwrap().is_alphabetic() {
                let mut source_destination = raw_line.split(" ").next().unwrap().split("-to-");
                cur_almanac_entry = Some(AlmanacEntry {
                    _from: source_destination.next().unwrap().to_string(),
                    _to: source_destination.next().unwrap().to_string(),
                    mapper: None,
                });
            } else {
                cur_partial_num_range_info.push(
                    raw_line
                        .split(" ")
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect(),
                );
            }
        }
    }
    let mut almanac_entry = cur_almanac_entry.unwrap();
    almanac_entry.mapper = Some(cur_partial_num_range_info.clone());
    almanac_entries.push(almanac_entry);

    let mut locations: Vec<usize> = vec![];
    for seed in &seeds {
        let cur_location = almanac_entries
            .iter_mut()
            .fold(*seed, |thing_id: usize, alma_e: &mut AlmanacEntry| {
                alma_e.do_mapping(thing_id)
            });
        locations.push(cur_location);
    }

    println!("{:?}", locations.iter().min().unwrap());
    Ok(())
}
