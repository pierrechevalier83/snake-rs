extern crate rand;

use std::time::Duration;
use std::time::Instant;

#[derive(Clone)]
pub struct Fruit {
    pub symbol: char,
    color: u8,
    expiration_date: Duration,
    created: Instant,
}

fn fruit_color(symbol: &char) -> u8 {
    match *symbol {
        '🍏' => 47, // green apple
        '🍎' => 88, // red apple
        '🍐' => 36, // pale green pear
        '🍑' => 179, // pale orange peach
        '🍒' => 169, // bright red cherry
        '🍋' => 118, // yellow lemon
        '🍉' => 9, // orange orange
        '🍓' => 1, // red strawberry
        '🍇' => 54, // purple grape
        '🍈' => 9, // orange orange
        '🍍' => 191, // yellow pineaple
        _ => 0,
    }
}

fn fruit_expiration_date(symbol: &char) -> Duration {
    match *symbol {
        '🍏' => Duration::from_millis(5000),
        '🍎' => Duration::from_millis(4750),
        '🍐' => Duration::from_millis(4500),
        '🍑' => Duration::from_millis(4250),
        '🍒' => Duration::from_millis(4000),
        '🍋' => Duration::from_millis(3750),
        '🍉' => Duration::from_millis(3500),
        '🍓' => Duration::from_millis(3250),
        '🍇' => Duration::from_millis(3000),
        '🍈' => Duration::from_millis(2750),
        '🍍' => Duration::from_millis(2500),
        _ => Duration::from_millis(0),
    }
}

fn fruit_score_value(symbol: &char) -> i32 {
    match *symbol {
        '🍏' => 1,
        '🍎' => 2,
        '🍐' => 3,
        '🍑' => 4,
        '🍒' => 5,
        '🍋' => 6,
        '🍉' => 7,
        '🍓' => 8,
        '🍇' => 9,
        '🍈' => 10,
        '🍍' => 11,
        _ => 0,
    }
}

impl Fruit {
    fn new(symbol: &char) -> Fruit {
        Fruit {
            symbol: symbol.clone(),
            color: fruit_color(symbol),
            expiration_date: fruit_expiration_date(symbol),
            created: Instant::now(),
        }
    }
    pub fn rotten(&self) -> bool {
        Instant::now().duration_since(self.created) > self.expiration_date
    }
    pub fn score_value(&self) -> i32 {
        fruit_score_value(&self.symbol)
    }
    pub fn color(&self) -> u8 {
        if self.ripe() {
            rand::random::<u8>()
        } else {
            self.color
        }
    }
    fn ripe(&self) -> bool {
        Instant::now().duration_since(self.created) + Duration::from_secs(1) > self.expiration_date
    }
}

pub fn get_random_fruit() -> Fruit {
    let symbols = vec!['🍇', '🍈', '🍉', '🍋', '🍍', '🍎', '🍏', '🍐', '🍑',
                       '🍒', '🍓'];
    use rand::Rng;
    rand::thread_rng()
        .choose(&symbols
                     .iter()
                     .map(|x| Fruit::new(x))
                     .collect::<Vec<_>>())
        .unwrap()
        .clone()
}
