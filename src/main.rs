use std::collections::HashMap;
use std::fs::{File, self};

fn main() {
    let mut table: HashMap<String, i32> = HashMap::new();
    let file = fs::read_to_string("./tratado.csv").unwrap();
    let mut total = 0;
    let mut i = 0;

    while i <= 100 {
        for lin in file.lines() {
            let line = lin;
            let word = line.split(",").nth(4).unwrap();

            let mut count = table.entry(word.to_string()).or_insert(0);
            *count += 1;
            total += 1;
        }
        i += 1;
    }

    println!("{:?}", total);
    println!("{:?}", table);
}
