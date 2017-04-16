use point;
use direction;
use modulo;
use fruit;

use matrix_display::*;
use direction::Direction;
use snake::Snake;
use point::Point;

pub enum Status {
    Hungry,
    Fed,
    Dead,
}

pub struct Game {
    size: Point<isize>,
    snake: (Point<isize>, Snake),
    fruit: (Point<isize>, fruit::Fruit),
    score: i32,
}

impl Game {
    pub fn new(n_cols: isize) -> Game {
        Game {
            size: Point::new(n_cols, n_cols),
            snake: (Point::new(n_cols / 2, n_cols / 2), Snake::new()),
            fruit: (point::random_point(&Point::new(n_cols, n_cols)), fruit::get_random_fruit()),
            score: 0,
        }
    }
    pub fn n_cols(&self) -> isize {
        self.size.x
    }
    pub fn refresh(&mut self) {
        if self.fruit.1.rotten() {
            self.spawn_fruit();
        }
    }
    pub fn process_input(&mut self, direction: &mut Direction) -> Status {
        if *direction == direction::opposite(&self.snake.1.direction()) {
            *direction = self.snake.1.direction()
        }
        point::move_point(&direction,
                   &mut self.snake.0,
                   &self.size);
        let mut status = Status::Hungry;
        if self.fruit.0 == self.snake.0 {
            self.score += self.fruit.1.score_value();
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
    pub fn board(&self) -> Vec<cell::Cell<char>> {
        let bg_col = 233;
        let head_col = 21;
        let body_col = 32;
        let body = self.snake_body();
        (0..self.size.x * self.size.y)
            .map(|i| Point::new(modulo::modulo(i, self.size.x), i / self.size.y))
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
    pub fn score(&self) -> i32 {
        self.score
    }
    fn spawn_fruit(&mut self) {
        self.fruit = (point::random_point(&self.size),
                      fruit::get_random_fruit());
    }
    fn snake_body(&self) -> Vec<Point<isize>> {
        let mut pos = self.snake.0.clone();
        self.snake.1
            .body
            .iter()
            .map(|dir| {
                     point::move_point(dir, &mut pos, &self.size);
                     pos.clone()
                 })
            .collect::<Vec<_>>()
    }
}

