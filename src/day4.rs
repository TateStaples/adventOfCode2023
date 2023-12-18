use std::fs;

pub(crate) fn main() {
    part1();
    part2();
}

fn part1() {
    let filepath = "src/data/day3.txt";
    let file = fs::read_to_string(filepath).expect("Couldn't read day3 file");
    let lines = file.lines();
    for (i, line) in lines.enumerate() {
        let line_length = line.len();
        
    }
}

fn part2() {

}