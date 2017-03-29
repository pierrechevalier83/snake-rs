extern crate matrix_display;
use matrix_display::*;

struct Board {
    n_cols: usize,
}

impl Board {
    fn new(size: usize) -> Board {
	    Board {
		    n_cols: size,
		}
	}
	fn n_cols(&self) -> usize {
	    self.n_cols
	}
	fn data(&self) -> Vec<matrix_display::cell::Cell<char>> {
        (0..self.n_cols * self.n_cols).map(|_| cell::Cell::new(' ', 0, 16)).collect::<Vec<_>>()
	}
}

#[derive(Clone)]
enum Direction {
    up,
	down,
	left,
	right,
}

struct Snake {
    heads_index: usize,
	body: Vec<Direction>,
}

impl Snake {
    fn new(size: usize) -> Snake {
	    Snake {
		    heads_index: 0,
			body: vec![Direction::left; size],
		}
	}
}

fn main() {
    let format = Format::new(3, 1);
    let board = Board::new(22);
	let snake = Snake::new(4);
    let data = matrix::Matrix::new(board.n_cols(), board.data());
    let display = MatrixDisplay::new(format, data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::None);
}
