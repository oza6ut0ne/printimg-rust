use std::{io::Write, marker::PhantomData};

#[cfg(feature = "opencv")]
use cv::*;
#[cfg(feature = "opencv")]
mod cv;

#[cfg(not(feature = "opencv"))]
use img::*;
#[cfg(not(feature = "opencv"))]
mod img;

pub struct PrinterFactory<T> {
    _marker: PhantomData<fn() -> T>,
}

impl<T: Write> PrinterFactory<T> {
    pub fn create(flat: bool) -> Box<dyn Printer<T>> {
        if flat {
            Box::new(FlatPrinter)
        } else {
            Box::new(SuperResolutionPrinter)
        }
    }
}
