use common::l_system::Grammar;
use common::point_ext::line_to;
use nannou::geom::Point2;

///
/// Angle should be 120 deg
#[derive(Clone, Copy, Debug)]
pub enum TerDragon {
    F,
    Plus,
    Minus
}


impl Grammar for TerDragon {
    type Item = Point2;

    fn production_rules(self) -> Vec<Self> {
        use TerDragon::*;
        match self {
            F => vec![F, Plus, F, Minus, F],
            Plus => vec![Plus],
            Minus => vec![Minus]
        }
    }

    fn reify(v: &Self, current_pt: &mut Self::Item, current_angle: &mut f32, angle_step: f32, line_length: f32) -> Option<Self::Item>
    {
        use TerDragon::*;
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


