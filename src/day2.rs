use std::fs;

pub(crate) fn main() {
    part1();
    part2();
}

fn part1() {
    let filepath = "src/data/day2.txt";
    let file = fs::read_to_string(filepath).expect("Couldn't read day2 file");
    let lines = file.lines();

    let mut acc = 0;
    for (id, line) in lines.enumerate() {
        let information = line.split(':').last().expect("No colon in message");
        let games = information.split(';');
        let mut valid_bag = true;
        for game in games {
            let balls = game.split(',');
            for ball in balls {
                let parts: Vec<&str> = ball.split_whitespace().collect();
                let size: i32 = parts.first().unwrap().parse().unwrap();
                let color = parts.last().unwrap();
                let target = match *color {
                    "green" => 13,
                    "red" => 12,
                    "blue" => 14,
                    _ => -1
                };
                if size > target {
                    valid_bag = false;
                    break;
                }
            }
            if !valid_bag {break;}
        }
        if valid_bag {
            acc += id as i32 + 1;
        }
    }
    println!("{acc}");
}

fn part2() {
    let filepath = "src/data/day2.txt";
    let file = fs::read_to_string(filepath).expect("Couldn't read day2 file");
    let lines = file.lines();

    let mut acc = 0;
    for (id, line) in lines.enumerate() {
        let information = line.split(':').last().expect("No colon in message");
        let games = information.split(';');
        let mut valid_bag = true;
        let mut greenMax = 0;
        let mut redMax = 0;
        let mut blueMax = 0;
        for game in games {
            let balls = game.split(',');
            for ball in balls {
                let parts: Vec<&str> = ball.split_whitespace().collect();
                let size: i32 = parts.first().unwrap().parse().unwrap();
                let color = parts.last().unwrap();
                match *color {
                    "green" => greenMax = greenMax.max(size),
                    "red" => redMax = redMax.max(size),
                    "blue" => blueMax = blueMax.max(size),
                    _ => ()
                };
            }
        }
        acc += greenMax * blueMax * redMax;
    }
    println!("{acc}");
}