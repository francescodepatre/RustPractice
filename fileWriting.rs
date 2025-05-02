use std::fs::File;
use std::io::BufWriter;

fn main() {
    let data = "Some data!";
    let f = File::create("foo.txt").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(data.as_bytes()).expect("Unable to write data");
}
