extern crate matrix_display;
extern crate num;
extern crate rand;
extern crate termion;
extern crate tui;

use matrix_display::*;
use termion::event::{Key, Event};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout};

mod direction;
mod game;
mod modulo;
mod point;
mod fruit;
mod snake;

use direction::Direction;
use game::Game;
use game::Status;
use tui::Terminal;
use tui::backend::TermionBackend;

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
    write!(stdout, "{}Score: {} \r\n", termion::color::Bg(termion::color::AnsiValue(233)), game.score()).unwrap();
    display.print(stdout, &style::BordersStyle::None);
    stdout.flush().unwrap();
}

fn pick_a_size(terminal: &Terminal<TermionBackend>) -> isize {
    match terminal.size() {
        Ok(r) => std::cmp::min((r.width / 3) as isize, std::cmp::min(r.height as isize, 40)),
        Err(_) => 20,
    }
}

fn main() {
    let terminal = Terminal::new(TermionBackend::new().unwrap()).unwrap();
    let mut stdin = termion::async_stdin().events();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut game = Game::new(pick_a_size(&terminal));
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
                Event::Key(Key::Char(' ')) => {
                    while !stdin.next().is_some() {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
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
        game.refresh();
        print_game(&game, &mut stdout);
        std::thread::sleep(std::time::Duration::from_millis((10000 / speed) as u64));
    }
}
