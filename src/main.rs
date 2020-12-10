#[macro_use]
extern crate maplit;

use std::env::args;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
// mod day_11;
// mod day_12;
// mod day_13;
// mod day_14;
// mod day_15;
// mod day_16;
// mod day_17;
// mod day_18;
// mod day_19;
// mod day_20;
// mod day_21;
// mod day_22;
// mod day_23;
// mod day_24;
// mod day_25;

fn main() {
    let args: Vec<String> = args().collect();
    let day: u32 = args[1].parse().expect("Usage:\n\tadvent2020 <day>");

    let answer = match day {
        1 => format!("P1:\n{}\n\nP2:\n{}", day_01::p1(), day_01::p2()),
        2 => format!("P1:\n{}\n\nP2:\n{}", day_02::p1(), day_02::p2()),
        3 => format!("P1:\n{}\n\nP2:\n{}", day_03::p1(), day_03::p2()),
        4 => format!("P1:\n{}\n\nP2:\n{}", day_04::p1(), day_04::p2()),
        5 => format!("P1:\n{}\n\nP2:\n{}", day_05::p1(), day_05::p2()),
        6 => format!("P1:\n{}\n\nP2:\n{}", day_06::p1(), day_06::p2()),
        7 => format!("P1:\n{}\n\nP2:\n{}", day_07::p1(), day_07::p2()),
        8 => format!("P1:\n{}\n\nP2:\n{}", day_08::p1(), day_08::p2()),
        9 => format!("P1:\n{}\n\nP2:\n{}", day_09::p1(), day_09::p2()),
        10 => format!("P1:\n{}\n\nP2:\n{}", day_10::p1(), day_10::p2()),
        // 11 => format!("P1:\n{}\n\nP2:\n{}", day_11::p1(), day_11::p2()),
        // 12 => format!("P1:\n{}\n\nP2:\n{}", day_12::p1(), day_12::p2()),
        // 13 => format!("P1:\n{}\n\nP2:\n{}", day_13::p1(), day_13::p2()),
        // 14 => format!("P1:\n{}\n\nP2:\n{}", day_14::p1(), day_14::p2()),
        // 15 => format!("P1:\n{}\n\nP2:\n{}", day_15::p1(), day_15::p2()),
        // 16 => format!("P1:\n{}\n\nP2:\n{}", day_16::p1(), day_16::p2()),
        // 17 => format!("P1:\n{}\n\nP2:\n{}", day_17::p1(), day_17::p2()),
        // 18 => format!("P1:\n{}\n\nP2:\n{}", day_18::p1(), day_18::p2()),
        // 19 => format!("P1:\n{}\n\nP2:\n{}", day_19::p1(), day_19::p2()),
        // 20 => format!("P1:\n{}\n\nP2:\n{}", day_20::p1(), day_20::p2()),
        // 21 => format!("P1:\n{}\n\nP2:\n{}", day_21::p1(), day_21::p2()),
        // 22 => format!("P1:\n{}\n\nP2:\n{}", day_22::p1(), day_22::p2()),
        // 23 => format!("P1:\n{}\n\nP2:\n{}", day_23::p1(), day_23::p2()),
        // 24 => format!("P1:\n{}\n\nP2:\n{}", day_24::p1(), day_24::p2()),
        // 25 => format!("P1:\n{}\n\nP2:\n{}", day_25::p1(), day_25::p2()),
        _ => "".to_string(),
    };

    println!("Day {}:\n{}", day, answer);
}
