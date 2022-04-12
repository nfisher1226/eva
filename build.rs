#[cfg(feature = "png-icons")]
use {
    std::{
        env,
        ffi::OsString,
        fs,
        path::PathBuf,
    },
    tiny_skia::Transform,
    usvg::{ FitTo, Options, Tree },
};

use std::error::Error;

#[cfg(feature = "png-icons")]
fn png(tree: &Tree, size: u32, outdir: &OsString) -> Result<(), Box<dyn Error>>{
    let fit = FitTo::Size(size, size);
    let transform = Transform::from_scale(1.0, 1.0);
    let mut pixmap = match tiny_skia::Pixmap::new(size, size) {
        Some(p) => p,
        None => return Err(String::from("Error creating png").into()),
    };
    resvg::render(&tree, fit, transform, pixmap.as_mut());
    let file = format!("eva{}x{}.png", size, size);
    let mut outfile = PathBuf::from(outdir);
    outfile.push(&file);
    pixmap.save_png(outfile)?;
    Ok(())
}

#[cfg(feature = "png-icons")]
fn iconvert(outdir: &OsString) -> Result<(), Box<dyn Error>> {
    let infile: PathBuf = ["data", "eva.svg"].iter().collect();
    let data = fs::read(&infile)?;
    let tree = Tree::from_data(&data, &Options::default().to_ref())?;
    for size in [256, 128, 64, 48, 32] {
        png(&tree, size, outdir)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "png-icons")]
    {
        let outdir = match env::var_os("OUT_DIR") {
            None => return Ok(()),
            Some(outdir) => outdir,
        };
        iconvert(&outdir)?;

        println!("cargo:warning=icons generated in: {:?}", outdir);
    }

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
