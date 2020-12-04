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

fn find_trees(tree_grid: &TreeGrid, row_step: usize, col_step: usize) -> usize {
    let mut count = 0;
    let mut col = 0;
    for row in (0..tree_grid.height()).step_by(row_step) {
        if tree_grid.has_tree_at(row, col) {
            count += 1;
        }
        col += col_step;
    }
    count
}

fn multiply_paths(tree_grid: &TreeGrid) -> usize {
    vec![
        find_trees(tree_grid, 1, 1),
        find_trees(tree_grid, 1, 3),
        find_trees(tree_grid, 1, 5),
        find_trees(tree_grid, 1, 7),
        find_trees(tree_grid, 2, 1),
    ]
    .iter()
    .fold(1, |acc, value| acc * value)
}

pub fn p1() -> usize {
    let tree_grid = parse_input(INPUT);
    find_trees(&tree_grid, 1, 3)
}

pub fn p2() -> usize {
    let tree_grid = parse_input(INPUT);
    multiply_paths(&tree_grid)
}

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
        let tree_count = find_trees(&tree_grid, 1, 3);

        assert_eq!(7, tree_count);
    }

    #[test]
    fn p1_correct_answer() {
        let tree_grid = parse_input(INPUT);
        let tree_count = find_trees(&tree_grid, 1, 3);

        assert_eq!(292, tree_count);
    }

    #[test]
    fn p2_example() {
        let tree_grid = parse_input(EXAMPLE);
        let multiplied_paths = multiply_paths(&tree_grid);

        assert_eq!(336, multiplied_paths);
    }

    #[test]
    fn p2_correct_answer() {
        let tree_grid = parse_input(INPUT);
        let multiplied = multiply_paths(&tree_grid);

        assert_eq!(9354744432, multiplied);
    }
}
