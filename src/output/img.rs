use std::io::{BufWriter, Write};

use anyhow::Result;
use image::{DynamicImage, GenericImageView};

pub trait Printer<T: Write> {
    fn print_img(&self, img: &DynamicImage, out: &mut BufWriter<T>) -> Result<()>;
}

pub struct FlatPrinter;

impl<T: Write> Printer<T> for FlatPrinter {
    fn print_img(&self, img: &DynamicImage, out: &mut BufWriter<T>) -> Result<()> {
        let size = img.dimensions();
        for y in 0..size.1 {
            for x in 0..size.0 {
                let val = img.get_pixel(x, y);
                write!(out, "\x1b[48;2;{};{};{}m  ", val[0], val[1], val[2])?;
            }
            out.write_all(b"\x1b[0m\n")?;
        }
        Ok(())
    }
}

pub struct SuperResolutionPrinter;

impl<T: Write> Printer<T> for SuperResolutionPrinter {
    fn print_img(&self, img: &DynamicImage, out: &mut BufWriter<T>) -> Result<()> {
        let size = img.dimensions();
        for y in (0..size.1 - 1).step_by(2) {
            for x in 0..size.0 {
                let upper = img.get_pixel(x, y);
                let lower = img.get_pixel(x, y + 1);
                write!(
                    out,
                    "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀",
                    upper[0], upper[1], upper[2], lower[0], lower[1], lower[2],
                )?;
            }
            out.write_all(b"\x1b[0m\n")?;
        }
        Ok(())
    }
}
