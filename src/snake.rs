use direction::Direction;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Snake {
    pub body: VecDeque<Direction>,
}

impl Snake {
    pub fn new() -> Snake {
        Snake { body: vec![Direction::Left; 3].into_iter().collect() }
    }
    pub fn crawl(&mut self, direction: &Direction) {
        self.grow(direction);
        self.body.pop_back();
    }
    pub fn grow(&mut self, direction: &Direction) {
        self.body.push_front(::direction::opposite(direction));
    }
    pub fn direction(&self) -> Direction {
        ::direction::opposite(self.body.front().unwrap())
    }
}
