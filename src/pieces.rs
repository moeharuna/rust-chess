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

#[derive(Clone)]
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
    pub fn set_position(&mut self, point:Point)
    {
        self.position = point;
    }
    pub fn all_moves(&self, board_width:u32, board_height:u32) ->  Vec<Point> //its not good that i made function here that requiers board_width & height,
    {
        let board_width = board_width as i32;
        let board_height =board_height as i32;
        for pattern in self.piece_type.move_patterns()
            {
                match pattern
                {
                    MovePattern::Simple(p) => vec![p+self.position],
                    MovePattern::InfiniteLine(step) => {
                        let mut result: Vec<Point> = Vec::new();
                        let mut cell = step.clone();
                        while cell.x < board_width && cell.y < board_height &&
                              cell.x > 0           && cell.y > 0
                        {
                            result.push(cell.clone());
                            cell = cell+step;
                        }
                        result
                    },
                };
            }
        vec![]
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
    Pawn(PieceColor),
    Rook(PieceColor),
    Knight(PieceColor),
    Bishop(PieceColor),
    Queen(PieceColor),
    King(PieceColor),
}

impl PieceType
{
    pub fn move_patterns(&self) -> Vec<MovePattern> //this function generates vector of  possible moves. They are relative to piece origin
    {
        match self
        {
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
