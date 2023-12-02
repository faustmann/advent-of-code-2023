use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn map_strnum2num(strnum: &str) -> u32 {
    match strnum {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => strnum.parse().unwrap(),
    }
}

fn main() -> std::io::Result<()> {
    let path = Path::new("instances/1.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let strnum_regex = "one|two|three|four|five|six|seven|eight|nine";

    let re = Regex::new(&format!(r"{strnum_regex}|\d")).unwrap();

    let strnum_regex_rev = &strnum_regex.chars().rev().collect::<String>();
    let re_rev = Regex::new(&format!(r"{strnum_regex_rev}|\d")).unwrap();

    let mut sum = 0;
    for line in reader.lines() {
        let line_unwrap: &str = &line.unwrap();
        let fst_num_str: &str = re.find(line_unwrap).unwrap().as_str();
        let fst_num = map_strnum2num(fst_num_str);

        let line_reverse = &line_unwrap.chars().rev().collect::<String>();
        let lst_num_str_rev: &str = re_rev.find(line_reverse).unwrap().as_str();
        let lst_num_str = &lst_num_str_rev.chars().rev().collect::<String>();
        let lst_num = map_strnum2num(lst_num_str);

        let num = 10 * fst_num + lst_num;

        sum += num;
    }
    println!("{:?}", sum);

    Ok(())
}
