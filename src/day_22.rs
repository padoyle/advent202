use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    hash::{Hash, Hasher},
};

static INPUT: &'static str = include_str!("assets/day_22_input.txt");

fn parse_input(input: &str) -> (Vec<u8>, Vec<u8>) {
    let mut split = input.split("\n\n");
    let player1 = split.next().unwrap();
    let player2 = split.next().unwrap();

    let p1_deck: Vec<u8> = player1
        .lines()
        .skip(1)
        .map(|v| v.parse().unwrap())
        .collect();
    let p2_deck: Vec<u8> = player2
        .lines()
        .skip(1)
        .map(|v| v.parse().unwrap())
        .collect();

    (p1_deck, p2_deck)
}

static NO_WIN: usize = 0;
static P1_WIN: usize = 1;
static P2_WIN: usize = 2;

fn play_round(p1: &mut Vec<u8>, p2: &mut Vec<u8>) -> usize {
    match (p1.is_empty(), p2.is_empty()) {
        (false, false) => {
            let p1_draw = p1.remove(0);
            let p2_draw = p2.remove(0);
            if p1_draw > p2_draw {
                p1.push(p1_draw);
                p1.push(p2_draw);
            } else {
                p2.push(p2_draw);
                p2.push(p1_draw);
            }
            NO_WIN
        }
        (false, true) => P1_WIN,
        (true, false) => P2_WIN,
        _ => panic!("Where did the cards go?"),
    }
}

fn play_combat(p1: Vec<u8>, p2: Vec<u8>) -> Vec<u8> {
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

fn calculate_score(deck: Vec<u8>) -> u32 {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, card)| ((i + 1) as u32) * (*card as u32))
        .sum()
}

#[derive(Debug, Hash)]
struct GameState {
    game_id: usize,
    p1: Vec<u8>,
    p2: Vec<u8>,
}

impl GameState {
    fn new(id: usize, p1: Vec<u8>, p2: Vec<u8>) -> Self {
        Self {
            game_id: id,
            p1,
            p2,
        }
    }

    fn parse(input: &str) -> Self {
        let (p1, p2) = parse_input(input);
        Self { game_id: 0, p1, p2 }
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:\n\tP1: {:?}\n\tP2: {:?}",
            self.game_id, self.p1, self.p2
        )
    }
}

fn hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

fn play_recursive_combat(initial_state: GameState) -> (usize, Vec<u8>) {
    let mut seen_states: HashSet<u64> = HashSet::new();
    let mut state = initial_state;
    loop {
        // Check for first victory condition: previously seen state
        let hashed_state = hash(&state);
        println!("{} - {}", hashed_state, state);
        if seen_states.contains(&hashed_state) {
            return (P1_WIN, state.p1);
        }
        seen_states.insert(hashed_state);

        // Draw cards
        let p1_draw = state.p1.pop();
        let p2_draw = state.p2.pop();

        // Check for win conditions
        if state.p1.is_empty() {
            // Put p2's drawn card back on the stack before returning
            state.p2.push(p2_draw.unwrap());
            return (P2_WIN, state.p2);
        } else if state.p2.is_empty() {
            // Put p1's drawn card back on the stack before returning
            state.p1.push(p1_draw.unwrap());
            return (P1_WIN, state.p1);
        }

        // Check for a recursive subgame, and start one if needed
        let p1_draw = state.p1.remove(0) as usize;
        let p2_draw = state.p2.remove(0) as usize;
        let winner = if p1_draw <= state.p1.len() && p2_draw <= state.p2.len() {
            // Time for a sub-game!
            let p1_subdeck: Vec<u8> = state.p1.iter().rev().take(p1_draw).cloned().collect();
            let p2_subdeck: Vec<u8> = state.p2.iter().rev().take(p2_draw).cloned().collect();
            let subgame_state = GameState::new(state.game_id + 1, p1_subdeck, p2_subdeck);

            let (subwinner, _) = play_recursive_combat(subgame_state);
            subwinner
        } else {
            // Standard rules of play
            if p1_draw > p2_draw {
                P1_WIN
            } else {
                P2_WIN
            }
        };

        // Resolve winner for this round
        if winner == P1_WIN {
            state.p1.insert(0, p1_draw as u8);
            state.p1.insert(0, p2_draw as u8);
        } else {
            state.p2.insert(0, p2_draw as u8);
            state.p2.insert(0, p1_draw as u8);
        }
    }
}

pub fn p1() -> u32 {
    let (p1_deck, p2_deck) = parse_input(INPUT);
    calculate_score(play_combat(p1_deck, p2_deck))
}

pub fn p2() -> u32 {
    let initial_state = GameState::parse(INPUT);
    let (_, winning_deck) = play_recursive_combat(initial_state);
    calculate_score(winning_deck)
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
        let winner = play_combat(p1_deck, p2_deck);
        assert_eq!(306, calculate_score(winner));
    }

    #[test]
    fn p1_correct_answer() {
        let (p1_deck, p2_deck) = parse_input(INPUT);
        let winner = play_combat(p1_deck, p2_deck);
        assert_eq!(30780, calculate_score(winner));
    }

    #[test]
    #[ignore]
    fn p2_example() {
        let initial_state = GameState::parse(EXAMPLE);
        let (_, winning_deck) = play_recursive_combat(initial_state);
        assert_eq!(291, calculate_score(winning_deck));
    }

    // #[test]
    // fn p2_correct_answer() {
    // }
}
