use super::brfile::{BrFile, BrReader, BrWriter};
use std::collections::HashMap;
use std::fs::File;

pub struct Disk {
    files: HashMap<u32, String>,
    inodes: Vec<u32>,
    opened: Vec<u32>,
    mount: String,
}

impl Disk {
    fn new(mountpoint: String) -> Self {
        Disk {
            files: HashMap::new(),
            inodes: Vec::new(),
            opened: Vec::new(),
            mount: mountpoint,
        }
    }
}
