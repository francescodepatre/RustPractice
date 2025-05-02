use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("foo.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
