extern crate rand;

use direction::Direction;
use modulo;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point {
            x: x,
            y: y,
        }
    }
}

pub fn move_point(direction: &Direction, p: &mut Point<isize>, bounds: &Point<isize>) {
    match *direction {
        Direction::Left => modulo::wrap_dec(&mut p.x, bounds.x),
        Direction::Right => modulo::wrap_inc(&mut p.x, bounds.x),
        Direction::Up => modulo::wrap_dec(&mut p.y, bounds.y),
        Direction::Down => modulo::wrap_inc(&mut p.y, bounds.y),
    };
}

pub fn random_point(bounds: &Point<isize>) -> Point<isize> {
    Point::new(modulo::modulo(rand::random::<isize>(), bounds.x),
               modulo::modulo(rand::random::<isize>(), bounds.y))

}
