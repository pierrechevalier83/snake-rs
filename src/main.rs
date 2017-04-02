extern crate matrix_display;
extern crate termion;

use matrix_display::*;
use termion::event::{Key, Event};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout};
use std::collections::VecDeque;

struct Board {
    n_cols: usize,
    snakes_position: (usize, usize),
    snake: Snake,
}

impl Board {
    fn new(size: usize) -> Board {
        Board {
            n_cols: size,
            snakes_position: (size / 2, size / 2),
            snake: Snake::new(),
        }
    }
    fn n_cols(&self) -> usize {
        self.n_cols
    }
    fn snake_body(&self) -> Vec<(usize, usize)> {
        let (mut x, mut y) = self.snakes_position;
        self.snake
            .body
            .iter()
            .map(|dir| {
                match dir {
                    // TODO: deal with collisions
                    &Direction::Left => {
                        x -= 1;
                    }
                    &Direction::Right => {
                        x += 1;
                    }
                    &Direction::Up => {
                        y -= 1;
                    }
                    &Direction::Down => {
                        y += 1;
                    }
                };
                (x, y)
            })
            .collect::<Vec<_>>()
    }
    fn data(&self) -> Vec<matrix_display::cell::Cell<char>> {
        let body = self.snake_body();
        (0..self.n_cols * self.n_cols)
            .map(|i| (i % self.n_cols, i / self.n_cols))
            .map(|pos| if pos == self.snakes_position {
                     cell::Cell::new('@', 4, 8)
                 } else if body.contains(&pos) {
                cell::Cell::new('o', 15, 8)
            } else {
                cell::Cell::new(' ', 0, 8)
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
struct Snake {
    body: VecDeque<Direction>,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: vec![Direction::Left,
                       Direction::Up,
                       Direction::Left,
                       Direction::Left,
                       Direction::Left,
                       Direction::Down,
                       Direction::Down,
                       Direction::Down,
                       Direction::Right]
                    .into_iter()
                    .collect(),
        }
    }
    fn crawl(&mut self, direction: Direction) {
        self.body.push_front(opposite(direction));
        self.body.pop_back();
    }
}

fn main() {
    let mut stdin = termion::async_stdin().events();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut board = Board::new(22);
    board.snake = Snake::new();
    let data = matrix::Matrix::new(board.n_cols(), board.data());
    let format = Format::new(3, 1);
    let display = MatrixDisplay::new(format, data);
    write!(stdout,
           "{}{}{}",
           termion::clear::All,
           termion::cursor::Hide,
           termion::cursor::Goto(1, 1))
            .unwrap();
    display.print(&mut stdout, &style::BordersStyle::None);
    stdout.flush().unwrap();

    loop {
        if let Some(evt) = stdin.next() {
            match evt.unwrap() {
                Event::Key(Key::Char('q')) => {
                    break;
                }
                Event::Key(Key::Up) => {
                    board.snake.crawl(Direction::Up);
                }
                Event::Key(Key::Down) => {
                    board.snake.crawl(Direction::Down);
                }
                Event::Key(Key::Left) => {
                    board.snake.crawl(Direction::Left);
                }
                Event::Key(Key::Right) => {
                    board.snake.crawl(Direction::Right);
                }
                _ => (),
            };
            let data = matrix::Matrix::new(board.n_cols(), board.data());
            let disp = MatrixDisplay::new(Format::new(3, 1), data);
            write!(stdout,
                   "{}{}{}",
                   termion::clear::All,
                   termion::cursor::Hide,
                   termion::cursor::Goto(1, 1))
                    .unwrap();
            disp.print(&mut stdout, &style::BordersStyle::None);
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
        stdout.flush().unwrap();
    }
}
