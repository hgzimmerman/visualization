use common::l_system::Grammar;
use nannou::geom::Point2;

///
/// Angle should be 120 deg
#[derive(Clone, Copy, Debug)]
pub enum Koch {
    F,
    Plus,
    Minus
}


impl Grammar for Koch {
    type Item = Point2;

    fn production_rules(self) -> Vec<Self> {
        use Koch::*;
        match self {
            F => vec![F, Plus, F, Minus, Minus, F, Plus, F],
            Plus => vec![Plus],
            Minus => vec![Minus]
        }
    }

    fn reify(v: &Self, current_pt: &mut Self::Item, current_angle: &mut f32, angle_step: f32, line_length: f32) -> Option<Self::Item>
    {
        use Koch::*;
        let cpy = current_pt.clone();
        match v {
            F => *current_pt = line_to(*current_pt, *current_angle, line_length),
            Plus => *current_angle += angle_step,
            Minus => *current_angle -= angle_step,
        };
        match v {
            F => Some(cpy),
            Plus | Minus => None
        }
    }
}


/// Produces a new point from the current one
#[inline]
fn line_to(pt: Point2, angle: f32, line_length: f32) -> Point2 {
    Point2 {
        x: pt.x + line_length * angle.cos(),
        y: pt.y + line_length * angle.sin()
    }
}