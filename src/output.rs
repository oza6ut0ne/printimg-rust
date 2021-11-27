use anyhow::Result;
use opencv::{core, prelude::*};
use std::io::{BufWriter, StdoutLock, Write};


pub struct PrinterFactory;

impl PrinterFactory {
    pub fn create(flat: &bool) -> Box<dyn Printer> {
        if *flat {
            Box::new(FlatPrinter)
        } else {
            Box::new(SuperResolutionPrinter)
        }
    }
}


pub trait Printer {
    fn print_img(&self, img: &core::Mat, out: &mut BufWriter<StdoutLock>) -> Result<()>;
}


struct FlatPrinter;

impl Printer for FlatPrinter {
    fn print_img(&self, img: &core::Mat, out: &mut BufWriter<StdoutLock>) -> Result<()> {
        let size = img.size()?;
        for y in 0..size.height {
            for x in 0..size.width {
                let val = img.at_2d::<core::Vec3b>(y, x)?;
                write!(out, "\x1b[48;2;{};{};{}m  ", val[2], val[1], val[0])?;
            }
            out.write_all(b"\x1b[0m\n")?;
        }
        Ok(())
    }
}


struct SuperResolutionPrinter;

impl Printer for SuperResolutionPrinter {
    fn print_img(&self, img: &core::Mat, out: &mut BufWriter<StdoutLock>) -> Result<()> {
        let size = img.size()?;
        for y in (0..size.height - 1).step_by(2) {
            for x in 0..size.width {
                let upper = img.at_2d::<core::Vec3b>(y, x)?;
                let lower = img.at_2d::<core::Vec3b>(y + 1, x)?;
                write!(
                    out, "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀",
                    upper[2], upper[1], upper[0],
                    lower[2], lower[1], lower[0],
                )?;
            }
            out.write_all(b"\x1b[0m\n")?;
        }
        Ok(())
    }
}
