use std::fs::File;

mod png;

fn main() {
    let mut file = File::open("test.png").expect("Failed to open file");
    let _png = png::parse(&mut file).expect("Could not parse PNG");
}
