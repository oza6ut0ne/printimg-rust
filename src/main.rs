use std::io::{self, Write};

use anyhow::{anyhow, Result};
use output::PrinterFactory;
use resize::ResizerFactory;
use structopt::StructOpt as _;

mod cli;
mod output;
mod resize;

#[cfg(feature = "opencv")]
mod util;

const USAGE: &str = "\
USAGE:
    printi <PATH>

For more information try --help\
";

fn restore_cursor() -> Result<()> {
    print!("\x1b[?25h\x1b[0m");
    io::stdout().flush()?;
    Ok(())
}

#[cfg(feature = "opencv")]
fn run(opt: cli::Opt) -> Result<()> {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    use anyhow::bail;
    use crossterm::{cursor, terminal, ExecutableCommand};
    use opencv::{core, imgproc, prelude::*, videoio};

    if opt.build_info {
        println!("{} {}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        print!("Built with OpenCV");
        println!("{}", core::get_build_information()?);
        return Ok(());
    }

    if !opt.verbose {
        util::suppress_stderr()
    }

    let path = opt.path.ok_or_else(|| anyhow!(USAGE))?;
    let mut cap = match path.parse() {
        #[cfg(feature = "opencv-32")]
        Ok(num) => videoio::VideoCapture::new_default(num)?,
        #[cfg(not(feature = "opencv-32"))]
        Ok(num) => videoio::VideoCapture::new(num, videoio::CAP_ANY)?,
        Err(_) => videoio::VideoCapture::from_file(&path, videoio::CAP_ANY)?,
    };
    if !videoio::VideoCapture::is_opened(&cap)? {
        bail!("Unable to open src.");
    }

    print!("\x1b[?25l");
    if (cap.get(videoio::CAP_PROP_FRAME_COUNT)? - 1f64).abs() > f64::EPSILON {
        if let Ok((_, y)) = cursor::position() {
            io::stdout().execute(terminal::ScrollUp(y))?;
            print!("\x1b[1;1H");
        }
    }

    let killed = Arc::new(AtomicBool::new(false));
    let k = Arc::clone(&killed);
    ctrlc::set_handler(move || {
        k.store(true, Ordering::SeqCst);
    })?;

    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());
    let resizer = ResizerFactory::create(opt.protrude, opt.flat);
    let printer = PrinterFactory::create(opt.flat);

    let mut first_frame = true;
    while !killed.load(Ordering::SeqCst) {
        let mut img = core::Mat::default();
        if !cap.read(&mut img)? {
            break;
        }

        if opt.rotate {
            let mut rotated = core::Mat::default();
            core::rotate(&img, &mut rotated, core::ROTATE_90_COUNTERCLOCKWISE)?;
            img = rotated;
        }

        let resized = resizer.resize_img(&img)?;
        if img.typ() == core::CV_8UC1 {
            imgproc::cvt_color(&img.clone(), &mut img, imgproc::COLOR_GRAY2BGR, 0)?;
        }

        if first_frame {
            first_frame = false;
        } else {
            out.write_all(b"\x1b[1;1H")?;
        }
        printer.print_img(&resized, &mut out)?;
    }
    out.flush()?;

    restore_cursor()?;
    Ok(())
}

#[cfg(not(feature = "opencv"))]
fn run(opt: cli::Opt) -> Result<()> {
    if opt.build_info {
        println!("{} {}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        println!("Built without OpenCV");
        return Ok(());
    }

    let path = opt.path.ok_or_else(|| anyhow!(USAGE))?;
    let mut img = image::open(path)?;

    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());
    let resizer = ResizerFactory::create(opt.protrude, opt.flat);
    let printer = PrinterFactory::create(opt.flat);

    if opt.rotate {
        img = img.rotate270();
    }

    let resized = resizer.resize_img(&img)?;

    print!("\x1b[?25l");
    ctrlc::set_handler(|| {
        restore_cursor().unwrap();
    })?;
    printer.print_img(&resized, &mut out)?;
    out.flush()?;

    restore_cursor()?;
    Ok(())
}

fn main() -> Result<()> {
    let opt = cli::Opt::from_args();

    if let Err(e) = run(opt) {
        println!("{:?}", e);
        restore_cursor()?;
        std::process::exit(1);
    }

    restore_cursor()?;
    Ok(())
}
