use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn check_neighbourhood(instance: &Vec<Vec<char>>, y: usize, x: usize) -> Option<(usize, usize)> {
    for delta_x in -1isize..2 {
        for delta_y in -1isize..2 {
            let check_y = y as isize + delta_y;
            let check_x = x as isize + delta_x;

            if (delta_x == 0 && delta_y == 0)
                || check_y < 0
                || check_x < 0
                || check_y as usize >= instance.len()
                || check_x as usize >= instance[0].len()
            {
                continue;
            }

            let check_sign: char = instance[check_y as usize][check_x as usize];

            if !check_sign.is_digit(10) && check_sign != '.' {
                return Some((check_y as usize, check_x as usize));
            }
        }
    }
    return None;
}

fn main() -> std::io::Result<()> {
    let path = Path::new("instances/3.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let instance: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let mut cur_part_number: Vec<char> = vec![];
    let mut symbol_coordinate_found: Option<(usize, usize)> = None;
    let mut found_numbers: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for y in 0..instance.len() {
        for x in 0..instance[0].len() {
            let sign = instance[y][x];
            if sign.is_ascii_digit() {
                cur_part_number.push(sign);
                let found_part = check_neighbourhood(&instance, y, x);
                if symbol_coordinate_found.is_none() && found_part.is_some() {
                    symbol_coordinate_found = found_part;
                }
            } else if !cur_part_number.is_empty() && symbol_coordinate_found.is_some() {
                let comp_coord = symbol_coordinate_found.unwrap();
                let number: u32 = cur_part_number.iter().collect::<String>().parse().unwrap();
                if found_numbers.contains_key(&comp_coord) {
                    let existing_number_list: &mut Vec<u32> =
                        found_numbers.get_mut(&comp_coord).unwrap();
                    existing_number_list.push(number);
                } else {
                    let number_list = vec![number];
                    found_numbers.insert(comp_coord, number_list);
                }

                cur_part_number.clear();
                symbol_coordinate_found = None;
            } else {
                cur_part_number.clear();
                symbol_coordinate_found = None;
            }
        }
    }
    let res = found_numbers
        .values()
        .cloned()
        .filter(|values| values.len() == 2)
        .map(|values| values[0] * values[1])
        .sum::<u32>();
    println!("{:?}", res);
    Ok(())
}
