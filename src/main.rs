mod day1;

fn main() {
    // Check what day you want to run
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if arg == "day1" {
            day1::run()
        } else {
            println!("No day selected!");
        }
    }
}
