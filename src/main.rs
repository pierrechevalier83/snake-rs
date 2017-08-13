extern crate matrix_display;
extern crate ansi_term;
extern crate num;
extern crate rand;
extern crate termion;
extern crate tui;

use matrix_display::*;
use termion::event::{Key, Event};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout};

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
use tui::widgets::{Block, Paragraph, Widget};
use tui::layout::{Group, Size};

fn print_game<W>(game: &Game, stdout: &mut W, terminal: &mut Terminal<TermionBackend>, previous_string: &mut String)
    where W: Write
{
    let mut data = matrix::Matrix::new(game.n_cols() as usize, game.board());
    let format = Format::new(3, 1);
    let display = MatrixDisplay::new(&format, &mut data);
    let raw = display.render(&style::BordersStyle::None);
    let new_string = format!("{}Score: {}\r\n{}", termion::color::Bg(termion::color::AnsiValue(233)), game.score(), ansi_term::ANSIStrings(&*raw));
    if *previous_string == String::new() {
        *previous_string = new_string.lines().map(|_| { "\r\n" } ).collect();
    }
    write!(stdout, "{}\r\n", termion::cursor::Goto(1,1)).unwrap();
    new_string
        .lines()
        .zip(previous_string.lines())
        .map(|(new, old)| {
            if new == old {
                write!(stdout, "\r\n").unwrap();
            } else {
                write!(stdout, "{}\r\n", new).unwrap();
            }
        })
        .collect::<Vec<_>>();
    *previous_string = new_string.clone();
}

fn pick_a_size(terminal: &Terminal<TermionBackend>) -> isize {
    match terminal.size() {
        Ok(r) => std::cmp::min((r.width / 3) as isize, (r.height - 4) as isize),
        Err(_) => 20,
    }
}

fn main() {
    let mut terminal = Terminal::new(TermionBackend::new().unwrap()).unwrap();
    let mut stdin = termion::async_stdin().events();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut previous_string = String::new();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    let mut game = Game::new(pick_a_size(&terminal));
    terminal.clear().unwrap();
    print_game(&game, &mut stdout, &mut terminal, &mut previous_string);
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
        print_game(&game, &mut stdout, &mut terminal, &mut previous_string);
        std::thread::sleep(std::time::Duration::from_millis((10000 / speed) as u64));
    }
}
