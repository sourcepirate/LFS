//! All commands for LZMA filesystem
//! compression tool.

use brotli::enc::BrotliEncoderParams;
use brotli::{CompressorWriter, Decompressor};
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Write};

const DEFAULT_BUFFER: u32 = 4096;

/// unarchive the lzma file
fn cat(filename: String) -> () {
    let is_archive = filename.find(".br");
    match is_archive {
        Some(_) => {
            let _dfile = File::open(&filename).unwrap();
            let _decompressed = Decompressor::new(_dfile, DEFAULT_BUFFER as usize);
            let mut _reader = BufReader::new(_decompressed);
            let mut out = stdout();
            loop {
                let mut content = String::new();
                let res = _reader.read_line(&mut content);
                if res.is_err() || content.is_empty() {
                    break;
                }
                out.write(content.as_bytes()).unwrap();
            }
        }
        None => (),
    };
}

/// compress the given text file
fn compress(filename: String) -> () {
    let new_filename = format!("{}.br", filename.clone());

    let _dfile = File::open(&filename).unwrap();
    let mut _cfile = File::create(&new_filename).unwrap();
    let params = BrotliEncoderParams::default();
    let mut _compressed = CompressorWriter::with_params(_cfile, DEFAULT_BUFFER as usize, &params);
    let mut _reader = BufReader::new(_dfile);
    loop {
        let mut content = String::new();
        _reader.read_line(&mut content).unwrap();
        if content.is_empty() {
            break;
        }
        _compressed.write(content.as_bytes()).unwrap();
    }
}

#[derive(Debug, Deserialize)]
pub struct Opts {
    pub cmd_cat: bool,
    pub cmd_compress: bool,
    pub arg_name: String,
}

impl Opts {
    pub fn run(&self) -> () {
        if self.cmd_cat {
            cat(self.arg_name.clone());
        } else if self.cmd_compress {
            compress(self.arg_name.clone());
        }
    }
}
