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

fn main() {
    let format = Format::new(3, 1);
    let board = Board::new(22);
    let data = matrix::Matrix::new(board.n_cols(), board.data());
    let display = MatrixDisplay::new(format, data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::None);
}
