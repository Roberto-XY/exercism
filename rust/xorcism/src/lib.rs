use std::{
    borrow::Borrow,
    io::{BufWriter, Read, Write},
    iter::Cycle,
    slice::Iter,
};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: Cycle<Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: ?Sized + AsRef<[u8]>>(key: &'a Key) -> Xorcism<'a> {
        Xorcism {
            key: key.as_ref().iter().cycle(),
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut()
            .zip(&mut self.key)
            .for_each(|(data_byte, key_byte)| *data_byte ^= *key_byte)
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'b, T, Data>(
        &'b mut self,
        data: Data,
    ) -> impl Iterator<Item = u8> + Captures<'a> + 'b
    where
        T: 'b + Borrow<u8>,
        Data: IntoIterator<Item = T>,
        <Data as IntoIterator>::IntoIter: 'b,
    {
        data.into_iter()
            .zip(&mut self.key)
            .map(|(data_byte, key_byte)| *data_byte.borrow() ^ *key_byte)
    }

    pub fn reader<R>(self, reader: R) -> XorcismReader<'a, R>
    where
        R: Read,
    {
        XorcismReader::new(reader, self)
    }

    pub fn writer<W>(self, writer: W) -> XorcismWriter<'a, W>
    where
        W: Write,
    {
        XorcismWriter::new(writer, self)
    }
}
pub struct XorcismReader<'a, R> {
    reader: R,
    munger: Xorcism<'a>,
}

impl<'a, R> XorcismReader<'a, R> {
    fn new(reader: R, munger: Xorcism<'a>) -> Self {
        XorcismReader { reader, munger }
    }
}

impl<'a, R> Read for XorcismReader<'a, R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf).map(|n| {
            self.munger.munge_in_place(&mut buf[..n]);
            n
        })
    }
}

pub struct XorcismWriter<'a, W>
where
    W: Write,
{
    writer: std::io::BufWriter<W>,
    munger: Xorcism<'a>,
}

impl<'a, W> XorcismWriter<'a, W>
where
    W: Write,
{
    fn new(writer: W, munger: Xorcism<'a>) -> Self {
        XorcismWriter {
            writer: BufWriter::new(writer),
            munger,
        }
    }
}

impl<'a, W> Write for XorcismWriter<'a, W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.munger
            .munge(buf)
            .try_fold(0, |s, b| self.writer.write(&[b]).map(|n| s + n))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

pub trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}
