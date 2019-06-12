use std::iter::FromIterator;

/// A linear slice of memory that will wrap around when adding or subtracting elements
#[derive(Debug)]
pub struct RingBuffer<T> {
    start: usize,
    occupied: usize,
    buf: Vec<Option<T>> // The option T isn't ideal
}

impl <T> RingBuffer<T> {
    fn index(&self) -> usize {
        (self.start + self.occupied) % self.buf.len()
    }

    pub fn push(&mut self, value: T) {
        let index = self.index();
        self.buf[index] = Some(value);

        if self.occupied < self.buf.len() {
            self.occupied += 1;
        } else {
            self.start = (self.start + 1) % self.buf.len()
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.occupied > 0 {
            let index = self.index();
            let v = self.buf[index].take();
            self.occupied -= 1;
            v
        } else {
            None
        }

    }
    pub fn iter(&self) -> RingBufferIterator<T> {
        RingBufferIterator {
            ring_buffer: &self,
            pos: 0
        }
    }
}

impl <T: Clone> RingBuffer<T> {
    /// The size is fixed once the ring buffer is created.
    /// New entries will overwrite old ones.
    pub fn new(size: usize) -> Self {
        RingBuffer {
            start: 0,
            occupied: 0,
            buf: vec![None; size]
        }
    }
}

impl <T> IntoIterator for RingBuffer<T> {
    type Item = T;
    type IntoIter = IntoRingBufferIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoRingBufferIterator {
            ring_buffer: self,
            pos: 0
        }
    }
}

impl <A> FromIterator<A> for RingBuffer<A> {
    fn from_iter<T>(iter: T) -> Self
        where
        T: IntoIterator<Item = A>
    {
        let buf: Vec<Option<A>> = iter.into_iter().map(Option::Some).collect();
        RingBuffer {
            start: 0,
            occupied: buf.len(),
            buf
        }
    }
}


#[derive(Debug)]
pub struct IntoRingBufferIterator<T> {
    ring_buffer: RingBuffer<T>,
    pos: usize
}

impl <T> Iterator for IntoRingBufferIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.ring_buffer.occupied {
           None
        } else {
            let index = (self.ring_buffer.start + self.pos) % self.ring_buffer.buf.len();
            self.pos += 1;
            self.ring_buffer.buf[index].take()
        }
    }
}

#[derive(Debug)]
pub struct RingBufferIterator<'a, T> {
    ring_buffer: &'a RingBuffer<T>,
    pos: usize
}

impl <'a, T> Iterator for RingBufferIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.ring_buffer.occupied {
           None
        } else {
            let index = (self.ring_buffer.start + self.pos) % self.ring_buffer.buf.len();
            self.pos += 1;
            self.ring_buffer.buf[index].as_ref()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ring_buffer() {
        let mut rb = RingBuffer::new(10);
        rb.push(1);
        rb.push(2);
        let mut iter = rb.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ring_buffer2() {
        let mut rb = RingBuffer::new(3);
        println!("{:?}", rb);
        rb.push(1);
        println!("{:?}", rb);
        rb.push(2);
        println!("{:?}", rb);
        rb.push(3);
        println!("{:?}", rb);
        rb.push(4);
        println!("{:?}", rb);


        let mut iter = rb.iter();
        println!("{:?}", iter);
        assert_eq!(iter.next(), Some(&2));
        println!("{:?}", iter);
        assert_eq!(iter.next(), Some(&3));
        println!("{:?}", iter);
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }
}