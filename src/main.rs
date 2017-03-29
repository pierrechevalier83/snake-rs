extern crate matrix_display;
use matrix_display::*;

fn main() {
    let format = Format::new(3, 1);
    let n_cols = 22 as usize;
    let board = (0..n_cols * n_cols).map(|_| cell::Cell::new(" ", 0, 16)).collect::<Vec<_>>();
    let data = matrix::Matrix::new(n_cols, board);
    let display = MatrixDisplay::new(format, data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::None);
}
