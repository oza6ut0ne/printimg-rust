use std::{env, fs::File, mem, os::unix::io::IntoRawFd};
use std::io::{self, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{anyhow, Result};
use ctrlc;
use libc::{dup2, ioctl, STDERR_FILENO, TIOCGWINSZ, winsize};
use opencv::{
    core,
    imgproc,
    prelude::*,
    videoio,
};

fn get_terminal_size() -> Option<(i32, i32)> {
    let fd = match File::open("/dev/tty") {
        Ok(file) => file.into_raw_fd(),
        Err(_) => STDERR_FILENO
    };

    let mut ws: winsize = unsafe { mem::zeroed() };
    if unsafe { ioctl(fd, TIOCGWINSZ, &mut ws) } == -1 {
        None
    } else {
        Some((ws.ws_col as i32, ws.ws_row as i32))
    }
}

fn suppress_stderr() {
    let fd = match File::create("/dev/null") {
        Ok(file) => file.into_raw_fd(),
        Err(_) => return
    };
    unsafe { dup2(fd, STDERR_FILENO) };
}

fn calc_resized_img_size(img_w: i32, img_h: i32, term_w: i32, term_h: i32) -> (i32, i32) {
    let term_h = term_h - 2;
    let mut h = term_h;
    let mut w = img_w * term_h / img_h;
    if (term_w / 2) < w {
        h = h * term_w / 2 / w;
        w = term_w / 2;
    }
    (w, h)
}

fn resize_img(img: &core::Mat) -> Result<core::Mat> {
    let size = img.size()?;
    let (term_w, term_h) = get_terminal_size().unwrap_or((80, 24));
    let (w, h) = calc_resized_img_size(size.width, size.height, term_w, term_h);

    let mut resized = core::Mat::default()?;
    imgproc::resize(img, &mut resized, core::Size::new(w, h),
                    0f64, 0f64, imgproc::INTER_LINEAR)?;
    Ok(resized)
}

fn print_img <T: io::Write> (img: &core::Mat, out: &mut io::BufWriter<T>) -> Result<()> {
    let size = img.size()?;
    for y in 0..size.height {
        for x in 0..size.width {
            let val = img.at_2d::<core::Vec3b>(y, x)?;
            write!(out, "\x1b[48;2;{};{};{}m  ", val[2], val[1], val[0])?;
        }
        out.write(b"\x1b[0m\n")?;
    }
    Ok(())
}

fn run(path: &str) -> Result<()> {
    let mut cam = match path.parse::<i32>() {
        Ok(num) => videoio::VideoCapture::new(num, videoio::CAP_ANY)?,
        Err(_) => videoio::VideoCapture::from_file(path, videoio::CAP_ANY)?,
    };
    if ! videoio::VideoCapture::is_opened(&cam)? {
        return Err(anyhow!("Unable to open src."));
    }

    let out = io::stdout();
    let mut out = io::BufWriter::new(out.lock());

    let killed = Arc::new(AtomicBool::new(false));
    let k = Arc::clone(&killed);
    ctrlc::set_handler(move || {
        k.store(true, Ordering::SeqCst);
    })?;

    let mut first_frame = true;
    while ! killed.load(Ordering::SeqCst) {
        let mut img = core::Mat::default()?;
        if ! cam.read(&mut img)? {
            break;
        }

        if img.typ()? == core::CV_8UC1 {
            imgproc::cvt_color(&img.clone(), &mut img, imgproc::COLOR_GRAY2BGR, 0)?;
        }
        let resized = resize_img(&img)?;

        if first_frame {
            first_frame = false;
        } else {
            out.write(b"\x1b[1;1H")?;
        }
        print_img(&resized, &mut out)?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No src");
    }
    if env::var("PRINTI_DEBUG").is_err() {
        suppress_stderr();
    }
    print!("\x1b[?25l");
    if let Err(e) = run(&args[1]) {
        println!("{:?}", e);
    }
    println!("\x1b[?25h\x1b[0m");
}
