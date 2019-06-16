
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

