#[derive(Clone,Copy, Debug, PartialEq, Eq)]
pub struct Point
{
    pub x:i32,
    pub y:i32,
}
impl Point
{
    pub fn new(x:i32, y:i32) -> Point
    {
        Point{x, y}
    }
}

impl std::ops::Add for Point
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

#[derive(Clone, Debug)]
pub struct Piece
{
    pub position:Point,
    pub piece_type:PieceType,
}

impl Piece
{
    pub fn new(position:Point, piece_type:PieceType) -> Piece
    {
        Piece{position, piece_type}
    }
    pub fn set_position(&mut self, point:&Point)
    {
        self.position = *point;
    }
}

pub enum MovePattern
{
    Simple(Point),
    InfiniteLine(Point), //point here is move step and direction e.g (0, 1) means infinite up
    //    EnPassent(Point),
    //Castling(Point),
}

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub enum PieceColor
{
    Black,
    White,
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
    Pawn{color:PieceColor},
    Rook{color:PieceColor},
    Knight{color:PieceColor},
    Bishop{color:PieceColor},
    Queen{color:PieceColor},
    King{color:PieceColor},
}

impl PieceType
{
    pub fn move_patterns(&self) -> Vec<MovePattern> //TODO: Make this compile time?
    //this function generates vector of  possible moves. They are relative to piece origin and they are ignoring color
    {
        match self
        {
            PieceType::Pawn{..} =>
            {
                vec![MovePattern::Simple(Point::new(0, 1))]
            },
            PieceType::Rook{..} => {
                vec!
                [
                    MovePattern::InfiniteLine(Point::new(0,  1)), //up
                    MovePattern::InfiniteLine(Point::new(1,  0)), //right
                    MovePattern::InfiniteLine(Point::new(0, -1)), //down
                    MovePattern::InfiniteLine(Point::new(-1, 0)), //left
                ]
            }
            PieceType::Bishop{..} =>
            {
                vec!
                    [
                        MovePattern::InfiniteLine(Point::new(1,   1)),
                        MovePattern::InfiniteLine(Point::new(1,  -1)),
                        MovePattern::InfiniteLine(Point::new(-1,  1)),
                        MovePattern::InfiniteLine(Point::new(-1, -1)),
                    ]
            }
            PieceType::King{..} =>
            {
                vec![
                    MovePattern::Simple(Point::new(0,1)),
                    MovePattern::Simple(Point::new(1,0)),
                    MovePattern::Simple(Point::new(1,1)),

                    MovePattern::Simple(Point::new(0, -1)),
                    MovePattern::Simple(Point::new(-1, 0)),
                    MovePattern::Simple(Point::new(-1,-1)),

                    MovePattern::Simple(Point::new(-1,1)),
                    MovePattern::Simple(Point::new(1, 1)),
                ]
            }
            PieceType::Queen{..} =>
            {
                vec![
                    MovePattern::InfiniteLine(Point::new(0,  1)), //up
                    MovePattern::InfiniteLine(Point::new(1,  0)), //right
                    MovePattern::InfiniteLine(Point::new(0, -1)), //down
                    MovePattern::InfiniteLine(Point::new(-1, 0)), //left

                    MovePattern::InfiniteLine(Point::new(1,  1)),
                    MovePattern::InfiniteLine(Point::new(1, -1)),
                    MovePattern::InfiniteLine(Point::new(-1, 1)),
                    MovePattern::InfiniteLine(Point::new(-1,-1)),
                ]
            }
            PieceType::Knight{..} =>
            {
                vec![
                    MovePattern::Simple(Point::new(1,  2)),
                    MovePattern::Simple(Point::new(1, -2)),
                    MovePattern::Simple(Point::new(-1, 2)),
                    MovePattern::Simple(Point::new(-1,-2)),

                    MovePattern::Simple(Point::new(2,  1)),
                    MovePattern::Simple(Point::new(2, -1)),
                    MovePattern::Simple(Point::new(-2, 1)),
                    MovePattern::Simple(Point::new(-2,-1)),
                ]
            }
        }
    }
}
