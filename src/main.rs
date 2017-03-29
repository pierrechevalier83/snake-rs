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
            end_index: v.len(),
            data: v,
        }
    }
}

impl<T> Iterator for Tor<T>
    where T: Clone
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let value = if self.current_index == self.end_index - 1 {
            None
        } else {
            Some(self.data[self.current_index].clone())
        };
        self.current_index = (self.current_index + 1) % self.data.len();
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
}

fn main() {
    let format = Format::new(3, 1);
    let snake = Snake::new();
    let board = Board::new(22, snake);
    let data = matrix::Matrix::new(board.n_cols(), board.data());
    let display = MatrixDisplay::new(format, data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::None);
}
