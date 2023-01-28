//! This example is use `QmcDump` to convert a qmc file to flac file
//!
//! You should use your own qmcflac file instead the test file
//!
use std::fs::File;
use std::io::Write;

use anyhow::Result;
use ncmdump::QmcDump;

fn main() -> Result<()> {
    let file = File::open("tests/test.qmcflac")?;
    let mut qmc = QmcDump::from_reader(file)?;
    let data = qmc.get_data()?;

    let mut target = File::options()
        .create(true)
        .write(true)
        .open("tests/test.flac")?;
    target.write_all(&data)?;
    Ok(())
}
