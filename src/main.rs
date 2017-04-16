extern crate matrix_display;
extern crate num;
extern crate rand;
extern crate termion;

use matrix_display::*;
use termion::event::{Key, Event};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout};

mod direction;
mod fruit;
mod snake;

use direction::Direction;
use snake::Snake;

/// Workaround strange behaviour of % operator in rust:
/// -1 % 10 returns -1 instead of 9!!!
fn modulo<T>(x: T, n: T) -> T where T: num::PrimInt {
    let m = x.rem(n);
	if m < T::zero() {
		m + n
	} else {
		m
	}
}

fn wrap_inc(x: &mut isize, n: isize) {
    *x = modulo(*x + 1, n);
}

fn wrap_dec(x: &mut isize, n: isize) {
    *x = modulo(*x - 1, n);
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point {
            x: x,
            y: y,
        }
    }
}

fn move_point(direction: &Direction, p: &mut Point<isize>, bounds: &Point<isize>) {
    match *direction {
        Direction::Left => wrap_dec(&mut p.x, bounds.x),
        Direction::Right => wrap_inc(&mut p.x, bounds.x),
        Direction::Up => wrap_dec(&mut p.y, bounds.y),
        Direction::Down => wrap_inc(&mut p.y, bounds.y),
    };
}

fn random_point(bounds: &Point<isize>) -> Point<isize> {
    Point::new(modulo(rand::random::<isize>(), bounds.x),
               modulo(rand::random::<isize>(), bounds.y))

}

enum Status {
    Hungry,
    Fed,
    Dead,
}

struct Game {
    size: Point<isize>,
    snake: (Point<isize>, Snake),
    fruit: (Point<isize>, fruit::Fruit),
}

impl Game {
    fn new(n_cols: isize) -> Game {
        Game {
            size: Point::new(n_cols, n_cols),
            snake: (Point::new(n_cols / 2, n_cols / 2), Snake::new()),
            fruit: (random_point(&Point::new(n_cols, n_cols)), fruit::get_random_fruit()),
        }
    }
    fn n_cols(&self) -> isize {
        self.size.x
    }
    fn fruit(&self) -> fruit::Fruit {
        self.fruit.1.clone()
    }
    fn spawn_fruit(&mut self) {
        self.fruit = (random_point(&self.size),
                      fruit::get_random_fruit());
    }
    fn process_input(&mut self, direction: &mut Direction) -> Status {
        if *direction == direction::opposite(&self.snake.1.direction()) {
            *direction = self.snake.1.direction()
        }
        move_point(&direction,
                   &mut self.snake.0,
                   &self.size);
        let mut status = Status::Hungry;
        if self.fruit.0 == self.snake.0 {
            self.spawn_fruit();
            self.snake.1.grow(direction);
            status = Status::Fed;
        } else {
            self.snake.1.crawl(direction);
        }
        if self.snake_body()
               .iter()
               .skip(1)
               .collect::<Vec<_>>()
               .contains(&&self.snake.0) {
            status = Status::Dead
        }
        status
    }
    fn snake_body(&self) -> Vec<Point<isize>> {
        let mut pos = self.snake.0.clone();
        self.snake.1
            .body
            .iter()
            .map(|dir| {
                     move_point(dir, &mut pos, &self.size);
                     pos.clone()
                 })
            .collect::<Vec<_>>()
    }
    fn board(&self) -> Vec<matrix_display::cell::Cell<char>> {
        let bg_col = 233;
        let head_col = 21;
        let body_col = 32;
        let body = self.snake_body();
        (0..self.size.x * self.size.y)
            .map(|i| Point::new(modulo(i, self.size.x), i / self.size.y))
            .map(|pos|
                 if pos == self.snake.0 {
                     cell::Cell::new('▣', head_col, bg_col)
                 } else if body.contains(&pos) {
                     cell::Cell::new('◼', body_col, bg_col)
                 } else if self.fruit.0 == pos {
                     cell::Cell::new(self.fruit.1.symbol, self.fruit.1.color, bg_col)
                 } else {
                     cell::Cell::new(' ', bg_col, bg_col)
                 })
            .collect::<Vec<_>>()
    }
}

fn print_game<W>(game: &Game, stdout: &mut W)
    where W: Write
{
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

fn pick_a_size() -> isize {
    match termion::terminal_size() {
        Ok((n_cols, n_rows)) => std::cmp::min((n_cols / 3) as isize, std::cmp::min(n_rows as isize, 40)),
        Err(_) => 20,
    }
}

fn main() {
    let mut stdin = termion::async_stdin().events();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut game = Game::new(pick_a_size());
    print_game(&game, &mut stdout);
    let mut direction = Direction::Right;
    let mut speed = 100;
    loop {
        if let Some(evt) = stdin.next() {
            match evt.unwrap() {
                Event::Key(Key::Char('q')) => {
                    break;
                }
                Event::Key(Key::Char('w')) |
                Event::Key(Key::Char('k')) |
                Event::Key(Key::Up) => {
                    direction = Direction::Up;
                }
                Event::Key(Key::Char('s')) |
                Event::Key(Key::Char('j')) |
                Event::Key(Key::Down) => {
                    direction = Direction::Down;
                }
                Event::Key(Key::Char('a')) |
                Event::Key(Key::Char('h')) |
                Event::Key(Key::Left) => {
                    direction = Direction::Left;
                }
                Event::Key(Key::Char('d')) |
                Event::Key(Key::Char('l')) |
                Event::Key(Key::Right) => {
                    direction = Direction::Right;
                }
                _ => (),
            };
        }
        let status = game.process_input(&mut direction);
        match status {
            Status::Dead => {
                break;
            }
            Status::Fed => {
                // increase the speed every time a fruit is eaten
                speed += 1;
            }
            Status::Hungry => (),
        };
        if game.fruit().rotten() {
            game.spawn_fruit();
        }
        print_game(&game, &mut stdout);
        std::thread::sleep(std::time::Duration::from_millis((10000 / speed) as u64));
    }
}
