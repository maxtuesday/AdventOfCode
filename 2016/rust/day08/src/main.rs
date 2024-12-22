fn main() {
    let input = include_str!("../../../input/day08.txt");

    let mut screen = Screen::new(50, 6);
    let operations = parse(input);

    screen.handle_operations(&operations);

    println!("Part 1: {}", part1(&screen));
    println!("Part 2:");
    part2(&screen);
}

enum RotateKind {
    Row,
    Column,
}

enum Operation {
    Rect {
        cols: usize,
        rows: usize,
    },
    Rotate {
        kind: RotateKind,
        index: usize,
        shift: usize,
    },
}

fn parse(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|line| {
            let (prefix, rest) = line.split_once(" ").unwrap();
            match prefix {
                "rect" => {
                    let (cols, rows) = rest.split_once("x").unwrap();
                    let cols = cols.parse::<usize>().unwrap();
                    let rows = rows.parse::<usize>().unwrap();
                    Operation::Rect { cols, rows }
                }
                "rotate" => {
                    let (kind, rest) = rest.split_once(" ").unwrap();
                    let tokens = rest
                        .split_once("=")
                        .unwrap()
                        .1
                        .split(" ")
                        .filter_map(|token| token.parse::<usize>().ok())
                        .collect::<Vec<_>>();
                    let kind = match kind {
                        "column" => RotateKind::Column,
                        "row" => RotateKind::Row,
                        c => unimplemented!("unknown rotate kind: {c}"),
                    };
                    Operation::Rotate {
                        kind,
                        index: tokens[0],
                        shift: tokens[1],
                    }
                }
                s => unimplemented!("unknown prefix: {s}"),
            }
        })
        .collect()
}

struct Screen {
    pixels: Vec<Vec<char>>,
}

impl Screen {
    fn new(w: usize, h: usize) -> Self {
        Self {
            pixels: vec![vec!['.'; w]; h],
        }
    }

    fn display(&self) {
        println!("{}", self);
    }

    fn handle_operation(&mut self, op: &Operation) {
        match op {
            Operation::Rect { cols, rows } => {
                // turn "on" all the pixels within cols and rows
                for row in 0..*rows {
                    for col in 0..*cols {
                        self.pixels[row][col] = '#';
                    }
                }
            }
            Operation::Rotate { kind, index, shift } => match kind {
                RotateKind::Row => {
                    self.pixels[*index].rotate_right(*shift);
                }
                RotateKind::Column => {
                    let mut column = Vec::new();
                    for r in 0..self.pixels.len() {
                        column.push(self.pixels[r][*index]);
                    }
                    column.rotate_right(*shift);
                    for r in 0..self.pixels.len() {
                        self.pixels[r][*index] = column[r];
                    }
                }
            },
        }
    }

    fn handle_operations(&mut self, ops: &Vec<Operation>) {
        for op in ops {
            self.handle_operation(op);
        }
    }
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.pixels {
            for col in row {
                write!(f, "{col}")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

fn part1(screen: &Screen) -> usize {
    screen
        .pixels
        .iter()
        .flat_map(|row| row.iter().filter(|c| **c == '#'))
        .count()
}

fn part2(screen: &Screen) {
    screen.display()
}
