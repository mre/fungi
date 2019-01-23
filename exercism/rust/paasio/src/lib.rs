use std::io::{Read, Result, Write};

// [read]: https://doc.rust-lang.org/std/io/trait.Read.html
// [write]: https://doc.rust-lang.org/std/io/trait.Write.html

pub struct ReadStats<R> {
    bytes_through: usize,
    reads: usize,
    wrapped: R,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        return Self {
            bytes_through: 0,
            reads: 0,
            wrapped: wrapped,
        };
    }

    pub fn get_ref(&self) -> &R {
        return &self.wrapped;
    }

    pub fn bytes_through(&self) -> usize {
        return self.bytes_through;
    }

    pub fn reads(&self) -> usize {
        return self.reads;
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        return match self.wrapped.read(buf) {
            Ok(n) => {
                self.bytes_through += n;
                return Ok(n);
            }
            Err(e) => Err(e),
        };
    }
}

pub struct WriteStats<W> {
    bytes_through: usize,
    writes: usize,
    wrapped: W,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        return Self {
            bytes_through: 0,
            writes: 0,
            wrapped: wrapped,
        };
    }

    pub fn get_ref(&self) -> &W {
        return &self.wrapped;
    }

    pub fn bytes_through(&self) -> usize {
        return self.bytes_through;
    }

    pub fn writes(&self) -> usize {
        return self.writes;
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        return match self.wrapped.write(buf) {
            Ok(n) => {
                self.bytes_through += n;
                return Ok(n);
            }
            Err(e) => Err(e),
        };
    }

    fn flush(&mut self) -> Result<()> {
        return self.wrapped.flush();
    }
}
