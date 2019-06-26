use nannou::geom::Point2;
use nannou::app::Draw;
use nannou::draw::properties::Rgba;
use nannou::color::Gradient;

/// Draws rainbow lines
pub fn rainbow_lines(draw: &Draw, points: &[Point2], thickness: f32, corner_thickness: f32, skip: usize) {
    let colors = vec![
        (0.0, Rgba::new_u8(0xff, 0, 0, 0xff)),
        (25.0, Rgba::new_u8(0, 0xff, 0, 0xff)),
        (75.0, Rgba::new_u8(0, 0, 0xff, 0xff)),
        (100.0, Rgba::new_u8(0xff, 0, 0, 0xff)),
    ];
    let gradient = Gradient::with_domain(colors.clone());
    colored_lines(draw, points, thickness, corner_thickness, skip, gradient)
}

/// Draws lines between a set of points with colors determined by a given gradient.
/// Skip will determine how far along the set of points the colors will rotate per draw cycle.
pub fn colored_lines(draw: &Draw, points: &[Point2], thickness: f32, corner_thickness: f32, skip: usize, gradient: Gradient<Rgba>) {
    let len = points.len();

    // Make the gradient cycle
    let mut gradient = gradient
        .take(len)
        .cycle()
        .skip(skip);

    if let Some(pt_1) = points.get(0) {
        draw.ellipse()
            .xy(*pt_1)
            .color(gradient.next().unwrap())
            .radius(corner_thickness);
    }

    points
        .windows(2)
        .zip( gradient)
        .for_each(|(window, color): (&[Point2], Rgba)| {
            let pt_0 = window[0];
            let pt_1 = window[1];
            draw.line()
                .start(pt_0)
                .end(pt_1)
                .thickness(thickness)
                .color(color);
            draw.ellipse()
                .xy(pt_1)
                .color(color)
                .radius(corner_thickness);
        });
}