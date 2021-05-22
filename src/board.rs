use raylib::ffi::GL_COMPRESSED_SIGNED_RED_RGTC1;

use super::pieces::*;
use std::convert::TryInto;
pub struct Board
{
    pub pieces:Vec<Piece>,
    width:i32,
    height:i32,
    pub selected_piece:Option<usize>,
    current_player_color:PieceColor,
    is_checked:bool,
}
impl Board
{
    fn regenerate_moves(&mut self)
    {
        for i in 0..self.pieces.len()
        {
            self.pieces[i].moves_list = self.generate_moves(&self.pieces[i]);
        }
        print!("generated\n");
    }
    pub fn turn(&mut self)
    {
        self.regenerate_moves();
        if self.is_king_checked(!self.current_player_color)
        {
            print!("King is checked!\n");
        }
        self.selected_piece = None;
        self.change_player();
        print!("turn ended\n")
    }

    pub fn board_size(&self) -> (i32, i32)
    {
        (self.width, self.height)
    }
    pub fn from_fen_string(string:&str, width:i32, height:i32) ->Board
    {
        let pieces = Board::parse_fen_string(string);
        let mut result = Board{
            pieces,
            width,
            height,
            selected_piece:None,
            current_player_color:PieceColor::White,
            is_checked:false
        };
        result.regenerate_moves();
        result
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
                    else {pieces_vector.push(Piece::new(Square::new(current_x, i.try_into().unwrap()), fen_char_to_piece_type(ch), vec![]));}
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

    pub fn change_player(&mut self)
    {
        self.current_player_color = match self.current_player_color
        {
            PieceColor::Black => PieceColor::White,
            PieceColor::White => PieceColor::Black,
        }
    }

    pub fn get_piece(&self, square:&Square) -> Option<&Piece>
    {
        self.pieces.iter().find(|val:&&Piece| val.position==*square)
    }
    pub fn remove_piece(&mut self, square:&Square) -> bool //TODO: Validate selected_piece after remove
    {
        let remove_index = self.pieces.iter().position(|piece:&Piece| piece.position==*square);
        match remove_index
        {
            None => return false,
            Some(val) => self.pieces.remove(val),
        };
        let selected_index = match self.selected_piece
        {
            None => return true,
            Some(val) => val,
        };
        match remove_index.unwrap().cmp(&selected_index)
        {
            std::cmp::Ordering::Less =>   self.selected_piece= Some(selected_index-1),
            std::cmp::Ordering::Equal =>  self.selected_piece = None,
            std::cmp::Ordering::Greater => (),
        };
        true
    }

    fn get_king(&self, color:PieceColor) -> Option<&Piece>
    {
        self.pieces.iter().find(|piece| piece.piece_type==PieceType::King(color))
    }

    pub fn selected(&self) -> Option<&Piece>
    {
        Some(&self.pieces[self.selected_piece?])
    }

    pub fn is_king_checked(&self, color:PieceColor) -> bool //Should probably  throw some  error when king not found, but i don't really care about errors rn
    {
        let last_moved_piece = match self.selected()
        {
            None => return false,
            Some(val) => val
        };
        let king:&Piece = match self.get_king(color)
        {
            None => return false,
            Some(val) => val
        };
        if self.he_attacks(last_moved_piece, king)
        {
            return true
        }
        false
    }



    pub fn select_piece_by_pos(&mut self, pos:&Square)
    {
        let selected = match self.pieces.iter().position(|val:&Piece| val.position==*pos)
        {
            None => return,
            Some(val) => val,
        };
        if self.current_player_color == self.pieces[selected].piece_type.get_color()
        {
            self.selected_piece = self.pieces.iter().position(|val:&Piece| val.position==*pos);
        }
    }

    pub fn move_piece(&mut self,  move_to:&Square)
    {
        if self.selected_piece.is_none()
        {
            return
        }

        self.remove_piece(move_to);
        let val = self.selected_piece.unwrap(); //make sure that right piece selected
        self.pieces[val].set_position(move_to);
        self.turn();
        print!("move_piece_ended\n");

    }

    fn he_attacks(&self, attacker:&Piece, defender:&Piece) -> bool
    {
        attacker.moves_list.iter().any(|ray| ray.end==defender.position)
    }


    //TODO: implement castling
    //TODO: implement pawn killing diagnally
    //TODO: implement en_passant
    fn pattern2ray(&self, pattern:&MovePattern, piece:&Piece) -> Option<MoveRay>
    {
        let upper_bound = |step:Square|
        {
            let mut upper_bound = piece.position;
            while upper_bound.x < self.width && upper_bound.y < self.height &&
                  upper_bound.x > -1         && upper_bound.y > -1
            {
                let checked_square = upper_bound + step;
                match self.get_piece(&checked_square)
                {
                    Some(piece_on_square) => {
                        if piece_on_square.piece_type.get_color()!=piece.piece_type.get_color()
                        {
                            return checked_square+step
                        }
                        return checked_square
                    },
                    None => {
                        upper_bound = checked_square
                    },
                }
            }
            return upper_bound
        };


        let pos = piece.position;
        match pattern
        {
            MovePattern::Simple(p) =>
            {
                let mut p = *p;
                if piece.piece_type.get_color()==PieceColor::Black
                {
                    p.y = -p.y
                }
                let result_square = pos+p;
                match self.get_piece(&result_square)
                {
                    Some(val) if val.piece_type.get_color()==piece.piece_type.get_color() => None,
                    _ => Some(MoveRay::one_square(result_square)),
                }
            }
            MovePattern::InfiniteLine(p) =>
            {
                let p = *p;
                let ray_end = upper_bound(p)-p;
                if ray_end == piece.position
                {
                    return None
                }
                Some (MoveRay::new(pos+p, ray_end))
            }
        }
    }
    pub fn generate_moves(&self, piece:&Piece) ->  Vec<MoveRay>
    {
        piece.move_patterns().iter().filter_map(|pattern| self.pattern2ray(pattern, piece)).collect()
    }
}
