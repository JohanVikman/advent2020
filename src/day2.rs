use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::convert::TryInto;

pub fn run() {
    let mut validpart1passwords: u32 = 0;
    let mut validpart2passwords: u32 = 0;
    let mut f = File::open("priv/day2input.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong ");

    let re = Regex::new(r"^(\d+)[\-](\d+) ([a-z]?): ([a-z]+)$").unwrap();
    for line in contents.lines(){
        for cap in re.captures_iter(line) {
            //part1
            let min = &cap[1].parse::<u32>().expect("Integer");
            let max = &cap[2].parse::<u32>().expect("Integer");
            let letter = &cap[3];
            let v: Vec<&str> = cap[4].matches(letter).collect();
            let nletters: u32 = v.len().try_into().unwrap();
            if (nletters >= *min) && (nletters <= *max) { 
                validpart1passwords += 1;
            }
            //part2
            let charletter = letter.chars().collect::<Vec<char>>()[0];
            let firstIndex = &cap[4].chars().nth(*min as usize - 1).unwrap();
            let secondIndex = &cap[4].chars().nth(*max as usize - 1).unwrap();
            let firstMatch = *firstIndex == charletter;
            let secondMatch = *secondIndex == charletter;
            if firstMatch && !secondMatch {
                validpart2passwords += 1;
            } else if !firstMatch && secondMatch {
                validpart2passwords += 1;
            }
        }
    }
    println!("Part1: Valid passwords {}", validpart1passwords);
    println!("Part2: Valid passwords {}", validpart2passwords);
}
