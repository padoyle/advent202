static INPUT: &str = include_str!("assets/day_18_input.txt");

#[derive(Debug, Clone)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn apply(&self, prev: i64, next: i64) -> i64 {
        match self {
            Op::Add => prev + next,
            Op::Mul => prev * next,
        }
    }
}

fn evaluate_p1(expression: &str) -> i64 {
    // let mut depth = 0;
    let mut values: Vec<i64> = vec![0];
    let mut ops: Vec<Op> = vec![Op::Add];
    for c in expression.chars() {
        match c {
            ' ' => {
                continue;
            }
            '+' => ops.push(Op::Add),
            '*' => ops.push(Op::Mul),
            '(' => {
                // initialize with implicit 0 and add
                values.push(0);
                ops.push(Op::Add);
            }
            ')' => {
                let resolved_subvalue = values.pop().unwrap();
                // Apply op between value at new depth, and value we just dropped back from
                let op = ops.pop().unwrap();
                let applied = op.apply(values.pop().unwrap(), resolved_subvalue);
                values.push(applied);
            }
            num => {
                let op = ops.pop().unwrap();
                let previous = values.pop().unwrap();
                let new_value: i64 = num
                    .to_string()
                    .parse()
                    .unwrap_or_else(|_e| panic!("Invalid character: {}", num));
                values.push(op.apply(previous, new_value));
            }
        }
    }

    values[0]
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Add,
    Mul,
    Value(i64),
    SubExpr(Box<Vec<Expr>>),
}

impl Expr {
    fn parse(input: &str) -> Expr {
        Self::parse_from_index(input.as_bytes(), 0).0
    }

    fn parse_from_index(input: &[u8], index: usize) -> (Expr, usize) {
        let mut result = Vec::new();
        let mut i = index;
        while i < input.len() {
            match input[i] as char {
                ' ' => {}
                '+' => result.push(Expr::Add),
                '*' => result.push(Expr::Mul),
                '(' => {
                    let (sub, skip_to) = Self::parse_from_index(&input, i + 1);
                    result.push(sub);
                    i = skip_to;
                }
                ')' => {
                    return (Expr::SubExpr(Box::new(result)), i);
                }
                num => {
                    let value: i64 = num
                        .to_string()
                        .parse()
                        .unwrap_or_else(|_e| panic!("Invalid character: {}", num));
                    result.push(Expr::Value(value));
                }
            }
            i += 1;
        }

        (Expr::SubExpr(Box::new(result)), i)
    }
}
// 2309

fn resolve(expr: Expr) -> i64 {
    if let Expr::SubExpr(values) = expr {
        let mut reduced: Vec<Expr> = values.iter().map(|expr| (*expr).to_owned()).collect();
        // Addition first!
        while let Some(index) = reduced.iter().position(|value| value == &Expr::Add) {
            let index = index - 1;
            let left = reduced.remove(index);
            let right = reduced.remove(index + 1);
            let new_value = Expr::Value(resolve(left) + resolve(right));
            reduced[index] = new_value;
        }

        while let Some(index) = reduced.iter().position(|value| value == &Expr::Mul) {
            let index = index - 1;
            let left = reduced.remove(index);
            let right = reduced.remove(index + 1);
            let new_value = Expr::Value(resolve(left) * resolve(right));
            reduced[index] = new_value;
        }

        assert!(reduced.len() == 1);
        return resolve(reduced[0].to_owned());
    } else if let Expr::Value(value) = expr {
        return value;
    }

    unreachable!("Invalid expression, cannot parse");
}

fn evaluate_p2(expression: &str) -> i64 {
    resolve(Expr::parse(expression))
}

fn evaluate_all_p1(input: &str) -> i64 {
    input.lines().map(evaluate_p1).sum()
}

fn evaluate_all_p2(input: &str) -> i64 {
    input.lines().map(evaluate_p2).sum()
}

pub fn p1() -> i64 {
    evaluate_all_p1(INPUT)
}

pub fn p2() -> i64 {
    evaluate_all_p2(INPUT)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_example() {
        assert_eq!(26, evaluate_p1("2 * 3 + (4 * 5)"));
        assert_eq!(437, evaluate_p1("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(
            12240,
            evaluate_p1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
        assert_eq!(
            13632,
            evaluate_p1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }

    #[test]
    fn p1_correct_answer() {
        assert_eq!(464478013511, evaluate_all_p1(INPUT));
    }

    #[test]
    fn p2_example() {
        assert_eq!(231, evaluate_p2("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(46, evaluate_p2("2 * 3 + (4 * 5)"));
        assert_eq!(1445, evaluate_p2("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(
            669060,
            evaluate_p2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
        assert_eq!(
            23340,
            evaluate_p2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }

    // #[test]
    // fn p2_correct_answer() {}
}
