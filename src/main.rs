use std::env::args;

mod day_01;

fn main() {
    let args: Vec<String> = args().collect();
    let day: u32 = args[1].parse().expect("Usage:\n\tadvent2020 <day>");

    let answer = match day {
        1 => {
            format!("P1:\n{}\n\nP2:\n{}", day_01::p1(), day_01::p2())
        }
        _ => "".to_string(),
    };

    println!("Day {}:\n{}", day, answer);
}
