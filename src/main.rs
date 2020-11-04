mod image;

use crate::image::Dot;
use anyhow::{bail, Result};
use qrcode::QrCode;
use std::io::{self, Read};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    /// Data to display in a terminal QR code.
    #[structopt(name = "DATA")]
    data: Vec<String>,
}

fn main() -> Result<()> {
    let options = Options::from_args();
    let data = if options.data.is_empty() {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;
        buffer
    } else {
        options.data.join(" ").into_bytes()
    };

    if data.is_empty() {
        bail!("empty data");
    }

    // TODO(nlordell): Implement terminal colours.
    let image = QrCode::new(&data)?.render::<Dot>().build();
    for line in &image.lines {
        for point in line {
            print!("{}", point.to_char());
        }
        println!();
    }
    if let Some(last_line) = &image.last_line {
        for point in last_line {
            print!("{}", point.to_char());
        }
        println!();
    }

    Ok(())
}
