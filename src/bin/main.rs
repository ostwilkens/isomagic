extern crate dot_vox;
extern crate image;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate error_chain;
extern crate isomagic;

use dot_vox::{DotVoxData, Model, Voxel};
use image::{ImageBuffer, Rgba, RgbaImage};
use structopt::StructOpt;

use isomagic::enums::{Side, View};
use isomagic::Error;
use isomagic::{Options, Renderer};

use std::fs::create_dir_all;
use std::path::PathBuf;

fn run() -> Result<(), isomagic::Error> {
    let options = Options::from_args();

    let r = Renderer::new(&options.filename);
    match r {
        Ok(mut r) => (r.render_all(options)),
        Err(e) => Err(format!("Failed to parse '{}'.", &options.filename).into()),
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);

        for cause in error.iter().skip(1) {
            eprintln!("Caused by: {}", cause);
        }
    }
}
