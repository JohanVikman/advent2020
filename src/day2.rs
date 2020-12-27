use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::convert::TryInto;

pub fn run() {
    let mut validpasswords: u32 = 0;
    let mut f = File::open("priv/day2input.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong ");

    let re = Regex::new(r"^(\d+)[\-](\d+) ([a-z]?): ([a-z]+)$").unwrap();
    for line in contents.lines(){
        for cap in re.captures_iter(line) {
            let min = &cap[1].parse::<u32>().expect("Integer");
            let max = &cap[2].parse::<u32>().expect("Integer");
            let letter = &cap[3];
            let v: Vec<&str> = cap[4].matches(letter).collect();
            let nletters: u32 = v.len().try_into().unwrap();
            if (nletters >= *min) && (nletters <= *max) { 
                validpasswords += 1;
            }
        }
    }
    println!("Valid passwords {}", validpasswords);
}
