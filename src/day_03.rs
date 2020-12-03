use std::collections::HashSet;

static INPUT: &'static str = include_str!("assets/day_03_input.txt");

#[derive(Debug)]
struct TreeGrid {
    width: usize,
    trees: Vec<HashSet<usize>>,
}

impl TreeGrid {
    fn has_tree_at(&self, row: usize, col: usize) -> bool {
        let col_wrapped = col % self.width;
        self.trees[row].get(&col_wrapped).is_some()
    }

    fn height(&self) -> usize {
        self.trees.len()
    }
}

fn parse_input(input: &'static str) -> TreeGrid {
    let width = input.lines().next().unwrap().len();
    let trees: Vec<HashSet<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(i, value)| match value {
                    '#' => Some(i),
                    '.' => None,
                    _ => panic!("bad input"),
                })
                .collect()
        })
        .collect();

    TreeGrid { width, trees }
}

fn find_trees_p1(tree_grid: &TreeGrid) -> usize {
    let mut count = 0;
    for row in 0..tree_grid.height() {
        if tree_grid.has_tree_at(row, row * 3) {
            count += 1;
        }
    }
    count
}

pub fn p1() -> usize {
    let tree_grid = parse_input(INPUT);
    find_trees_p1(&tree_grid)
}

// pub fn p2() -> usize {
//     let input = process_input(INPUT);
//     get_valid_passwords_p2(input)
// }

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &'static str = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

    #[test]
    fn p1_example() {
        let tree_grid = parse_input(EXAMPLE);
        let tree_count = find_trees_p1(&tree_grid);

        assert_eq!(7, tree_count);
    }

    #[test]
    fn p1_correct_answer() {
        let tree_grid = parse_input(INPUT);
        let tree_count = find_trees_p1(&tree_grid);

        assert_eq!(292, tree_count);
    }
}
