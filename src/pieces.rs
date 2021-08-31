use std::error::Error;

use raylib::ffi::PFNGLDRAWRANGEELEMENTARRAYATIPROC;
use raylib::math::Vector2;
use crate::math::*;

use crate::Board;
///This struct is how i store data about one square.
#[derive(Clone,Copy, Debug, PartialEq, Eq)]
pub struct Square
{
    pub x:i32,
    pub y:i32,
}
impl Square
{
    pub fn new(x:i32, y:i32) -> Square
    {
        Square{x, y}
    }
}

impl std::ops::Add for Square
{
    type  Output=Self;
    fn add(self, other:Self) -> Self::Output
    {
        Self {
            x: self.x+other.x,
            y: self.y+other.y,
        }
    }
}
impl std::ops::Sub for Square
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self
        {
            x: self.x-rhs.x,
            y: self.y-rhs.y,
        }
    }
}




#[derive(Clone, Copy)]
pub struct Ray
{
    start:Option<Square>,
    end:Option<Square>,
    step:Square,
}

impl Ray
{
    pub fn new(step:Square) -> Ray
    {
        Ray{start:None, end:None, step}
    }

    pub fn resolve(&mut self, host:&Piece, board:&Board) //TODO:rename
    {

        let first_piece_on_the_way_or_border = |&step|  -> Square
        {
            let mut end_of_line =  host.position+step;
            while  board.is_square_on_board(&end_of_line) {
                match board.get_piece(&end_of_line)
                {
                    Some(piece) if piece.kind.color()==host.kind.color() =>
                    {
                        end_of_line= end_of_line - step;
                        break;
                    },
                    Some(_) => break,
                    _ =>  end_of_line= end_of_line + step,
                }
            }
            end_of_line
        };


        self.start = Some(host.position+self.step);
        self.end = Some(first_piece_on_the_way_or_border(&self.step))
    }

    fn panic_if_unresolved(&self)
    {
        if !(self.start.is_some() && self.end.is_some()) {
            panic!("Trying to use unresolved Ray!")
        }
    }

    pub fn to_squares(&self) -> Vec<Square>
    {
        self.panic_if_unresolved();
        let mut result:Vec<Square> = Vec::new();
        let mut square = self.start.unwrap();
        while square!= self.end.unwrap() + self.step
        {
            result.push(square);
            square = square+self.step;
        }
        result
    }
    pub fn is_square_on_ray(&self, square:Square)
    {
        self.panic_if_unresolved();
        is_point_on_line(square, (self.start.unwrap(), self.end.unwrap()));
    }
}
///This enum is how i store data about abstract move that yet to be generated.
#[derive(Clone)]
pub enum MovePattern
{
    Simple(Square),
    LineMove(Ray),
    //    EnPassent(Point), //maybe?
    //Castling(Point), //maybe?
}


#[derive(Clone, PartialEq, Hash, Eq, Debug, Copy)]
pub enum PieceColor
{
    Black,
    White,
}


impl std::ops::Not for PieceColor
{
    type Output = Self;
    fn not(self) -> Self::Output
    {
        match self
        {
            PieceColor::Black => PieceColor::White,
            PieceColor::White => PieceColor::Black,
        }
    }
}
impl Default for PieceColor
{
    fn default() -> Self
    {
        PieceColor::Black
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum PieceType
{
    Pawn(PieceColor),
    Rook(PieceColor),
    Knight(PieceColor),
    Bishop(PieceColor),
    Queen(PieceColor),
    King(PieceColor),
}

impl PieceType
{


    pub fn color(&self) -> PieceColor
    {
        match self
        {
            PieceType::Pawn(color) => *color, //bad code, but as far as i know it's the only way to extract enum value probably should refactor pieceType into trait but it's too much work :/
            PieceType::Rook(color) => *color,
            PieceType::Knight(color) => *color,
            PieceType::Bishop(color) => *color,
            PieceType::Queen(color) => *color,
            PieceType::King(color) => *color
        }
    }

    fn is_double_square_move_allowed(&self, pos:Square) -> bool
    {
        match self
        {
            PieceType::Pawn(color) =>
            {
                if  (*color==PieceColor::Black && pos.y==6)
                 || (*color==PieceColor::White && pos.y==1) {
                     return true
                }
                else {return false}
            }
            _ => false
        }
    }
    fn move_patterns(&self, pos:Square) -> Vec<MovePattern>
    //this function generates vector of  possible moves. They are relative to piece origin and they are ignoring color(black pieces will reverse their y later)
    {
        match self
        {
            PieceType::Pawn(_) if self.is_double_square_move_allowed(pos) =>
            {
                vec![MovePattern::Simple(Square::new(0, 1)),
                     MovePattern::Simple(Square::new(0, 2))]
            }
            PieceType::Pawn(_) =>
            {
                vec![MovePattern::Simple(Square::new(0, 1))]
            },

            PieceType::Rook(_) => {
                vec! [
                    MovePattern::LineMove(Ray::new(Square::new(0,  1))), //up
                    MovePattern::LineMove(Ray::new(Square::new(1,  0))), //right
                    MovePattern::LineMove(Ray::new(Square::new(0, -1))), //down
                    MovePattern::LineMove(Ray::new(Square::new(-1, 0))), //left
                ]
            }
            PieceType::Bishop(_) =>
            {
                vec![
                    MovePattern::LineMove(Ray::new(Square::new(1,   1))),
                    MovePattern::LineMove(Ray::new(Square::new(1,  -1))),
                    MovePattern::LineMove(Ray::new(Square::new(-1,  1))),
                    MovePattern::LineMove(Ray::new(Square::new(-1, -1))),
                    ]
            }
            PieceType::King(_) =>
            {
                vec![
                    MovePattern::Simple(Square::new(0,1)),
                    MovePattern::Simple(Square::new(1,0)),
                    MovePattern::Simple(Square::new(1,1)),

                    MovePattern::Simple(Square::new(0, -1)),
                    MovePattern::Simple(Square::new(-1, 0)),
                    MovePattern::Simple(Square::new(-1,-1)),

                    MovePattern::Simple(Square::new(-1,1)),
                    MovePattern::Simple(Square::new(1, 1)),
                ]
            }
            PieceType::Queen(_) =>
            {
                vec![
                    MovePattern::LineMove(Ray::new(Square::new(0,  1))), //up
                    MovePattern::LineMove(Ray::new(Square::new(1,  0))), //right
                    MovePattern::LineMove(Ray::new(Square::new(0, -1))), //down
                    MovePattern::LineMove(Ray::new(Square::new(-1, 0))), //left

                    MovePattern::LineMove(Ray::new(Square::new(1,  1))),
                    MovePattern::LineMove(Ray::new(Square::new(1, -1))),
                    MovePattern::LineMove(Ray::new(Square::new(-1, 1))),
                    MovePattern::LineMove(Ray::new(Square::new(-1,-1))),
                ]
            }
            PieceType::Knight(_) =>
            {
                vec![
                    MovePattern::Simple(Square::new(1,  2)),
                    MovePattern::Simple(Square::new(1, -2)),
                    MovePattern::Simple(Square::new(-1, 2)),
                    MovePattern::Simple(Square::new(-1,-2)),

                    MovePattern::Simple(Square::new(2,  1)),
                    MovePattern::Simple(Square::new(2, -1)),
                    MovePattern::Simple(Square::new(-2, 1)),
                    MovePattern::Simple(Square::new(-2,-1)),
                ]
            }
        }
    }
}



#[derive(Debug)]
pub struct Piece
{
    pub position:Square,
    pub kind:PieceType, //its should be called "type" but "type" is reserved keyword
}

impl Piece
{
    pub fn new(position:Square, piece_type:PieceType) -> Piece
    {
        Piece{position, kind: piece_type}
    }


    pub fn set_position(&mut self, point:&Square)
    {
        self.position = *point;
    }

    pub fn checked_move_list(&self, board:&Board) -> Vec<Square>
    {
        use PieceType::*;
        if let King(_) = self.kind {
                self.move_list(board)
                    .into_iter()
                    .filter(|square| board.is_square_safe(square, !self.kind.color()))
                    .collect()
            }
        else {
            self.move_list(board)
                .into_iter()
                .filter(|square| self.will_move_save_king(board, square))
                .collect()
        }
    }

    fn will_move_save_king(&self, _board:&Board, _square:&Square) -> bool
    {
        false
    }


    pub fn move_list(&self, board:&Board) ->  Vec<Square>
    {
        self.kind.move_patterns(self.position)
            .into_iter()
            .map(|pattern| self.pattern2squares(pattern, board))
            .flatten()
            .collect()
    }


    fn pattern2squares(&self,
                       pattern: MovePattern,
                       board:&Board) -> Vec<Square>
    {
        use MovePattern::*;
        use PieceType::*;
        use PieceColor::*;


        let get_end_square = |square:Square| -> Square
        {
            if self.kind.color() == Black {
                return self.position-square
            }
            self.position+square
        };
        match pattern
        {
            Simple(square) =>
            {
                let result_square = get_end_square(square);


                if let Pawn(_) = self.kind //FIXME: Horrible solution for pawn capturing, idk how to make it better
                {
                    let left = Square::new(result_square.x-1, result_square.y);
                    let right = Square::new(result_square.x+1, result_square.y);

                    let mut result_vec = vec![];

                    if let None = board.get_piece(&result_square)
                    {
                        result_vec.push(result_square);
                    }
                    if matches!(board.get_piece(&left), Some(piece) if piece.kind.color()!=self.kind.color())
                    {
                        result_vec.push(left);
                    }
                    if matches!(board.get_piece(&right), Some(piece) if piece.kind.color()!=self.kind.color())
                    {
                        result_vec.push(right);
                    }
                    return result_vec
                }

                if matches!(board.get_piece(&result_square), Some(piece) if piece.kind.color()==self.kind.color())
                {
                    return vec![];
                }
                vec![result_square]
            },
            LineMove(mut ray) => {

                ray.resolve(self, board);
                ray.to_squares()
            }
        }
    }
}
