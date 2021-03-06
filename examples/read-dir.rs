// TODO: document
//! Lists files in a directory given as an argument.

use std::env;
use std::fs;

use futures::io;
use futures::prelude::*;
use smol::blocking;

fn main() -> io::Result<()> {
    let path = env::args().nth(1).expect("missing path argument");

    smol::run(async move {
        let mut dir = smol::iter(blocking!(fs::read_dir(path))?);

        while let Some(res) = dir.next().await {
            println!("{}", res?.file_name().to_string_lossy());
        }

        Ok(())
    })
}
