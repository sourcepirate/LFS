//! All commands for LZMA filesystem
//! compression tool.

use lzma::{LzmaReader, LzmaWriter};
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Write};

const DEFAULT_COMPRESSION: u32 = 9;

/// unarchive the lzma file
fn cat(filename: String) -> () {
    let is_archive = filename.find(".xz");
    match is_archive {
        Some(_) => {
            let _dfile = File::open(&filename).unwrap();
            let _decompressed = LzmaReader::new_decompressor(_dfile).unwrap();
            let mut _reader = BufReader::new(_decompressed);
            let mut out = stdout();
            loop {
                let mut content = String::new();
                let res = _reader.read_line(&mut content);
                if res.is_err() || content.is_empty() {
                    break;
                }
                out.write(content.as_bytes());
            }
        }
        None => (),
    };
}

/// compress the given text file
fn compress(filename: String) -> () {
    let new_filename = format!("{}.xz", filename.clone());

    let _dfile = File::open(&filename).unwrap();
    let _cfile = File::create(&new_filename).unwrap();
    let mut _compressed = LzmaWriter::new_compressor(_cfile, DEFAULT_COMPRESSION).unwrap();
    let mut _reader = BufReader::new(_dfile);
    loop {
        let mut content = String::new();
        _reader.read_line(&mut content);
        if content.is_empty() {
            _compressed.finish();
            break;
        }
        _compressed.write(content.as_bytes());
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
