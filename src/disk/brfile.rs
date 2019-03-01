use brotli::enc::BrotliEncoderParams;
use brotli::{CompressorWriter, Decompressor};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

pub const DEFAULT_BUFF: usize = 4096;

pub struct BrReader<'a, T>
where
    &'a T: Read,
{
    inner: BufReader<Decompressor<&'a T>>,
}

impl<'a, T> BrReader<'a, T>
where
    &'a T: Read,
{
    fn new(inner: &'a T) -> Self {
        BrReader {
            inner: BufReader::new(Decompressor::new(inner, DEFAULT_BUFF)),
        }
    }
}

pub struct BrWriter<'a, T>
where
    &'a mut T: Write,
{
    inner: BufWriter<CompressorWriter<&'a mut T>>,
}

impl<'a, T> BrWriter<'a, T>
where
    &'a mut T: Write,
{
    fn new(inner: &'a mut T) -> Self {
        let params = BrotliEncoderParams::default();
        BrWriter {
            inner: BufWriter::new(CompressorWriter::with_params(inner, DEFAULT_BUFF, &params)),
        }
    }
}

impl<'a, T> Read for BrReader<'a, T>
where
    &'a T: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<'a, T> Write for BrWriter<'a, T>
where
    &'a mut T: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

pub struct BrFile<'a, T>
where
    &'a mut T: Write,
    &'a T: Read,
{
    block: &'a mut T,
}

impl<'a, T> BrFile<'a, T>
where
    &'a mut T: Write,
    &'a T: Read,
{
    fn new(block_device: &'a mut T) -> Self {
        BrFile {
            block: block_device,
        }
    }

    fn into_reader(&'a self) -> BrReader<T> {
        BrReader::new(&*self.block)
    }

    fn into_writer(&'a mut self) -> BrWriter<T> {
        BrWriter::new(self.block)
    }
}

pub fn open_br_file<'a>(file: &'a mut File) -> BrFile<File> {
    BrFile::new(file)
}
