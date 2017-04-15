extern crate rand;

use std::time::Duration;

#[derive(Clone)]
pub struct Fruit {
    pub symbol: char,
    pub color: u8,
    pub lifetime: Duration,
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

fn fruit_lifetime(symbol: &char) -> Duration {
    match *symbol {
        '🍏' => Duration::from_millis(5000),
        '🍎' => Duration::from_millis(4500),
        '🍐' => Duration::from_millis(4000),
        '🍑' => Duration::from_millis(3500),
        '🍒' => Duration::from_millis(3000),
        '🍋' => Duration::from_millis(2500),
        '🍉' => Duration::from_millis(2000),
        '🍓' => Duration::from_millis(1500),
        '🍇' => Duration::from_millis(1000),
        '🍈' => Duration::from_millis(800),
        '🍍' => Duration::from_millis(500),
        _ => Duration::from_millis(0),
    }
}

impl Fruit {
    fn new(symbol: &char) -> Fruit {
        Fruit {
            symbol: symbol.clone(),
            color: fruit_color(symbol),
            lifetime: fruit_lifetime(symbol),
        }
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
