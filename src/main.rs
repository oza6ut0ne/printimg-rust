use std::io::{self, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{bail, Result};
use crossterm::{cursor, ExecutableCommand, terminal};
use opencv::{
    core,
    imgproc,
    prelude::*,
    videoio,
};
use output::PrinterFactory;
use resize::ResizerFactory;
use structopt::StructOpt as _;

mod cli;
mod output;
mod resize;
mod util;


fn run(opt: cli::Opt) -> Result<()> {
    let mut cam = match opt.path.parse() {
        #[cfg(feature = "opencv-32")]
        Ok(num) => videoio::VideoCapture::new_default(num)?,
        #[cfg(not(feature = "opencv-32"))]
        Ok(num) => videoio::VideoCapture::new(num, videoio::CAP_ANY)?,
        Err(_) => videoio::VideoCapture::from_file(&opt.path, videoio::CAP_ANY)?,
    };
    if ! videoio::VideoCapture::is_opened(&cam)? {
        bail!("Unable to open src.");
    }

    if (cam.get(videoio::CAP_PROP_FRAME_COUNT)? - 1f64).abs() > f64::EPSILON {
        if let Ok((_, y)) = cursor::position() {
            io::stdout().execute(terminal::ScrollUp(y))?;
            print!("\x1b[1;1H");
        }
    }

    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());
    let resizer = ResizerFactory::create(&opt.protrude, &opt.flat);
    let printer = PrinterFactory::create(&opt.flat);

    let killed = Arc::new(AtomicBool::new(false));
    let k = Arc::clone(&killed);
    ctrlc::set_handler(move || {
        k.store(true, Ordering::SeqCst);
    })?;

    let mut first_frame = true;
    while ! killed.load(Ordering::SeqCst) {
        let mut img = core::Mat::default();
        if ! cam.read(&mut img)? {
            break;
        }

        if opt.rotate {
            let mut rotated = core::Mat::default();
            core::rotate(&img, &mut rotated, core::ROTATE_90_COUNTERCLOCKWISE)?;
            img = rotated;
        }

        let resized = resizer.resize_img(&img)?;
        if img.typ()? == core::CV_8UC1 {
            imgproc::cvt_color(&img.clone(), &mut img, imgproc::COLOR_GRAY2BGR, 0)?;
        }

        if first_frame {
            first_frame = false;
        } else {
            out.write_all(b"\x1b[1;1H")?;
        }
        printer.print_img(&resized, &mut out)?;
    }
    Ok(())
}

fn main() {
    let opt = cli::Opt::from_args();
    if ! opt.verbose {
        util::suppress_stderr()
    }

    print!("\x1b[?25l");
    if let Err(e) = run(opt) {
        println!("{:?}", e);
    }
    print!("\x1b[?25h\x1b[0m");
}
