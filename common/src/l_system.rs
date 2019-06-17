use apply::Apply;

/// Lindenmayer system
#[derive(Clone, Debug)]
pub struct LSystem<T>
{
    working_set: Vec<T>,
}

impl <T> LSystem<T>
where
    T: Expandable,
{
    pub fn new(axiom: Vec<T>) -> Self {
        LSystem {
            working_set: axiom,
        }
    }

    /// Advances the system to the next iteration by expanding the existing symbols using the production rules.
    pub fn iterate(self) -> Self {
        let working_set = self.working_set
            .into_iter()
            .flat_map(T::production_rules)
            .collect();
        LSystem {
            working_set,
        }
    }

    pub fn iterate_n(self, n: usize) -> Self {
        let mut iter: Box<Iterator<Item=T>> = Box::new(self.working_set.into_iter());
        // TODO I would love a better way to accomplish this, but Boxing is a quick way to erase type info so that I can prevent multiple collections.
        let i = (0..n)
            .fold( iter, |i: Box<Iterator<Item=T>>, _| {
                Box::new(i
                    .flat_map(T::production_rules)
                )
            });

        let working_set = i.collect();
        Self {
            working_set
        }
    }

    /// An iterator over the symbols.
    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.working_set.iter()
    }

    /// Turns the working set into a series of points.
    pub fn reify_iter(&self, angle: f32, line_length: f32, origin: T::Item) -> impl Iterator<Item=T::Item> + '_ {

        let reify = move |v: &T, current_pt: &mut T::Item, current_angle: &mut f32| {
            T::reify(v, current_pt, current_angle, angle, line_length)
        };

        ReificationIterator::new(self, origin, reify)
    }
}

pub trait Expandable: Sized {
    /// This is the type of point that will be used.
    type Item;

    /// Rules used to expand the set of symbols to its next iteration.
    fn production_rules(self) -> Vec<Self>;

    // TODO Reify is a shitty word for what this does
    fn reify(v: &Self, current_pt: &mut Self::Item, current_angle: &mut f32, angle_step: f32, line_length: f32) -> Self::Item;
}




pub struct ReificationIterator<'a, T, U,  F> {
    inner: std::slice::Iter<'a, T>,
    f: F,
    current_pt: U,
    current_angle: f32, // TODO this is fine for 2D, but I probably need another structure for representing higher-dimension angles.
}

impl <'a, T, U, F> ReificationIterator<'a, T, U, F>
{
    pub fn new(l_system: &'a LSystem<T>, origin: U, f: F) -> Self {
        ReificationIterator {
            inner: l_system.working_set.iter(),
            f,
            current_pt: origin,
            current_angle: 0.0,
        }
    }
}


impl <'a, T: 'a, U, F> Iterator for ReificationIterator<'a, T, U, F>
where
    F: Fn(&T, &mut U, &mut f32) -> U,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|t: &T| {
                (self.f)(t, &mut self.current_pt, &mut self.current_angle)
            })
    }
}
