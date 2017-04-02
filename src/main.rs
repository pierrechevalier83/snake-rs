extern crate matrix_display;
extern crate termion;

use matrix_display::*;
use termion::event::{Key, Event};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout};
use std::collections::VecDeque;

fn wrap_inc(x: &mut isize, n: isize) {
    *x = (*x + 1) % n;
}

fn wrap_dec(x: &mut isize, n: isize) {
    // Workaround strange behaviour of modulo operator in rust:
	// -1 % 10 returns -1 instead of 9!!!
    *x = ((*x - 1) + n) % n;
}

fn move_point(direction: &Direction, (x, y): (&mut isize, &mut isize), bounds: (isize, isize)) {
    match *direction {
        Direction::Left => wrap_dec(x, bounds.0),
        Direction::Right => wrap_inc(x, bounds.0),
        Direction::Up => wrap_dec(y, bounds.1),
        Direction::Down => wrap_inc(y, bounds.1),
    };
}

enum Status {
    Alive,
	Dead,
	Quit,
}

struct Game {
    n_cols: isize,
    snakes_position: (isize, isize),
    snake: Snake,
}

impl Game {
    fn new(size: isize) -> Game {
        Game {
            n_cols: size,
            snakes_position: (size / 2, size / 2),
            snake: Snake::new(),
        }
    }
    fn n_cols(&self) -> isize {
        self.n_cols
    }
    fn process_input(&mut self, direction: Direction) -> Status {
        move_point(&direction, (&mut self.snakes_position.0, &mut self.snakes_position.1), (self.n_cols, self.n_cols));
        self.snake.crawl(direction);
		if self.snake_body().iter().skip(1).collect::<Vec<_>>().contains(&&self.snakes_position) {
		    Status::Dead
		} else {
		    Status::Alive
		}
    }
    fn snake_body(&self) -> Vec<(isize, isize)> {
        let (mut x, mut y) = self.snakes_position;
        self.snake
            .body
            .iter()
            .map(|dir| {
                     move_point(dir, (&mut x, &mut y), (self.n_cols, self.n_cols));
                     (x, y)
                 })
            .collect::<Vec<_>>()
    }
    fn board(&self) -> Vec<matrix_display::cell::Cell<char>> {
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

fn print_game<W>(game: &Game, stdout: &mut W) where W: Write {
    let data = matrix::Matrix::new(game.n_cols() as usize, game.board());
    let format = Format::new(3, 1);
    let display = MatrixDisplay::new(format, data);
    write!(stdout,
           "{}{}{}",
           termion::clear::All,
           termion::cursor::Hide,
           termion::cursor::Goto(1, 1))
            .unwrap();
    display.print(stdout, &style::BordersStyle::None);
    stdout.flush().unwrap();
}

fn main() {
    let mut stdin = termion::async_stdin().events();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut game = Game::new(22);
    game.snake = Snake::new();
    print_game(&game, &mut stdout);
    loop {
        if let Some(evt) = stdin.next() {
            let status = match evt.unwrap() {
                Event::Key(Key::Char('q')) => {
                    Status::Quit
                }
                Event::Key(Key::Up) => {
                    game.process_input(Direction::Up)
                }
                Event::Key(Key::Down) => {
                    game.process_input(Direction::Down)
                }
                Event::Key(Key::Left) => {
                    game.process_input(Direction::Left)
                }
                Event::Key(Key::Right) => {
                    game.process_input(Direction::Right)
                }
                _ => Status::Alive,
            };
			match status {
			    Status::Quit => { break; },
				Status::Dead => { break; },
				Status::Alive => (),
			};
    		print_game(&game, &mut stdout);
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
