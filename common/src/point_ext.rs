use nannou::geom::Point2;

/// Produces a new point from the current one
#[inline]
pub fn line_to(pt: Point2, angle: f32, line_length: f32) -> Point2 {
    Point2 {
        x: pt.x + line_length * angle.cos(),
        y: pt.y + line_length * angle.sin()
    }
}