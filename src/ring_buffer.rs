use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

/// A linear slice of memory that will wrap around when adding or subtracting elements
#[derive(Debug)]
pub struct RingBuffer<T> {
    /// The index to start iterating from.
    start: usize,
    /// Number of occupied elements in the ring buffer
    occupied: usize,
    /// Max size
    size: usize,
    /// Backing buffer.
    buf: Vec<T>
}

impl <T> RingBuffer<T> {
    /// First open spot, or the next element to replace.
    fn insert_index(&self) -> usize {
        (self.start + self.occupied) % self.size
    }

    /// The size is fixed once the ring buffer is created.
    /// New entries will overwrite old ones.
    pub fn new(size: usize) -> Self {
        RingBuffer {
            start: 0,
            occupied: 0,
            size,
            buf: Vec::new()
        }
    }

    /// clears the iterator.
    pub fn clear(&mut self) {
        self.buf.clear();
        self.start = 0;
        self.occupied = 0;
    }

    /// Max size of ring buffer.
    pub fn size(&self) -> usize {
        self.size
    }

    /// How many slots in the ring buffer are occupied
    pub fn occupied(&self) -> usize {
        self.occupied
    }

    pub fn first(&self) -> Option<&T> {
        if self.occupied == 0 {
            None
        } else {
            Some(&self.buf[self.start])
        }
    }

    // TODO consider removing and replacing with rb[rb.occupied()]
    pub fn last(&self) -> Option<&T> {
        if self.occupied == 0 {
            None
        } else {
            Some(&self.buf[(self.start + self.occupied - 1) % self.size])
        }
    }

    /// If it evicts a value, it will be returned.
    pub fn push(&mut self, value: T) -> Option<T> {
        if self.occupied < self.size {
            self.buf.push(value);
            self.occupied += 1;
            None
        } else {
            let index = self.insert_index();
            let v = std::mem::replace(&mut self.buf[index], value);
            self.start = (self.start + 1) % self.buf.len();
            Some(v)
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.occupied > 0 {
            let v = self.buf.pop();
            self.occupied -= 1;
            v
        } else {
            None
        }
    }

    /// Sets the start back to index 0, and the occupied length to the size of the buffer after it has been retained.
    pub fn retain<F>(&mut self, f: F)
        where F: FnMut(&T) -> bool
    {
        self.buf.retain(f);
        self.start = 0;
        self.occupied = self.buf.len();
    }

    pub fn iter(&self) -> RingBufferIterator<T> {
        RingBufferIterator {
            ring_buffer: &self,
            pos: 0
        }
    }

    pub fn iter_mut(&mut self) -> RingBufferMutIterator<T> {
        RingBufferMutIterator {
            ring_buffer: self,
            pos: 0
        }
    }
}

impl <T> Index<usize> for RingBuffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[(self.start + index) % self.occupied]
    }
}

impl <T> IndexMut<usize> for RingBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[(self.start + index) % self.occupied]
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
        let buf: Vec<A> = iter.into_iter().collect();
        RingBuffer {
            start: 0,
            occupied: buf.len(),
            size: buf.len(),
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
            let index = (self.ring_buffer.start + self.pos) % self.ring_buffer.size;
            self.pos += 1;
            Some(self.ring_buffer.buf.swap_remove(index))
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
            let index = (self.ring_buffer.start + self.pos) % self.ring_buffer.size;
            self.pos += 1;
            Some(&self.ring_buffer.buf[index])
        }
    }
}

#[derive(Debug)]
pub struct RingBufferMutIterator<'a, T: 'a> {
    ring_buffer: &'a mut RingBuffer<T>,
    pos: usize
}

impl <'a, T> Iterator for RingBufferMutIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.ring_buffer.occupied {
           None
        } else {
            let index = (self.ring_buffer.start + self.pos) % self.ring_buffer.size;
            self.pos += 1;
            unsafe {
                let buf_ptr = self.ring_buffer.buf.as_mut_ptr();
                Some(&mut *(buf_ptr.offset( index as isize)))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ring_buffer_iter() {
        let mut rb = RingBuffer::new(10);
        rb.push(1);
        rb.push(2);
        let mut iter = rb.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ring_buffer_iter_overwrite() {
        let mut rb = RingBuffer::new(3);
        rb.push(1);
        rb.push(2);
        rb.push(3);
        rb.push(4);


        let mut iter = rb.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ring_buffer_from_iter() {
        let rb = (0..10).collect::<RingBuffer<_>>();
        assert_eq!(rb.size(), 10)
    }

    #[test]
    fn ring_buffer_iter_mut() {
        let mut rb = RingBuffer::new(10);
        rb.push(1);
        rb.push(2);
        rb.push(3);
        let mut iter = rb.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));

        let z = 20;
        if let Some(v) = iter.next() {
            *v = z;
        }

        assert_eq!(rb.pop(), Some(20))
    }

    #[test]
    fn index() {
        let mut rb = RingBuffer::new(10);
        rb.push(1);
        rb.push(2);
        rb.push(3);
        assert_eq!(rb[0], 1);
        assert_eq!(rb[1], 2);
        assert_eq!(rb[2], 3);
        assert_eq!(rb[3], 1); // wraps back around
    }

    #[test]
    fn index_on_wrapped() {
        let mut rb = RingBuffer::new(2);
        rb.push(1);
        rb.push(2);
        rb.push(3);
        assert_eq!(rb[0], 2);
        assert_eq!(rb[1], 3);
        assert_eq!(rb[2], 2);
    }
}