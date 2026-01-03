use image::{
    DynamicImage, GenericImageView,
    imageops::{FilterType, grayscale},
};
const ASCII_CHARS: &[u8] = b"@%#*+=-:. ";
const SOLID_CHARS: &[&str] = &["█", "▓", "▒", "░", " "];

pub enum ConvertType {
    Ascii,
    Solid,
}
pub struct AsciilatorConfig {
    pub width: u32,
    pub scale_factor: f64,
    pub convert_type: ConvertType,
    //pub inverted: bool,
}

impl Default for AsciilatorConfig {
    fn default() -> Self {
        Self {
            width: 100,
            scale_factor: 0.55,
            convert_type: ConvertType::Ascii,
        }
    }
}

pub fn convert_to_ascii(img: &DynamicImage, config: &AsciilatorConfig) -> String {
    let (orig_width, orig_height) = img.dimensions();
    // Get aspect ratio
    let orig_ratio = orig_height as f64 / orig_width as f64;
    // Calculate new height with font proportions
    let new_height = (config.width as f64 * orig_ratio * config.scale_factor) as u32;
    let img = img.unsharpen(3.0, 1); // unsharpen mask повышает четкость границ
    // RESIZING
    let resized = img.resize_exact(config.width, new_height, FilterType::Nearest);

    // Convert to grayscale
    let grayscale = resized.to_luma8();
    // Get actual width for \n
    let (actual_width, _) = grayscale.dimensions();
    let mut ascii_art = String::with_capacity((config.width * new_height + new_height) as usize);

    for (x, _, pixel) in grayscale.enumerate_pixels() {
        let brightness = pixel[0];
        let char_index = brightness as usize * (ASCII_CHARS.len() - 1) / 255;

        match config.convert_type {
            ConvertType::Ascii => {
                let idx = (brightness as usize * (ASCII_CHARS.len() - 1)) / 255;
                ascii_art.push(ASCII_CHARS[idx] as char);
            }
            ConvertType::Solid => {
                let idx = (brightness as usize * (SOLID_CHARS.len() - 1)) / 255;
                ascii_art.push_str(SOLID_CHARS[idx]);
            }
        }

        if x == actual_width - 1 {
            ascii_art.push('\n');
        }
    }
    ascii_art
}
