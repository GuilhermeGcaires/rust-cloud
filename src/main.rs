use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./tratado.csv").unwrap();
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


    println!("{:?}", total);
    println!("{:?}", table);
}
