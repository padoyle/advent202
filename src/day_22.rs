static INPUT: &'static str = include_str!("assets/day_22_input.txt");

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut split = input.split("\n\n");
    let player1 = split.next().unwrap();
    let player2 = split.next().unwrap();

    let mut p1_deck: Vec<u32> = player1
        .lines()
        .skip(1)
        .map(|v| v.parse().unwrap())
        .collect();
    p1_deck.reverse();
    let mut p2_deck: Vec<u32> = player2
        .lines()
        .skip(1)
        .map(|v| v.parse().unwrap())
        .collect();
    p2_deck.reverse();

    (p1_deck, p2_deck)
}

static NO_WIN: usize = 0;
static P1_WIN: usize = 1;
static P2_WIN: usize = 2;

fn play_round(p1: &mut Vec<u32>, p2: &mut Vec<u32>) -> usize {
    match (p1.is_empty(), p2.is_empty()) {
        (false, false) => {
            let p1_draw = p1.pop().unwrap();
            let p2_draw = p2.pop().unwrap();
            if p1_draw > p2_draw {
                p1.insert(0, p1_draw);
                p1.insert(0, p2_draw);
            } else {
                p2.insert(0, p2_draw);
                p2.insert(0, p1_draw);
            }
            NO_WIN
        }
        (false, true) => P1_WIN,
        (true, false) => P2_WIN,
        _ => panic!("Where did the cards go?"),
    }
}

fn play_game(p1: Vec<u32>, p2: Vec<u32>) -> Vec<u32> {
    let mut p1 = p1;
    let mut p2 = p2;
    let mut last_round = NO_WIN;
    while last_round == NO_WIN {
        last_round = play_round(&mut p1, &mut p2);
    }

    if last_round == P1_WIN {
        p1
    } else {
        p2
    }
}

fn calculate_score(deck: Vec<u32>) -> u32 {
    deck.iter()
        .enumerate()
        .map(|(i, card)| ((i + 1) as u32) * card)
        .sum()
}

pub fn p1() -> u32 {
    let (p1_deck, p2_deck) = parse_input(INPUT);
    calculate_score(play_game(p1_deck, p2_deck))
}

pub fn p2() -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;

    #[test]
    fn p1_example() {
        let (p1_deck, p2_deck) = parse_input(EXAMPLE);
        let winner = play_game(p1_deck, p2_deck);
        assert_eq!(306, calculate_score(winner));
    }

    #[test]
    fn p1_correct_answer() {
        let (p1_deck, p2_deck) = parse_input(INPUT);
        let winner = play_game(p1_deck, p2_deck);
        assert_eq!(30780, calculate_score(winner));
    }

    // #[test]
    // fn p2_simple() {
    // }

    // #[test]
    // fn p2_example() {
    // }
}
