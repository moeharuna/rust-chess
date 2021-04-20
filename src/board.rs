use super::pieces::*;
use std::convert::TryInto;
pub struct Board
{
    pub pieces:Vec<Piece>,
    width:u32,
    height:u32,
    pub selected_piece:Option<usize>,
    current_player_color:PieceColor
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
            selected_piece:None,
            current_player_color:PieceColor::White
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
                    'p' => PieceType::Pawn{color},
                    'r' => PieceType::Rook{color},
                    'b' => PieceType::Bishop{color},
                    'n' => PieceType::Knight{color},
                    'q' => PieceType::Queen{color},
                    'k' => PieceType::King{color},
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
            pieces_vector
        }

        let vec:Vec<&str> = string.split(' ').collect();
        parse_placement(vec[0])
        //TODO:    let active_color = vec[1];
        //TODO:    let castling = vec[2];
        //TODO:    let en_passant  = vec[3];
    }
    pub fn move_piece(&mut self,  move_to:&Point)
    {
        //let moves = selected_piece.all_moves(self.width, self.height);
        match  self.selected_piece
        {
            Some(val) => self.pieces[val].set_position(move_to),
            None => panic!("trying to move unexisting piece"),
        }
    }

    pub fn possible_moves(&self) ->  Vec<Point>
    {
        match self.selected()
        {
            None => vec![],
            Some(piece) =>
            {
                for pattern in piece.piece_type.move_patterns()
                {
                    //print!("pattern");
                    match pattern
                    {
                        MovePattern::Simple(p) =>  {
                            //let mut p = p;
                            // if piece.piece_type.color==PieceColor::Black
                            // {
                            //     position.y = -p.y
                            //}

                            vec![p+piece.position]
                        },
                        MovePattern::InfiniteLine(step) => {
                            let mut result: Vec<Point> = Vec::new();
                            let mut cell = step;
                            while cell.x < self.width as i32 && cell.y < self.height as i32 &&
                                cell.x > 0           && cell.y > 0
                            {
                                result.push(cell);
                                cell = cell+step;
                            }
                            result
                        },
                    };
                }
                vec![]
            }
        }
    }

    pub fn selected(&self) -> Option<&Piece>
    {
        Some(&self.pieces[self.selected_piece?])
    }

    pub fn select_piece_by_pos(&mut self, pos:&Point)
    {
        self.selected_piece = self.pieces.iter().position(|val:&Piece| val.position==*pos);
    }
}
