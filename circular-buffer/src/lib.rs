#[derive(Debug)]
pub struct CircularBuffer<T: Clone> {
    m_write: usize,
    m_read: usize,
    m_size: usize,
    m_capacity: usize,
    m_buffer: Vec<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> where T: Clone + std::fmt::Debug {
    pub fn new(capacity: usize) -> Self {
        let b: Vec<T> = Vec::with_capacity(capacity);
        Self { m_write: 0, m_read: 0, m_size: 0, m_capacity: capacity, m_buffer: b }
    }
    
    fn is_empty(&self) -> bool {
        self.m_size == 0
    }

    fn is_full(&self) -> bool {
        self.m_size == self.m_capacity
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }
        if self.m_write == self.m_buffer.len() {
            self.m_buffer.push(_element);
        }
        else {
            self.m_buffer[self.m_write] = _element;
        }
        self.m_write = (self.m_write + 1) % self.m_capacity;
        self.m_size += 1;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::EmptyBuffer);
        }
        let element = self.m_buffer[self.m_read].clone();
        self.m_read = (self.m_read + 1) % self.m_capacity;
        self.m_size -= 1;
        Ok(element)
    }

    pub fn clear(&mut self) {
        self.m_write = 0;
        self.m_read = 0;
        self.m_buffer = Vec::new();
        self.m_size = 0;
    }

    pub fn overwrite(&mut self, _element: T) {
        if !self.is_full() {
            let _ = self.write(_element);
            return;
        }
        self.m_buffer[self.m_write] = _element;
        self.m_write = (self.m_write + 1) % self.m_capacity;
        self.m_read = (self.m_read + 1) % self.m_capacity;
    }
}
