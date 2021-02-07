use std::fs::File;
use std::io::Read;
use regex::Regex;
//use std::str::Chars;
//use std::convert::TryInto;

#[derive(PartialEq, Clone, Copy, Debug)]
enum EyeColor {
    UNDEFINED,
    AMBER,
    BROWN,
    BLUE,
    GREY,
    GREEN,
    HAZEL,
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
    valid: u32, //Valid passport needs seven valid fields
}


impl Passport {
    pub fn new(contents: &str) -> Passport{
        let mut byr: u32 = 0;
        let mut iyr: u32 = 0;
        let mut eyr: u32 = 0;
        let mut hgt = "0cm".to_string();
        let hgtCmRegExp = Regex::new(r"^(\d{3})(cm)$").unwrap();
        let hgtInRegExp = Regex::new(r"^(\d{2})(in)$").unwrap();
        let hclRegExp = Regex::new(r"#(?:[a-z]|[0-9]){6}$").unwrap();
        let mut hcl = "".to_string();
        let mut ecl: EyeColor = EyeColor::UNDEFINED;
        let mut pid = "".to_string(); //: u32 = 0;
        let pidRegExp = Regex::new(r"^\d{9}$").unwrap();
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
                    if (1920 < byr || byr == 1920) {
                        if (byr < 2002 || byr == 2002) {
                            println!("byr {} valid!", byr);
                            valid += 1;
                            }
                        }
                    },
                "iyr" => {
                    iyr = value.trim().parse::<u32>().unwrap();
                    if (iyr > 2010 || iyr == 2010) {
                        if (iyr < 2020 || iyr == 2020) {
                            println!("iyr {} valid!", iyr);
                            valid += 1
                            }
                        }
                    },
                "eyr" => {
                    eyr = value.trim().parse::<u32>().unwrap();
                    if (eyr > 2020 || eyr == 2020) {
                        if (eyr < 2030 || eyr == 2030) {
                            println!("eyr {} valid!", eyr);
                            valid += 1
                            }
                        }
                    },
                "hgt" => {
                    hgt = value.to_string();
                    if hgtCmRegExp.is_match(&hgt) {
                        let cap = hgtCmRegExp.captures(&hgt).unwrap();
                        let cms = &cap[1].trim().parse::<u32>().unwrap();
                        if cms > &149 && cms < &194 {
                            println!("hgt {} valid!", hgt);
                            valid += 1;
                            }
                    } else {
                        if hgtInRegExp.is_match(&hgt) {
                            let cap = hgtInRegExp.captures(&hgt).unwrap();
                            let ins = &cap[1].trim().parse::<u32>().unwrap();
                            if ins > &58 && ins < &77 {
                                println!("hgt {} valid!", hgt);
                                valid += 1;
                                }
                            }
                        }
                    },
                "pid" => {
                    pid = value.to_string();
                    if pidRegExp.is_match(&pid) {
                        println!("pid {} valid!", pid);
                        valid += 1
                        }
                    },
                "cid" => {
                    cid =value.trim().parse::<u32>().unwrap();
                },
                "hcl" => {
                    // a # followed by exactly six characters 0-9 or a-f.
                    hcl =value.to_string();
                    if hclRegExp.is_match(&hcl) {
                        println!("hcl {} valid", hcl);
                        valid += 1;
                    } else {
                        println!("hcl {} INVALID", hcl);
                    }
                },
                "ecl" => {
                    //amb blu brn gry grn hzl oth.
                    ecl = match value {
                            "brn" => EyeColor::BROWN,
                            "blu" => EyeColor::BLUE,
                            "gry" => EyeColor::GREY,
                            "lzr" => EyeColor::LAZER,
                            "amb" => EyeColor::AMBER,
                            "oth" => EyeColor::OTHER,
                            "hzl" => EyeColor::HAZEL,
                            "grn" => EyeColor::GREEN,
                            "oth" => EyeColor::OTHER,
                            _ => { println!("Unknown eyecolor {}", value);
                                EyeColor::UNDEFINED
                                },
                            };
                    if ecl != EyeColor::UNDEFINED {
                        println!("ecl {} VALID", value);
                        valid +=1;
                    }
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
        //println!("{:?}", self);
        let mut valid = false;
        /* .
hgt (Height) - a number followed by either cm or in:
If cm, the number must be at least 150 and at most 193.
If in, the number must be at least 59 and at most 76.
hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
pid (Passport ID) - a nine-digit number, including leading zeroes.
         */
        valid = self.valid == 7;
        println!("VALID {} {}", valid, self.valid);
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
}
