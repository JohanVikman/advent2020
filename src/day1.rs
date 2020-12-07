use std::fs::File;
use std::io::Read;

pub fn run() {
    let mut f = File::open("priv/day1input.txt").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong ");
    let lines = contents.lines();
    'outer: for line in lines{
        let line = line.trim().parse::<u32>().expect("Integer");
        let comparelines = contents.lines();
        for compareline in comparelines {
            let compareline = compareline.trim().parse::<u32>().expect("Integer");
            if line + compareline == 2020 {
                println!("{} and {} gives {}", line, compareline,line * compareline);
            }
            let comparelines2 = contents.lines();
            for compareline2 in comparelines2 {
                let compareline2 = compareline2.trim().parse::<u32>().expect("Integer");
                if line + compareline + compareline2 == 2020 {
                    println!("{} and {} and {} gives {}", line, compareline, compareline2, line * compareline * compareline2);
                    break 'outer;
                }
            }
        }
    }
}
