use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

static INPUT: [u32; 9] = [4, 6, 7, 5, 2, 8, 1, 9, 3];

fn move_cups(cc: usize, cups: &mut Vec<u32>, max: u32) {
    let len = cups.len();
    let label = cups[cc];
    let picked_up = (
        std::mem::replace(&mut cups[(cc + 1) % len], 0),
        std::mem::replace(&mut cups[(cc + 2) % len], 0),
        std::mem::replace(&mut cups[(cc + 3) % len], 0),
    );
    let mut dest = if label > 1 { label - 1 } else { max };
    while dest == picked_up.0 || dest == picked_up.1 || dest == picked_up.2 {
        dest -= 1;
        if dest == 0 {
            dest = max;
        }
    }

    let mut counter = 0;
    loop {
        let index = (cc + 1 + counter) % len;
        cups[index] = cups[(index + 3) % len];
        if cups[(index + 3) % len] == dest {
            cups[(index + 1) % len] = picked_up.0;
            cups[(index + 2) % len] = picked_up.1;
            cups[(index + 3) % len] = picked_up.2;
            break;
        }
        counter += 1;
    }
}

#[derive(Debug)]
struct Node {
    value: u32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn next_value(&self) -> u32 {
        self.next
            .as_ref()
            .map(|node| node.try_borrow().unwrap().value)
            .unwrap()
    }
}

#[derive(Debug)]
struct Cups {
    nodes: HashMap<u32, Rc<RefCell<Node>>>,
}

impl Cups {
    fn from_input(input: &[u32], extend: usize) -> Self {
        let mut new_map = Self {
            nodes: HashMap::new(),
        };

        new_map.nodes.insert(
            input[0],
            Rc::new(RefCell::new(Node {
                value: input[0],
                next: None,
            })),
        );
        let mut prev_value = input[0];
        for i in 1..extend {
            let value = if i < input.len() {
                input[i]
            } else {
                (i + 1) as u32
            };
            let node = Rc::new(RefCell::new(Node { value, next: None }));
            new_map.nodes.insert(value, node.clone());
            new_map.get_mut(&prev_value).next = Some(node);
            prev_value = value;
        }
        println!("Size: {}", new_map.nodes.len());

        new_map
    }

    fn next_value(&self, value: &u32) -> u32 {
        self.nodes
            .get(value)
            .unwrap()
            .try_borrow()
            .unwrap()
            .next_value()
    }

    fn three_away(&self, value: &u32) -> RefMut<'_, Node> {
        let first_next = self.next_value(value);
        let second_next = self.next_value(&first_next);
        self.get_mut(&second_next)
    }

    fn get_mut(&self, value: &u32) -> RefMut<'_, Node> {
        let node = self.nodes.get(value).unwrap();
        node.borrow_mut()
    }

    fn move_three(&mut self, current: u32, dest: u32) {
        let mut current_node = self.get_mut(&current);
        let mut dest_node = self.get_mut(&dest);

        let mut third_node = self.three_away(&current);

        std::mem::swap(&mut current_node.next, &mut third_node.next);
        std::mem::swap(&mut third_node.next, &mut dest_node.next);
    }
}

fn play_game_p1(cups: &mut Vec<u32>) {
    let max = *cups.iter().max().unwrap();
    for cc in 0..100 {
        move_cups(cc % cups.len(), cups, max)
    }
}

fn get_result_p1(cups: &Vec<u32>) -> String {
    let len = cups.len();
    let start = cups.iter().position(|v| *v == 1).unwrap() + 1;
    let mut result = String::new();
    for counter in 0..len - 1 {
        let index = (start + counter) % len;
        result.push_str(cups[index].to_string().as_str());
    }
    result
}

fn get_p2_input(starting_input: &[u32]) -> Vec<u32> {
    let mut result: Vec<u32> = starting_input.iter().copied().collect();
    for i in starting_input.len()..1_000_000 {
        result.push((i + 1) as u32);
    }

    result
}

fn play_game_p2(cups: &mut Vec<u32>) {
    let max = *cups.iter().max().unwrap();
    // for cc in 0..10_000_000 {
    for cc in 0..500 {
        move_cups(cc % cups.len(), cups, max)
    }
}

fn get_result_p2(cups: &Vec<u32>) -> u64 {
    let len = cups.len();
    let one_pos = cups.iter().position(|v| *v == 1).unwrap();
    let a = cups[(one_pos + 1) % len];
    let b = cups[(one_pos + 2) % len];
    (a as u64) * (b as u64)
}

pub fn p1() -> String {
    let mut cups = INPUT.iter().copied().collect();
    play_game_p1(&mut cups);
    get_result_p1(&cups)
}

pub fn p2() -> u64 {
    let mut cups = get_p2_input(&INPUT);
    play_game_p2(&mut cups);
    get_result_p2(&cups)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: [u32; 9] = [3, 8, 9, 1, 2, 5, 4, 6, 7];

    #[test]
    fn ridiculous_hash_approach() {
        let test = Cups::from_input(&EXAMPLE, 10_000);
        assert_eq!(0, 1);
    }

    #[test]
    fn p1_example() {
        let mut cups = EXAMPLE.iter().copied().collect();
        play_game_p1(&mut cups);
        assert_eq!("67384529", get_result_p1(&cups));
    }

    #[test]
    fn p1_correct_answer() {
        let mut cups = INPUT.iter().copied().collect();
        play_game_p1(&mut cups);
        assert_eq!("43769582", get_result_p1(&cups));
    }

    #[test]
    #[ignore = "solution too slow, not ready yet"]
    fn p2_example() {
        let mut cups = get_p2_input(&EXAMPLE);
        play_game_p2(&mut cups);
        assert_eq!(149245887792, get_result_p2(&cups));
    }

    // #[test]
    // fn p2_correct_answer() {
    // }
}
