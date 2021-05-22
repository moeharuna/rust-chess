use std::iter::Empty;

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

///This struct is how i store data about already generated move. If generated move is only one square start and end points should be the same
#[derive(Clone, Debug)]
pub struct MoveRay //FIXME:rename
{
    pub start:Square,
    pub end:Square,
}
impl MoveRay
{
    pub fn new(start:Square, end:Square) -> MoveRay
    {
        MoveRay{start, end}
    }
    pub fn one_square(square:Square) -> MoveRay
    {
        MoveRay{start:square, end:square}
    }
}

fn ray2square(ray:&MoveRay) -> Vec<Square>
{
    if ray.start == ray.end
    {
        return vec![ray.start]
    }
    let mut square = ray.start;
    let mut result:Vec<Square> = vec![];
    let direction = ray.end-ray.start;
    let step = Square{x: (direction.x>0) as i32 - (direction.x<0) as i32,
                      y: (direction.y>0) as i32 - (direction.y<0) as i32};
    //print!("start:{:?}, end:{:?}, direction:{:?}, step:{:?}\n", ray.start, ray.end, direction, step);
    loop
    {
        if square.x==ray.end.x && square.y==ray.end.y
        {
            break;
        }
        result.push(square);
        square = square + step;
    }
    //print!("vector: {:?}\n", result);

    result
}

///This enum is how i store data about abstract move that yet to be generated.
pub enum MovePattern
{
    Simple(Square),
    InfiniteLine(Square), //point here is move step and direction e.g. (0, 1) means up by one
    //    EnPassent(Point), //maybe?
    //Castling(Point), //maybe?
}



#[derive(Clone, Debug)]
pub struct Piece
{
    pub position:Square,
    pub piece_type:PieceType,
    pub moves_list:Vec<MoveRay>,
}

impl Piece
{
    pub fn new(position:Square, piece_type:PieceType, moves_list:Vec<MoveRay>) -> Piece
    {
        Piece{position, piece_type, moves_list}
    }
    pub fn set_position(&mut self, point:&Square)
    {
        self.position = *point;
    }
    pub fn move_patterns(&self) -> Vec<MovePattern>
    {
        self.piece_type.move_patterns(self.position)
    }
    pub fn move_squares(&self) -> Vec<Square>
    {
        self.moves_list.iter().fold(vec![], |acc, ray| [&acc[..], &ray2square(ray)[..]].concat())
    }
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
    fn move_patterns(&self, pos:Square) -> Vec<MovePattern>
    //TODO: Make this compile time?
    //this function generates vector of  possible moves. They are relative to piece origin and they are ignoring color
    {
        match self
        {
            PieceType::Pawn(PieceColor::Black) if  pos.y==6 => //FIXME: This should have better solution
            {
                vec![MovePattern::Simple(Square::new(0, 1)),
                     MovePattern::Simple(Square::new(0, 2))]
            }
            PieceType::Pawn(PieceColor::White) if  pos.y==1 =>
            {
                vec![MovePattern::Simple(Square::new(0, 1)),
                     MovePattern::Simple(Square::new(0, 2))]
            },
            PieceType::Pawn(_) =>
            {
                vec![MovePattern::Simple(Square::new(0, 1))]
            },

            PieceType::Rook(_) => {
                vec!
                [
                    MovePattern::InfiniteLine(Square::new(0,  1)), //up
                    MovePattern::InfiniteLine(Square::new(1,  0)), //right
                    MovePattern::InfiniteLine(Square::new(0, -1)), //down
                    MovePattern::InfiniteLine(Square::new(-1, 0)), //left
                ]
            }
            PieceType::Bishop(_) =>
            {
                vec!
                    [
                        MovePattern::InfiniteLine(Square::new(1,   1)),
                        MovePattern::InfiniteLine(Square::new(1,  -1)),
                        MovePattern::InfiniteLine(Square::new(-1,  1)),
                        MovePattern::InfiniteLine(Square::new(-1, -1)),
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
                    MovePattern::InfiniteLine(Square::new(0,  1)), //up
                    MovePattern::InfiniteLine(Square::new(1,  0)), //right
                    MovePattern::InfiniteLine(Square::new(0, -1)), //down
                    MovePattern::InfiniteLine(Square::new(-1, 0)), //left

                    MovePattern::InfiniteLine(Square::new(1,  1)),
                    MovePattern::InfiniteLine(Square::new(1, -1)),
                    MovePattern::InfiniteLine(Square::new(-1, 1)),
                    MovePattern::InfiniteLine(Square::new(-1,-1)),
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
