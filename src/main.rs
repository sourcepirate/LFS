//! The tools provides the implementation for running the
//! json rpc based compression archive store for text data
//! the module uses the LZMA/Brotli compression to store text
//! data

extern crate docopt;
extern crate env_logger;
extern crate jsonrpc_core;
extern crate libc;
extern crate lzma;
extern crate serde;

#[macro_use]
extern crate log;
extern crate jsonrpc_derive;

#[macro_use]
extern crate serde_derive;

use docopt::Docopt;

pub mod command;
pub mod rpc;

const USAGE: &'static str = "
LFS filesystem

Usage:
  lfs cat <name>
  lfs compress <name>

Options:
  -h --help     Show this screen.
";

use command::Opts;

fn main() {
    env_logger::init();
    info!("Initializing...");
    let args: Opts = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    args.run();
}
