use std::fs::File;
use std::io::Read;
//use std::str::Chars;
//use regex::Regex;
use std::convert::TryInto;

#[derive(PartialEq, Clone, Copy)]
enum MapSymbol {
    Tree,
    Empty,
}

pub struct Map {
    map: Vec<Vec<MapSymbol>>,
    linelength: u32,
}

impl Map {
    pub fn new(contents: &str) -> Map{
        let mut map: Vec<Vec<MapSymbol>> = Vec::new();
        let mut linelength = 0;
        for line in contents.lines() {
            if linelength == 0 {
                linelength = line.len().try_into().unwrap();
            }
            map.push(line.chars().map( |c|
                if c == '#' {MapSymbol::Tree}
                else {MapSymbol::Empty}).collect::<Vec<MapSymbol>>());
            }
        println!("Pushed {} onto map", map.len());
        Map{
            map: map,
            linelength: linelength,
        }
    }

    pub fn count_trees(&mut self, xstep: u32, ystep: u32) -> u32 {
        let mut x = 0;
        let mut y = 0;
        let mut ntrees = 0;
        for row in &self.map {
            if y == 0 {
                if row[(x % self.linelength) as usize] == MapSymbol::Tree
                    {
                        ntrees += 1;
                    }
            y = ystep;
            x += xstep;
            }
            y -= 1;
        }
        return ntrees;
    }
}

pub fn run() {
    let mut f = File::open("priv/day3input.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong ");

    let mut map = Map::new(&contents);

    println!("Part1: N trees are {}", map.count_trees(3, 1));

//Part 2:
    let part2trees = map.count_trees(1,1) * map.count_trees(3,1) *
        map.count_trees(5,1) * map.count_trees(7, 1) *
        map.count_trees(1, 2);
    println!("Part2: Valid passwords {}", part2trees);
}
