mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    // Check what day you want to run
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if arg == "day1" {
            day1::run()
        } if arg == "day2" {
            day2::run()
        } if arg == "day3" {
            day3::run()
        } if arg == "day4" {
            day4::run()
        } else {
            println!("No day selected!");
        }
    }
}
