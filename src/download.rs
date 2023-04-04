use crate::png::Png;
use crate::{PngError, Result};

pub fn download_png_from_url(url: &str) -> Result<Png> {
    let resp = reqwest::blocking::get(url).unwrap();

    if resp.status().is_success() {
        let bytes = resp.bytes().unwrap();
        println!("Downloaded {} bytes", bytes.len());
        Ok(Png::try_from(bytes.as_ref()).unwrap())
    } else {
        Err(PngError::DownloadFailed)
    }
}
