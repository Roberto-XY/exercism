#[derive(Debug)]
pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    write_idx: usize,
    read_idx: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: empty_buffer(capacity),
            read_idx: 0,
            write_idx: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        match self.buffer[self.write_idx] {
            Some(_) => Err(Error::FullBuffer),
            None => {
                self.unchecked_write(element);
                Ok(())
            }
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        match self.buffer[self.read_idx].take() {
            None => Err(Error::EmptyBuffer),
            Some(res) => {
                self.read_idx = (self.read_idx + 1) % self.buffer.capacity();
                Ok(res)
            }
        }
    }

    pub fn clear(&mut self) {
        self.buffer = empty_buffer(self.buffer.capacity());
        self.read_idx = 0;
        self.write_idx = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        if self.is_full() {
            self.unchecked_write(element);
            self.read_idx = (self.read_idx + 1) % self.buffer.capacity();
        } else {
            self.unchecked_write(element);
        }
    }

    fn is_full(&self) -> bool {
        self.buffer[self.write_idx].is_some()
    }

    fn unchecked_write(&mut self, element: T) {
        self.buffer[self.write_idx] = Some(element);
        self.write_idx = (self.write_idx + 1) % self.buffer.capacity();
    }
}

fn empty_buffer<T>(capacity: usize) -> Vec<Option<T>> {
    (0..capacity).map(|_| None).collect()
}
