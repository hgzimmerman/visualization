use common::l_system::Grammar;
use nannou::geom::Point2;
use common::point_ext::line_to;

///
/// Angle should be 90 deg
#[derive(Clone, Copy, Debug)]
pub enum Peano {
    F,
    L,
    R,
    Plus,
    Minus
}


impl Grammar for Peano {
    type Item = Point2;

    fn production_rules(self) -> Vec<Self> {
        use Peano::*;
        match self {
            F => vec![F],
            L => vec![L, F, R, F, L, Minus, F, Minus, R, F, L, F, R, Plus, F, Plus, L, F, R, F, L],
            R => vec![R, F, L, F, R, Plus, F, Plus, L, F, R, F, L, Minus, F, Minus, R, F, L, F, R],
            Plus => vec![Plus],
            Minus => vec![Minus]
        }
    }

    fn reify(v: &Self, current_pt: &mut Self::Item, current_angle: &mut f32, angle_step: f32, line_length: f32) -> Option<Self::Item>
    {
        use Peano::*;
        let cpy = current_pt.clone();
        match v {
            F => *current_pt = line_to(*current_pt, *current_angle, line_length),
            L | R => {},
            Plus => *current_angle += angle_step,
            Minus => *current_angle -= angle_step,
        };
        match v {
            F => Some(cpy),
            Plus | Minus | L | R => None
        }
    }
}

