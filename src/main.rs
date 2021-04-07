mod pieces;
mod board;
mod drawing;
use board::Board;
use drawing::*;












fn main() {
    let board = Board::from_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", 8, 8);
    let mut draw = PixelBoard::new(board, 800, 600, "Chess");
    while !draw.should_close() {
        println!("{:?}", draw.get_mouse_pos());
        draw.draw_board(0, 0);
    }

}


//TODO:
//RAYLIB_NOTES:
//Do not make function that take raylibHandle and raylibDraw, make only functions that takes one of those
