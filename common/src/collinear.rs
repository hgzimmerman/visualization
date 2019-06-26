use nannou::geom::Point2;
use nannou::prelude::*;

// TODO refine this into a custom iterator.
// As it is it is less perfect than the alternative, and still has to collect into a vec.
// An iterator solution could check if its internal iterator is none, as well as its prev.
/// Will remove the middle points of groups of 3 collinear points that appear in linear order along the curve.
pub fn condense_collinear_iter(pts: Vec<Point2>) -> Vec<Point2> {
    let mut prev_1 = None; // Most recent
    let mut prev_2 = None; // Second most recent

    let mut pts: Vec<Point2> = pts.into_iter()
        .filter_map(|pt| {
            let ret = if let Some(prev1) = prev_1 {
                if let Some(prev2) = prev_2 {
                    if are_collinear(prev2, prev1, pt) {
                        None
                    } else {
                        Some(prev1)
                    }
                } else {
                    Some(pt) // Depending on the curve, this can be either.
                }
            } else {
                Some(pt)
            };

            // End
            prev_2 = prev_1;
            prev_1 = Some(pt);
            ret
        })
    .collect();

    // Handle the last element, which needs to be added.
    // The current last point may need to be discarded if the previous and the last are collinear with it.
    let prev = prev_1.unwrap_or_default();
    if are_collinear(pts[pts.len() - 2], pts[pts.len() - 1], prev) {
        pts.pop();
    }
    pts.push(prev);
    pts
}


/// Will remove the middle points of groups of 3 collinear points that appear in linear order along the curve.
pub fn condense_collinear(pts: Vec<Point2>) -> Vec<Point2> {
    if pts.len() < 3 {
        return pts
    } else {
        let mut ret = vec![pts[0]];
        let mut candidate = pts[1];

        for i in 2..pts.len() {
            if !are_collinear(ret[ret.len() - 1], candidate, pts[i]) {
                ret.push(candidate)
            }

            candidate = pts[i]
        }
        // Handle the last element, which needs to be added.
        // The current last point may need to be discarded if the previous and the last are collinear with it.
        if are_collinear(ret[ret.len() - 2], ret[ret.len() - 1], candidate ) {
            ret.pop();
        }
        ret.push(candidate);
        ret
    }
}

pub fn are_collinear(pt1: Point2, pt2: Point2, pt3: Point2) -> bool {
//    dbg!((pt1, pt2, pt3));
    let x2_area =
        pt1.x * (pt2.y - pt3.y)
        + pt2.x * (pt3.y - pt1.y)
        + pt3.x * (pt1.y - pt2.y);
//    dbg!(x2_area);
//    dbg!(f32::epsilon());
    // Epsilon is apparently too small for some errors made when adjusting angles
    // Lowering the bar here allows some points that should be collinear to be calculated as such.
    x2_area.abs() < f32::epsilon() * 100000.0
}