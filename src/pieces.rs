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
    pub fn move_patterns(&self) -> Vec<MovePattern>
    {
        self.piece_type.move_patterns(self.position)
    }
}

pub enum MovePattern
{
    Simple(Point),
    InfiniteLine(Point), //point here is move step and direction e.g (0, 1) means infinite up
    //    EnPassent(Point),
    //Castling(Point),
}

#[derive(Clone, PartialEq, Hash, Eq, Debug, Copy)]
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
    Pawn(PieceColor),
    Rook(PieceColor),
    Knight(PieceColor),
    Bishop(PieceColor),
    Queen(PieceColor),
    King(PieceColor),
}

impl PieceType
{


    pub fn get_color(&self) -> PieceColor
    {
        match self
        {
            PieceType::Pawn(color) => *color, //pretty shitty that i forced to do something like this. but for now its fine
            PieceType::Rook(color) => *color,
            PieceType::Knight(color) => *color,
            PieceType::Bishop(color) => *color,
            PieceType::Queen(color) => *color,
            PieceType::King(color) => *color
        }
    }
    fn move_patterns(&self, pos:Point) -> Vec<MovePattern>
    //TODO: Make this compile time?
    //this function generates vector of  possible moves. They are relative to piece origin and they are ignoring color
    {
        match self
        {
            PieceType::Pawn(PieceColor::Black) if  pos.y==6 => //FIXME: This should have better solution
            {
                vec![MovePattern::Simple(Point::new(0, 1)),
                     MovePattern::Simple(Point::new(0, 2))]
            }
            PieceType::Pawn(PieceColor::White) if  pos.y==1 =>
            {
                vec![MovePattern::Simple(Point::new(0, 1)),
                     MovePattern::Simple(Point::new(0, 2))]
            },
            PieceType::Pawn(_) =>
            {
                vec![MovePattern::Simple(Point::new(0, 1))]
            },

            PieceType::Rook(_) => {
                vec!
                [
                    MovePattern::InfiniteLine(Point::new(0,  1)), //up
                    MovePattern::InfiniteLine(Point::new(1,  0)), //right
                    MovePattern::InfiniteLine(Point::new(0, -1)), //down
                    MovePattern::InfiniteLine(Point::new(-1, 0)), //left
                ]
            }
            PieceType::Bishop(_) =>
            {
                vec!
                    [
                        MovePattern::InfiniteLine(Point::new(1,   1)),
                        MovePattern::InfiniteLine(Point::new(1,  -1)),
                        MovePattern::InfiniteLine(Point::new(-1,  1)),
                        MovePattern::InfiniteLine(Point::new(-1, -1)),
                    ]
            }
            PieceType::King(_) =>
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
            PieceType::Queen(_) =>
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
            PieceType::Knight(_) =>
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
