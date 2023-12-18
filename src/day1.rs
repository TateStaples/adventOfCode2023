use std::collections::HashMap;
use std::fs;
use std::str::Chars;
use std::process;


pub(crate) fn main() {
    //part1();
    part2();
}
fn part1() {
    let filepath = "src/data/day1.txt";
    let file = fs::read_to_string(filepath).expect("Couldn't read day1 file");
    let lines = file.lines();
    let reduced: Vec<Vec<char>> = lines.map(|l| l.chars().filter(|c| c.is_digit(10)).collect()).collect();

    let numbers: Vec<i32> = reduced.iter()
        .filter_map(|s| {
            if let (Some(first), Some(last)) = (s.first(), s.last()) {
                let first_digit = first.to_digit(10)? as i32;
                let last_digit = last.to_digit(10)? as i32;
                Some(first_digit * 10 + last_digit)
            } else {
                None
            }
        })
        .collect();

    let sum: i32 = numbers.iter().sum();
    let size = numbers.len();
    println!("{sum}");
}

struct Trie {
    children: HashMap<char, Trie>,
    value: Option<i32>,
}

fn part2() {
    let filepath = "src/data/day1.txt";
    let file = fs::read_to_string(filepath).expect("Couldn't read day1 file");
    let lines = file.lines();
    // this seems like an application of a Trie
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut trie = Trie {
        children: HashMap::new(),
        value: None,
    };
    for (i, number) in numbers.iter().enumerate() {
        let mut activeTrie = &mut trie;
        for c in number.chars() {
            activeTrie = activeTrie.children.entry(c).or_insert(Trie {
                children: HashMap::new(),
                value: None,
            });
        }
        activeTrie.value = Some(i as i32 + 1);
    }
    for i in 0..9{
        trie.children.insert(digits[i], Trie {
            children: HashMap::new(),
            value: Some(i as i32 + 1)
        });
    }

    // now parse
    let mut sum = 0;
    for s in lines {
        let mut values: Vec<i32> = Vec::new();
        let mut active_tries = vec![&trie];
        for c in s.chars() {
            let mut next_tries = vec![&trie];
            for activeTrie in &active_tries {
                if let Some(nextTrie) = activeTrie.children.get(&c) {
                    next_tries.push(nextTrie);
                    if let Some(val) = nextTrie.value {
                        values.push(val);
                    }
                }
            }
            active_tries = next_tries;
        }
        let val = 10 * values.first().unwrap() + values.last().unwrap();
        sum += val;
        println!("{}, {}",s, val);
    }
    println!("{sum}");

}