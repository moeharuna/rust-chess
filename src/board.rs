use super::pieces::*;
use std::convert::TryInto;

pub struct Board
{
    pub pieces:Vec<Piece>,
    width:u32,
    height:u32,
    active_color:PieceColor
}
impl Board
{
    pub fn board_size(&self) -> (u32, u32)
    {
        (self.width, self.height)
    }
    pub fn from_fen_string(string:&str, width:u32, height:u32) ->Board
    {
        let pieces = Board::parse_fen_string(string);
        Board{
            pieces,
            width,
            height,
            active_color:PieceColor::White
        }
    }
    fn parse_fen_string(string:&str) -> Vec<Piece> //TODO: change type later
    {
        fn parse_placement(placement:&str) -> Vec<Piece>
        {
            fn fen_char_to_piece_type(ch:char) -> PieceType
            {
                let color = if ch.is_uppercase() {PieceColor::Black}
                else {PieceColor::White};
                match ch.to_ascii_lowercase()
                {
                    'p' => PieceType::Pawn(color),
                    'r' => PieceType::Rook(color),
                    'b' => PieceType::Bishop(color),
                    'n' => PieceType::Knight(color),
                    'q' => PieceType::Queen(color),
                    'k' => PieceType::King(color),
                    _ => panic!("Fen string error! Proper error handling not implemeted, sorry"),
                }
            }
            let vec:Vec<&str> = placement.split('/').collect();
            let mut pieces_vector:Vec<Piece> = vec![];


            for (i, string) in vec.iter().enumerate()
            {
                let mut current_x:i32 = 0;
                for (_j, ch) in string.chars().enumerate()
                {
                    if ch.is_ascii_digit() {current_x+=ch.to_digit(10).unwrap() as i32} //FIX THIS SHIT IT WONT WORK WITH BOARD SIZE > 9
                    else {pieces_vector.push(Piece::new(Point::new(current_x, i.try_into().unwrap()), fen_char_to_piece_type(ch)));}
                    current_x+=1;
                }
            }
            return pieces_vector;
        }

        let vec:Vec<&str> = string.split(' ').collect();
        parse_placement(vec[0])
        //TODO:    let active_color = vec[1];
        //TODO:    let castling = vec[2];
        //TODO:    let en_passant  = vec[3];
    }
    fn move_piece(&self, selected_piece:&mut Piece, move_to:Point)
    {
        //let moves = selected_piece.all_moves(self.width, self.height);
        selected_piece.set_position(move_to);
    }
    pub fn get_piece_on_cell(&mut self, cell:Point) -> Option<&mut Piece>
    {
        for i in self.pieces.iter_mut()
        {
            if i.position == cell{
                return Some(i);
            }
        }
        None
    }
}
