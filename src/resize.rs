use anyhow::Result;
use crossterm::terminal;

#[cfg(feature = "opencv")]
use opencv::{core, imgproc, prelude::*};

#[cfg(not(feature = "opencv"))]
use image::{imageops::FilterType, DynamicImage, GenericImageView};

const DEFAULT_TERMINAL_SIZE: (i32, i32) = (80, 24);

pub struct ResizerFactory;

impl ResizerFactory {
    pub fn create(protrude: bool, flat: bool) -> Box<dyn Resizer> {
        if flat {
            Box::new(FlatResizer { protrude })
        } else {
            Box::new(SuperResolutionResizer { protrude })
        }
    }
}

pub trait Resizer {
    fn calc_resized_img_size(&self, img_w: i32, img_h: i32, term_w: i32, term_h: i32)
        -> (i32, i32);

    fn get_terminal_size(&self) -> Option<(i32, i32)> {
        match terminal::size() {
            Ok((col, row)) => Some((col as i32, row as i32)),
            Err(_) => None,
        }
    }

    #[cfg(feature = "opencv")]
    fn resize_img(&self, img: &core::Mat) -> Result<core::Mat> {
        let size = img.size()?;
        let (term_w, term_h) = self.get_terminal_size().unwrap_or(DEFAULT_TERMINAL_SIZE);
        let (w, h) = self.calc_resized_img_size(size.width, size.height, term_w, term_h);

        let mut resized = core::Mat::default();
        imgproc::resize(
            img,
            &mut resized,
            core::Size::new(w, h),
            0f64,
            0f64,
            imgproc::INTER_NEAREST,
        )?;
        Ok(resized)
    }

    #[cfg(not(feature = "opencv"))]
    fn resize_img(&self, img: &DynamicImage) -> Result<DynamicImage> {
        let size = img.dimensions();
        let (term_w, term_h) = self.get_terminal_size().unwrap_or(DEFAULT_TERMINAL_SIZE);
        let (w, h) =
            self.calc_resized_img_size(size.0.try_into()?, size.1.try_into()?, term_w, term_h);
        let resized = img.resize_exact(w as u32, h as u32, FilterType::Nearest);
        Ok(resized)
    }
}

pub struct FlatResizer {
    pub protrude: bool,
}

impl Resizer for FlatResizer {
    fn calc_resized_img_size(
        &self,
        img_w: i32,
        img_h: i32,
        term_w: i32,
        term_h: i32,
    ) -> (i32, i32) {
        let term_h = term_h - 2;
        let mut h = img_h;
        let mut w = img_w;

        if !self.protrude {
            h = term_h;
            w = img_w * term_h / img_h;
        }
        if (term_w / 2) < w {
            h = h * term_w / 2 / w;
            w = term_w / 2;
        }
        (w, h)
    }
}

pub struct SuperResolutionResizer {
    pub protrude: bool,
}

impl Resizer for SuperResolutionResizer {
    fn calc_resized_img_size(
        &self,
        img_w: i32,
        img_h: i32,
        term_w: i32,
        term_h: i32,
    ) -> (i32, i32) {
        let term_h = term_h * 2 - 2;
        let mut h = img_h;
        let mut w = img_w;

        if !self.protrude {
            h = term_h;
            w = img_w * term_h / img_h;
        }
        if term_w < w {
            h = h * term_w / w;
            w = term_w;
        }
        (w, h)
    }
}
