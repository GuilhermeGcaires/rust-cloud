use std::collections::HashMap;
use std::fs::{File, self};
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../2019-Oct.csv").unwrap();
    let reader = BufReader::new(file);
    let mut table: HashMap<String, i32> = HashMap::new();
    let mut total = 0;

    for lin in reader.lines() {
        let line = lin.unwrap();
        let word = line.split(",").nth(4).unwrap();

        let mut count = table.entry(word.to_string()).or_insert(0);
        *count += 1;
        total += 1;
    }

    let mut write_file = File::create("most_occuring.txt");

    let mut result: Vec<_> = table.iter().collect();
    result.sort();

    fs::write("most_occuring.txt", result);
    println!("{:?}", total);
    println!("{:?}", result);
}
