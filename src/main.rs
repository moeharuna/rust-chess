mod pieces;
mod board;
mod drawing;
use board::Board;
use drawing::*;

fn main() {
    let board = Board::from_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", 8, 8);
    let mut draw = PixelBoard::new(board, 800, 600, "Chess");
    while !draw.should_close() {
        draw.tick()
    }
}
