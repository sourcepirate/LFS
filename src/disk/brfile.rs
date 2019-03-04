use brotli::enc::BrotliEncoderParams;
use brotli::{CompressorWriter, Decompressor};
use std::io::{self, BufReader, BufWriter, Read, Write};

pub const DEFAULT_BUFF: usize = 4096;

pub struct BrReader<T>
where
    T: Read,
{
    inner: BufReader<Decompressor<T>>,
}

impl<T> BrReader<T>
where
    T: Read,
{
    pub fn new(inner: T) -> Self {
        BrReader {
            inner: BufReader::new(Decompressor::new(inner, DEFAULT_BUFF)),
        }
    }
}

pub struct BrWriter<T>
where
    T: Write,
{
    inner: BufWriter<CompressorWriter<T>>,
}

impl<T> BrWriter<T>
where
    T: Write,
{
    pub fn new(inner: T) -> Self {
        let params = BrotliEncoderParams::default();
        BrWriter {
            inner: BufWriter::new(CompressorWriter::with_params(inner, DEFAULT_BUFF, &params)),
        }
    }
}

impl<T> Read for BrReader<T>
where
    T: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<T> Write for BrWriter<T>
where
    T: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
