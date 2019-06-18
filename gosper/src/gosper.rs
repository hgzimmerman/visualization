use common::l_system::Expandable;
use nannou::geom::Point2;

/// https://wikivisually.com/wiki/Gosper_curve
///
/// Alphabet for the grammar.
#[derive(Clone, Copy, Debug)]
pub enum Gosper {
    A,
    B,
    Plus,
    Minus
}


impl Expandable for Gosper {
    type Item = Point2;

    fn production_rules(self) -> Vec<Self> {
        use Gosper::*;
        match self {
            A => vec![A, Minus, B, Minus, Minus, B, Plus, A, Plus, Plus, A, A, Plus, B, Minus],
            B => vec![Plus, A, Minus, B, B, Minus, Minus, B, Minus, A, Plus, Plus, A, Plus, B],
            Plus => vec![Plus],
            Minus => vec![Minus]
        }
    }

    fn reify(v: &Self, current_pt: &mut Self::Item, current_angle: &mut f32, angle_step: f32, line_length: f32) -> Option<Self::Item>
    {
        use Gosper::*;
        let cpy = current_pt.clone();
        match v {
            A => *current_pt = line_to(*current_pt, *current_angle, line_length),
            B => *current_pt = line_to(*current_pt, *current_angle, line_length),
            Plus => *current_angle += angle_step,
            Minus => *current_angle -= angle_step,
        };

        match v {
            A | B => Some(cpy),
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