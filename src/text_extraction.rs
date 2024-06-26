use crate::image_camera;

use leptess::{leptonica, tesseract};
use opencv::{core::Vector, imgcodecs, imgproc, prelude::*};

lazy_static::lazy_static! {
    pub(crate) static ref TESSERACT_API: std::sync::Mutex<tesseract::TessApi> = std::sync::Mutex::new(tesseract::TessApi::new(None, "eng").unwrap());
}

pub(crate) fn extract_text_from_mat(frame: &Mat) -> Result<String, Box<dyn std::error::Error>> {
    let card_image = {
        let card = image_camera::CARD.lock().unwrap();
        if let Ok(card_image) = (*card).get_unwarped(frame) {
            Some(card_image.clone())
        } else {
            None
        }
    }
    .unwrap();

    // TODO : Normalize?
    // // Convert the image to grayscale
    // let mut gray = Mat::default();
    // imgproc::cvt_color(mat, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    // // Apply Gaussian Blur to reduce noise
    // let mut smoothed = Mat::default();
    // imgproc::gaussian_blur(
    //     &gray,
    //     &mut smoothed,
    //     core::Size::new(3, 3),
    //     0.0,
    //     0.0,
    //     core::BORDER_DEFAULT,
    // )?;

    // // Binarize the image using Otsu's method
    // let mut binary = Mat::default();
    // imgproc::threshold(
    //     &smoothed,
    //     &mut binary,
    //     0.0,
    //     255.0,
    //     imgproc::THRESH_BINARY | imgproc::THRESH_OTSU,
    // )?;

    let mut gray = Mat::default();
    imgproc::cvt_color(&card_image, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    // Convert Mat to a format that Leptess can use
    let mut buf = Vector::new();
    imgcodecs::imencode(".png", &gray, &mut buf, &Vector::new()).unwrap();
    let pix = leptonica::pix_read_mem(buf.as_ref())?;

    // Recognize text
    let mut api = TESSERACT_API.lock()?;
    api.set_image(&pix);
    Ok(api.get_utf8_text()?)
}
