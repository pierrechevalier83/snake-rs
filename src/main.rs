extern crate matrix_display;
use matrix_display::*;

struct Board {
    n_cols: usize,
    snakes_position: usize,
    snake: Snake,
}

impl Board {
    fn new(size: usize, snake: Snake) -> Board {
        Board {
            n_cols: size,
            snakes_position: size * size / 2 + size / 2,
            snake: snake,
        }
    }
    fn n_cols(&self) -> usize {
        self.n_cols
    }
    fn snake_body(&self) -> Vec<usize> {
        let mut pos = self.snakes_position;
        self.snake
            .body
            .clone()
            .into_iter()
            .map(|x| {
                pos = match x {
                    // TODO: deal with walls
                    Direction::Left => pos - 1,
                    Direction::Right => pos + 1,
                    Direction::Up => pos - self.n_cols,
                    Direction::Down => pos + self.n_cols,
                };
                pos

            })
            .collect::<Vec<_>>()
    }
    fn data(&self) -> Vec<matrix_display::cell::Cell<char>> {
        let body = self.snake_body();
        (0..self.n_cols * self.n_cols)
            .map(|x| if x == self.snakes_position {
                     cell::Cell::new('@', 4, 16)
                 } else if body.contains(&x) {
                cell::Cell::new('o', 15, 16)
            } else {
                cell::Cell::new(' ', 0, 16)
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn opposite(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

#[derive(Clone)]
struct Tor<T>
    where T: Clone
{
    current_index: usize,
    end_index: usize,
    data: Vec<T>,
}

impl<T> Tor<T>
    where T: Clone
{
    fn new(v: Vec<T>) -> Tor<T> {
        Tor {
            current_index: 0,
            end_index: v.len() - 1,
            data: v,
        }
    }
    fn increment_index(&self, index: usize) -> usize {
        if index == self.data.len() - 1 {
            0
        } else {
            index + 1
        }
    }
    fn decrement_index(&self, index: usize) -> usize {
        if index == 0 {
            self.data.len() - 1
        } else {
            index - 1
        }
    }
    fn insert(&mut self, value: T) {
        self.data[self.end_index] = value;
        self.current_index = self.decrement_index(self.current_index);
        self.end_index = self.decrement_index(self.end_index);
    }
}

impl<T> Iterator for Tor<T>
    where T: Clone
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let value = if self.current_index == self.end_index {
            None
        } else {
            Some(self.data[self.current_index].clone())
        };
        self.current_index = self.increment_index(self.current_index);
        value
    }
}

struct Snake {
    body: Tor<Direction>,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: Tor::new(vec![Direction::Left,
                                Direction::Up,
                                Direction::Left,
                                Direction::Left,
                                Direction::Left,
                                Direction::Down,
                                Direction::Down,
                                Direction::Down,
                                Direction::Right]),
        }
    }
    fn crawl(&mut self, direction: Direction) {
        self.body.insert(opposite(direction));
    }
}

fn main() {
    let format = Format::new(3, 1);
    let mut snake = Snake::new();
    snake.crawl(Direction::Up);
    snake.crawl(Direction::Right);
    snake.crawl(Direction::Up);
    let board = Board::new(22, snake);
    let data = matrix::Matrix::new(board.n_cols(), board.data());
    let display = MatrixDisplay::new(format, data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::None);
}
