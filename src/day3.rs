use std::fs::File;
use std::io::Read;
use std::str::Chars;
//use regex::Regex;
//use std::convert::TryInto;

#[derive(PartialEq)]
enum MapSymbol {
    Tree,
    Empty,
}

pub fn run() {
    let mut f = File::open("priv/day3input.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong ");

    let mut ntrees = 0;
    let mut map: Vec<Vec<MapSymbol>> = Vec::new();
    let mut linelength = 0;

    for line in contents.lines() {
        if linelength == 0 {
            linelength = line.len();
        }
        map.push(line.chars().map( |c|
            if c == '#' {MapSymbol::Tree}
            else {MapSymbol::Empty}).collect::<Vec<MapSymbol>>());
    }
    println!("Pushed {} onto map", map.len());
    let ystep = 1;
    let xstep = 3;
    let mut x = 0;
    let mut y = 0;
    for row in &map {
        if row[x % linelength] == MapSymbol::Tree 
            { 
                println!("Found a tree on {} {}", y, x);
                ntrees += 1;
            } 
        else {
                "Open".to_string();
            }
        x += xstep;
        y += ystep;
    }
    //println!("{}", map);
    // work with the board (of vectors)
    println!("Part1: N trees are {}", ntrees);
    println!("Part2: Valid passwords {}", ntrees);
}
