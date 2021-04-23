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

    pub fn get_piece(&self, square:&Point) -> Option<&Piece>
    {
        self.pieces.iter().find(|val:&&Piece| val.position==*square)
    }
    pub fn remove_piece(&mut self, square:&Point) -> bool //TODO: Validate selected_piece after remove
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

    pub fn selected(&self) -> Option<&Piece>
    {
        print!("selected: {:?}\n", self.pieces[self.selected_piece?]);
        Some(&self.pieces[self.selected_piece?])
    }

    pub fn select_piece_by_pos(&mut self, pos:&Point)
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

    pub fn move_piece(&mut self,  move_to:&Point)
    {
        if self.selected_piece.is_none()
        {
            return
        }

        self.remove_piece(move_to);
        let val = self.selected_piece.unwrap(); //make sure that right piece selected
        self.pieces[val].set_position(move_to);
        self.selected_piece = None;
        self.change_player();
    }

    //TODO: implement some castling
    //TODO: implement pawn killing diagnally
    //TODO: implement en_passant
    pub fn possible_moves(&self) ->  Vec<Point> //i really don't like how this function is implemented, its should be much smaller
    {
        match self.selected()
        {
            None => vec![],
            Some(piece) =>
            {
                let mut result:Vec<Point> = Vec::new();
                for pattern in piece.move_patterns()
                {
                    match pattern
                    {
                        MovePattern::Simple(p) =>  {
                            let mut p = p;
                            if piece.piece_type.get_color()==PieceColor::Black
                            {
                                p.y = -p.y
                            }
                            let m = p+piece.position;
                            match self.get_piece(&m)
                            {
                                None => result.push(p+piece.position),
                                Some(piece_on_square) =>
                                    if piece_on_square.piece_type.get_color()!=piece.piece_type.get_color() {
                                        result.push(p+piece.position)
                                    }
                            }
                        },
                        MovePattern::InfiniteLine(step) => {
                            let mut square = piece.position;
                            while (square.x < self.width as i32 && square.y < self.height as i32) &&
                                  (square.x > -1 && square.y > -1)
                            {
                                square = square+step;
                                if let Some(piece_on_square) = self.get_piece(&square)
                                {
                                    if piece_on_square.piece_type.get_color()!=piece.piece_type.get_color()
                                    {
                                        result.push(square)
                                    }
                                    break;
                                }
                                result.push(square);
                            }
                        },
                    }
                }
                result
            }
        }
    }
}
