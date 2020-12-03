use std::env::args;

mod day_01;
mod day_02;
mod day_03;

fn main() {
    let args: Vec<String> = args().collect();
    let day: u32 = args[1].parse().expect("Usage:\n\tadvent2020 <day>");

    let answer = match day {
        1 => {
            format!("P1:\n{}\n\nP2:\n{}", day_01::p1(), day_01::p2())
        }
        2 => {
            format!("P1:\n{}\n\nP2:\n{}", day_02::p1(), day_02::p2())
        }
        3 => {
            format!("P1:\n{}\n\nP2:\nNYI", day_03::p1() /*, day_03::p2()*/)
        }
        _ => "".to_string(),
    };

    println!("Day {}:\n{}", day, answer);
}
