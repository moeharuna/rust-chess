

//Im not sure if I even need this file, but i did it anyway.
use crate::pieces::Square;
use raylib::math::Vector2;

type Point = Square;
fn from_two_points(a:Point, b:Point) -> Vector2
{
    Vector2::new((b.x-a.x) as f32, (b.y-a.y) as f32)
}

fn cross_product(vector1:Vector2, vector2:Vector2) -> f32
{
//    determinant(Matrix(vector1.x, vector2.x,
//                       vector1.y, vector2.y))
        vector1.x*vector2.y-vector2.x*vector1.y
}

fn f32eq(a:f32, b:f32) -> bool
{
    (a-b).abs() <= f32::EPSILON
}


pub fn is_point_on_line(point:Point, line:(Point, Point)) -> bool
{
    let ab = from_two_points(line.0, line.1);
    let ac = from_two_points(line.0, point);

    let colliniar = cross_product(ab, ac)==0.0;
    if !colliniar {
        return false
    }
    let dot_ab = ab.dot(ab);
    let dot_ac = ab.dot(ac);


    if f32eq(dot_ac, 0.0) ||      //point==line.start
       f32eq(dot_ac, dot_ab) || //ab==ac, so point==line.end
        (dot_ac > 0.0 && dot_ac < dot_ab) { //point lies between start and end
        return true
    }

    true
}
