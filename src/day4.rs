use std::fs::File;
use std::io::Read;
//use std::str::Chars;
//use regex::Regex;
//use std::convert::TryInto;

#[derive(PartialEq, Clone, Copy, Debug)]
enum EyeColor {
    UNDEFINED,
    BROWN,
    AMBER,
    BLUE,
    GREY,
    LAZER,
    OTHER
}

#[derive(Debug)]
pub struct Passport {
    byr: u32, //Birth year
    iyr: u32, //Issue year
    eyr: u32, //Expiration year
    hgt: String, //Height in cm
    pid: String, //u32, //Passport ID
    cid: u32, //Country ID
    hcl: String, //Hair color
    ecl: EyeColor, //Eye color
    valid: u32,
}


impl Passport {
    pub fn new(contents: &str) -> Passport{
        let mut byr: u32 = 0;
        let mut iyr: u32 = 0;
        let mut eyr: u32 = 0;
        let mut hgt = "0cm".to_string();
        let mut hcl = "".to_string();
        let mut ecl: EyeColor = EyeColor::UNDEFINED;
        let mut pid = "".to_string(); //: u32 = 0;
        let mut cid: u32 = 0;
        let mut valid = 0;

        for item in contents.split_whitespace().collect::<Vec<_>>() {
            // "Key:value"
            let mut kv = item.split(':').collect::<Vec<&str>>();
            let mut key = kv[0];
            let mut value = kv[1];
            match key {
                "byr" => {
                    byr = value.trim().parse::<u32>().unwrap();
                    valid += 1
                        },
                "iyr" => {
                    iyr = value.trim().parse::<u32>().unwrap();
                    valid += 1
                    },
                "eyr" => {
                    eyr = value.trim().parse::<u32>().unwrap();
                    valid += 1
                        },
                "hgt" => {
                    hgt = value.to_string();
                    valid += 1
                        },   
                "pid" => {
                    pid = value.to_string();
                    valid += 1
                        }, //value.trim().parse::<u32>().unwrap(),
                "cid" => {
                    cid =value.trim().parse::<u32>().unwrap();
                    valid += 1
                        },
                "hcl" => {
                    hcl =value.to_string();
                    valid += 1
                        },
                "ecl" => {
                    ecl = match value {
                            "brn" => EyeColor::BROWN,
                            "blu" => EyeColor::BLUE,
                            "gry" => EyeColor::GREY,
                            "lzr" => EyeColor::LAZER,
                            "amb" => EyeColor::AMBER,
                            _ => { println!("Unknown eyecolor {}", value);
                                EyeColor::OTHER
                                },
                            };
                    valid += 1
                },
                _ => println!("Unexpected key {}", key),
            } 
        }

        Passport{
            byr: byr,
            iyr: iyr,
            eyr: eyr,
            hgt: hgt,
            hcl: hcl,
            ecl: ecl,
            pid: pid,
            cid: cid,
            valid: valid,
        }
    }

    pub fn validate(&mut self) -> bool {
        println!("{:?}", self);
        let mut valid = false;
        /* let vlen: usize= [self.byr, self.iyr, self.eyr].
            iter().filter(|&x| *x != 0).collect::<Vec<&u32>>().len();
        let all_numeric_set = vlen == 3; //not looking at cid
        if !all_numeric_set {
            println!("XXXXX All numeric NOTset {} {}", all_numeric_set, vlen);
        }
        valid = all_numeric_set && (self.hgt != "0cm".to_string()) &&
            (self.ecl != EyeColor::UNDEFINED) && (self.pid != "".to_string()); */
        valid = self.valid == 8 || (self.valid == 7 && self.cid == 0);
        println!("VAlid {} ", valid);
        return valid;
    }
}

pub fn run() {
    let mut f = File::open("priv/day4input.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong ");

    let mut passports: Vec<Passport> = Vec::new();
    let mut npassports: u32 = 0;
    let mut nvalid: u32 = 0;

    for passportstr in contents.split("\n\n") {
        //println!("passportstr {} ", passportstr);
        let mut passport = Passport::new(passportstr);
        npassports += 1;
        if passport.validate() {
            nvalid += 1;
        }
        passports.push(passport);
    }

    println!("Part1: {} passports and {} valid", npassports, nvalid);

//Part 2:
    /* let part2trees = map.count_trees(1,1) * map.count_trees(3,1) *
        map.count_trees(5,1) * map.count_trees(7, 1) *
        map.count_trees(1, 2);
    println!("Part2: Valid passwords {}", part2trees); */
}
