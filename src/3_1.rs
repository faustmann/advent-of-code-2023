use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn check_neighbourhood(instance: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
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
                return true;
            }
        }
    }
    return false;
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
    let mut symbol_found = true;
    let mut found_numbers: Vec<u32> = vec![];

    for y in 0..instance.len() {
        for x in 0..instance[0].len() {
            let sign = instance[y][x];
            if sign.is_ascii_digit() {
                cur_part_number.push(sign);
                symbol_found |= check_neighbourhood(&instance, y, x);
            } else if !cur_part_number.is_empty() && symbol_found {
                let number: u32 = cur_part_number.iter().collect::<String>().parse().unwrap();
                found_numbers.push(number);

                cur_part_number.clear();
                symbol_found = false;
            } else {
                cur_part_number.clear();
                symbol_found = false;
            }
        }
    }
    println!("{:?}", found_numbers.iter().sum::<u32>());
    Ok(())
}
