use nannou::geom::Point2;
use std::collections::VecDeque;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize
}

impl Point {
    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

}

pub fn d2xy(n: usize, d: usize) -> Point {

    let mut t = d;
    let mut pt = Point::default();
    let mut s = 1;

    while s < n {
        let rx = 1 & (t/2);
        let ry = 1 & (t ^ rx);
        pt = rot(s, pt, rx, ry);
        pt.x += s * rx;
        pt.y += s * ry;
        t /= 4;
        s *= 2;
    }
    pt

}


fn rot(n: usize, mut pt: Point, rx: usize, ry: usize) -> Point {
    if ry == 0 {
        if rx == 1 {
            pt.x = n-1 - pt.x;
            pt.y = n-1 - pt.y;
        }

        std::mem::swap(&mut pt.x, &mut pt.y);
    }
    pt
}


#[test]
fn hilbert_n_2() {
    assert_eq!(d2xy(2, 0), Point {
        x: 0,
        y: 0
    });

    assert_eq!(d2xy(2, 1), Point {
        x: 0,
        y: 1
    });

    assert_eq!(d2xy(2, 2), Point {
        x: 1,
        y: 1
    });

    assert_eq!(d2xy(2, 3), Point {
        x: 1,
        y: 0
    });
}

#[test]
fn iteration_limit_2() {
    let iter = HilbertIterator::new_with_iteration(2);
    assert_eq!(iter.d_max(), 15);
}
#[test]
fn iteration_limit_3() {
    let iter = HilbertIterator::new_with_iteration(3);
    assert_eq!(iter.d_max(), 63);
}


pub struct HilbertIterator {
    iteration: usize,
    d: usize
}

impl HilbertIterator {
    pub fn new() -> Self {
        Self::new_with_iteration(0)
    }

    #[inline]
    pub fn new_with_iteration(iteration: usize) -> Self {
        HilbertIterator {
            iteration,
            d: 0
        }
    }

    pub fn d_max(&self) -> usize {
        (4_usize.pow((self.iteration) as u32) ) - 1
    }
    pub fn n(&self) -> usize {
        2_usize.pow(self.iteration as u32)
    }
}


impl Iterator for HilbertIterator {
    type Item = (Point, Point);
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n();

        let pt_0 = d2xy(n, self.d);
        self.d += 1;
        let pt_1 = d2xy(n, self.d);

        if self.d > self.d_max() {
            None
        } else {
            Some((pt_0, pt_1))
        }
    }
}


/// http://bit-player.org/2013/mapping-the-hilbert-curve
/// Maps a number between 0 and 1 to a two dimensional point.
/// N [0, 1)
fn hilbret_map(n: f32) -> Point2 {


    let s = format!("{}", n);
    let s = s.split(".").collect::<Vec<_>>();

    let integer: &str = s[0];
    let fractional: &str = s.get(1).unwrap_or(&"0");


    fn to_digits(mut n: usize, b: usize) -> VecDeque<usize> {
        let mut digits = VecDeque::new();
        while n > 0 {
            digits.push_front(n % b);
            n  = n / b;
        }
        digits
    }

    fn from_digits(digits: &str, b: usize) -> usize {
        let mut n = 0;
        for d in digits.chars() {
            n = b * n + d.to_digit(10).unwrap() as usize
        }
        n
    }

    let mut integer: VecDeque<u8> = to_digits(integer.parse::<usize>().unwrap(), 4).into_iter().map(|x| x as u8).collect();
    let fractional: VecDeque<u8> = to_digits(fractional.parse::<usize>().unwrap(), 4).into_iter().map(|x| x as u8).collect();
    dbg!((&integer, &fractional));
    integer.extend(fractional);
    let mut quadits = integer;


    /// A quadit is represented by the digits {0, 1, 2, 3}
    fn hilbert_map_inner(quadits: &mut VecDeque<u8>) -> Point2 {
        if quadits.len() == 0 {
            Point2::from((0.5, 0.5))
        } else {
            let t = quadits.pop_front().unwrap();          // get first quadit
            let pt = hilbert_map_inner(quadits);     // recursive call
            let x = pt.x;
            let y = pt.y;
            match t {
                0 => Point2::from((y * 0.5, x * 0.5)),
                1 => Point2::from((x * 0.5, y * 0.5 + 0.5)),
                2 => Point2::from((x * 0.5 + 0.5, y * 0.5 + 0.5)),
                3 => Point2::from((y * -0.5 + 1.0, x * -0.5 + 0.5)),
                _ => panic!("should never get a number not in set of {0, 1, 2, 3}")
            }
        }
    }

    hilbert_map_inner(&mut quadits)
}

#[test]
fn hib_map() {
    assert_eq!(hilbret_map(0.0), Point2::from((0.5, 0.5)));
    assert_eq!(hilbret_map(4.0/17.0), Point2::from((0.0, 0.4)));
//    assert_eq!(hilbret_map(0.25), Point2::from((0.5, 0.5)));
}