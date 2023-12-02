use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = Path::new("instances/1.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let num_line: Vec<char> = line
            .unwrap()
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect();

        let fst = num_line.iter().next().unwrap();
        let lst = num_line.iter().last().unwrap();

        let num_string: String = vec![fst, lst].into_iter().collect();
        let num: u32 = num_string.parse().unwrap();
        sum += num;
    }
    println!("{:?}", sum);

    Ok(())
}
